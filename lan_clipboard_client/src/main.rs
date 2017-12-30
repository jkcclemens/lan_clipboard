extern crate lan_clipboard;
extern crate protobuf;
extern crate clipboard;
extern crate rustls;
extern crate chrono;
extern crate mio;
extern crate crypto;
extern crate parking_lot;
extern crate libc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate clap;
#[cfg(not(windows))]
extern crate daemonize;
extern crate snap;
extern crate rand;
#[macro_use]
extern crate log;
extern crate fern;
#[cfg(feature = "trusted_cas")]
extern crate webpki_roots;

use lan_clipboard::*;
use rustls::{ClientConfig, ClientSession, Session};
use parking_lot::Mutex;
use mio::*;
use mio::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Cursor, BufWriter};

mod config;
mod cli;
mod client;
mod daemon;
mod state;
mod logging;

use config::Config;
use client::Client;

fn main() {
  let app_config: Config = match cli::load_config(&cli::process_args()) {
    Ok(c) => c,
    Err(e) => {
      println!("{}", e);
      return;
    }
  };

  if let Err(e) = logging::set_up_logger(&app_config) {
    println!("could not set up logging: {}", e.into_inner());
    return;
  }

  let hostname = app_config.connection.hostname.as_ref().unwrap();
  let name = app_config.connection.name.as_ref().unwrap();

  let addr: SocketAddr = match format!("{}:{}", hostname, app_config.connection.port.unwrap()).to_socket_addrs() {
    Ok(mut s) => match s.next() {
      Some(s) => s,
      None => {
        error!("no addresses provided for connection");
        return;
      }
    },
    Err(e) => {
      error!("invalid address: {}", e);
      return;
    }
  };

  let mut config = ClientConfig::new();

  if let Some(ref cert_file) = app_config.certificate.file.as_ref() {
    let f = match File::open(cert_file) {
      Ok(f) => f,
      Err(e) => {
        error!("could not open cert file: {}", e);
        return;
      }
    };
    let (added, _) = match config.root_store.add_pem_file(&mut io::BufReader::new(f)) {
      Ok(a) => a,
      Err(e) => {
        error!("could not add certificate chain: {:?}", e);
        return;
      }
    };
    if added == 0 {
      error!("no certs found");
      return;
    }
  }

  #[cfg(feature = "trusted_cas")]
  config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

  let config = Arc::new(config);

  if let Err(e) = daemon::handle_daemon(&app_config) {
    error!("could not daemonize: {}", e);
    return;
  }

  let reconnect = app_config.connection.reconnect.unwrap_or(false);
  let reconnect_period = app_config.connection.reconnect_period.unwrap_or(15);

  while main_loop(&config, hostname, &addr, name) && reconnect {
    info!("waiting {} second{} before attempting to reconnect", reconnect_period, if reconnect_period == 1 { "" } else { "s" });
    thread::sleep(Duration::from_secs(reconnect_period));
    info!("attempting to reconnect");
  }
}

// returns whether to attempt a reconnection, if enabled
fn main_loop(config: &Arc<ClientConfig>, hostname: &str, addr: &SocketAddr, name: &str) -> bool {
  let session = ClientSession::new(&config, hostname);

  let poll = match Poll::new() {
    Ok(p) => p,
    Err(e) => {
      error!("could not create poll: {}", e);
      return false;
    }
  };

  let connection = match TcpStream::connect(&addr) {
    Ok(t) => t,
    Err(e) => {
      error!("could not create tcp stream: {}", e);
      return true;
    }
  };
  if let Err(e) = connection.set_keepalive(Some(Duration::from_secs(30))) {
    error!("could not set keepalive on socket: {}", e);
    return false;
  }
  if let Err(e) = poll.register(&connection, client::CLIENT, Ready::readable() | Ready::writable(), PollOpt::edge()) {
    error!("could not register on poll: {}", e);
    return false;
  }

  let poll = Arc::new(Mutex::new(poll));

  let client = Arc::new(Mutex::new(Client::new(connection, session)));
  Client::send_thread(Arc::clone(&client), Arc::clone(&poll));

  let mut events = Events::with_capacity(1024);

  let try_reconnect = 'outer: loop {
    if let Err(e) = poll.lock().poll(&mut events, Some(Duration::from_millis(100))) {
      error!("could not poll: {}", e);
      break false;
    }
    for event in events.iter() {
      let mut client = client.lock();

      if !client.state.registered && !client.state.hello_sent {
        let mut hello: Hello = Hello::new();
        hello.set_version(1);
        hello.set_name(name.to_string());
        if let Err(e) = client.queue_message(hello.into(), &mut poll.lock()) {
          warn!("could not queue message: {}", e);
        }
        client.state.hello_sent = true;
      }

      if event.readiness().is_readable() {
        if client.session.wants_read() {
          match client.do_tls_read() {
            Ok(0) if !client.session.wants_write() => break 'outer true,
            Err(e) => {
              #[cfg(windows)]
              {
                if let Some(10035) = e.raw_os_error() {
                  if let Err(e) = client.reregister(&mut poll.lock()) {
                    error!("could not reregister: {}", e);
                    break 'outer false;
                  }
                  debug!("ignoring async error on windows");
                  continue;
                }
              }
              error!("{}", e);
              break 'outer true;
            },
            _ => {}
          }
        }

        if let Err(e) = client.read_to_buf() {
          error!("{}", e);
          break 'outer true;
        }
        let (res, pos) = {
          let mut cursor = Cursor::new(&client.buf);
          (cursor.read_message(), cursor.position())
        };
        if res.is_ok() {
          client.buf = client.buf.split_off(pos as usize);
        }
        if let Ok(message) = res {
          client.receive(message, &mut poll.lock());
        }
        if let Err(e) = client.reregister(&mut poll.lock()) {
          error!("could not reregister: {}", e);
          break 'outer false;
        }
      }

      if event.readiness().is_writable() {
        if client.session.wants_write() {
          if let Err(e) = client.do_tls_write() {
            error!("could not send to server: {}", e);
            break 'outer true;
          }
        }

        if !client.tx.is_empty() {
          let mut tx = Vec::new();
          std::mem::swap(&mut client.tx, &mut tx);
          let mut writer = BufWriter::new(&mut client.session);
          for t in tx {
            if let Err(e) = writer.write_message(&t) {
              error!("could not write message: {}", e);
            }
          }
        }
        if let Err(e) = client.reregister(&mut poll.lock()) {
          error!("could not reregister: {}", e);
          break 'outer false;
        }
      }
    }
  };
  error!("an error occurred when communicating with the server");
  client.lock().hangup(&mut poll.lock());
  if let Some(handle) = client.lock().send_handle.take() {
    handle.join().expect("could not join on send thread");
  }
  let hur = client.lock().hang_up_reason;
  match hur {
    Some(HangingUp_HangUpReason::PING_TIMEOUT) => true,
    Some(_) => false,
    None => try_reconnect
  }
}
