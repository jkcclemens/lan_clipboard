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

use lan_clipboard::*;
use rustls::{ClientConfig, ClientSession, Session};
use parking_lot::Mutex;
use mio::*;
use mio::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::sync::Arc;
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

  let f = match File::open(app_config.certificate.file.as_ref().unwrap()) {
    Ok(f) => f,
    Err(e) => {
      error!("could not open cert file: {}", e);
      return;
    }
  };

  let mut config = ClientConfig::new();
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
  let config = Arc::new(config);
  let session = ClientSession::new(&config, hostname);

  if let Err(e) = daemon::handle_daemon(&app_config) {
    error!("could not daemonize: {}", e);
    return;
  }

  let poll = match Poll::new() {
    Ok(p) => p,
    Err(e) => {
      error!("could not create poll: {}", e);
      return;
    }
  };

  let connection = match TcpStream::connect(&addr) {
    Ok(t) => t,
    Err(e) => {
      error!("could not create tcp stream: {}", e);
      return;
    }
  };
  if let Err(e) = poll.register(&connection, client::CLIENT, Ready::readable() | Ready::writable(), PollOpt::edge()) {
    error!("could not register on poll: {}", e);
    return;
  }

  let poll = Arc::new(Mutex::new(poll));

  let client = match Client::new(connection, session) {
    Ok(c) => c,
    Err(e) => {
      error!("could not create client: {}", e);
      return;
    }
  };
  let client = Arc::new(Mutex::new(client));
  Client::send_thread(Arc::clone(&client), Arc::clone(&poll));

  let mut events = Events::with_capacity(1024);

  'outer: loop {
    if let Err(e) = poll.lock().poll(&mut events, Some(Duration::from_millis(100))) {
      error!("could not poll: {}", e);
      return;
    }
    for event in events.iter() {
      let mut client = client.lock();

      if !client.state.registered && !client.state.hello_sent {
        let mut hello: Hello = Hello::new();
        hello.set_version(1);
        hello.set_name(name.clone());
        if let Err(e) = client.queue_message(hello.into(), &mut poll.lock()) {
          warn!("could not queue message: {}", e);
        }
        client.state.hello_sent = true;
      }

      if event.readiness().is_readable() {
        if client.session.wants_read() {
          match client.do_tls_read() {
            Ok(0) if !client.session.wants_write() => break 'outer,
            Err(e) => {
              #[cfg(windows)]
              {
                if let Some(10035) = e.raw_os_error() {
                  if let Err(e) = client.reregister(&mut poll.lock()) {
                    error!("could not reregister: {}", e);
                    return;
                  }
                  debug!("ignoring async error on windows");
                  continue;
                }
              }
              error!("{}", e);
              break 'outer;
            },
            _ => {}
          }
        }

        if client.read_to_buf().is_err() {
          break 'outer;
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
          return;
        }
      }

      if event.readiness().is_writable() {
        if client.session.wants_write() {
          client.do_tls_write();
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
          return;
        }
      }
    }
  }
  error!("an error occurred when communicating with the server. shutting down");
  client.lock().hangup(&mut poll.lock());
}
