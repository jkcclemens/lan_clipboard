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

use lan_clipboard::*;
use rustls::{ClientConfig, ClientSession, Session};
use parking_lot::Mutex;
use mio::*;
use mio::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::sync::Arc;
use std::io::{self, BufWriter};

mod config;
mod cli;
mod client;
mod daemon;
mod state;

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

  let hostname = app_config.connection.hostname.as_ref().unwrap();
  let name = app_config.connection.name.as_ref().unwrap();

  let addr: SocketAddr = match format!("{}:{}", hostname, app_config.connection.port.unwrap()).to_socket_addrs() {
    Ok(mut s) => match s.next() {
      Some(s) => s,
      None => {
        println!("No addresses provided.");
        return;
      }
    },
    Err(e) => {
      println!("Invalid address: {}", e);
      return;
    }
  };

  let f = match File::open(app_config.certificate.file.as_ref().unwrap()) {
    Ok(f) => f,
    Err(e) => {
      println!("could not open cert file: {}", e);
      return;
    }
  };

  let mut config = ClientConfig::new();
  let (added, _) = match config.root_store.add_pem_file(&mut io::BufReader::new(f)) {
    Ok(a) => a,
    Err(e) => {
      println!("could not add certificate chain: {:?}", e);
      return;
    }
  };
  if added == 0 {
    println!("No certs found.");
    return;
  }
  let config = Arc::new(config);
  let session = ClientSession::new(&config, hostname);

  if let Err(e) = daemon::handle_daemon(&app_config) {
    println!("could not daemonize: {}", e);
    return;
  }

  let poll = match Poll::new() {
    Ok(p) => p,
    Err(e) => {
      println!("could not create poll: {}", e);
      return;
    }
  };

  let connection = match TcpStream::connect(&addr) {
    Ok(t) => t,
    Err(e) => {
      println!("could not create tcp stream: {}", e);
      return;
    }
  };
  if let Err(e) = poll.register(&connection, client::CLIENT, Ready::readable() | Ready::writable(), PollOpt::edge()) {
    println!("could not register on poll: {}", e);
    return;
  }

  let poll = Arc::new(Mutex::new(poll));

  let client = match Client::new(connection, session) {
    Ok(c) => c,
    Err(e) => {
      println!("could not create client: {}", e);
      return;
    }
  };
  let client = Arc::new(Mutex::new(client));
  Client::send_thread(Arc::clone(&client), Arc::clone(&poll));

  let mut events = Events::with_capacity(1024);

  'outer: loop {
    if let Err(e) = poll.lock().poll(&mut events, Some(std::time::Duration::from_millis(100))) {
      println!("could not poll: {}", e);
      return;
    }
    for event in events.iter() {
      let mut client = client.lock();

      if !client.state.registered && !client.state.hello_sent {
        let mut hello: Hello = Hello::new();
        hello.set_version(1);
        hello.set_name(name.clone());
        if let Err(e) = client.queue_message(hello.into(), &mut poll.lock()) {
          println!("could not queue message: {}", e);
        }
        client.state.hello_sent = true;
      }

      if event.readiness().is_readable() {
        if client.session.wants_read() {
          match client.do_tls_read() {
            Ok(0) if !client.session.wants_write() => break 'outer,
            Err(e) => {
              println!("{}", e);
              break 'outer;
            },
            _ => {}
          }
        }

        if client.read_to_buf().is_err() {
          break 'outer;
        }
        let (res, pos) = {
          let mut cursor = std::io::Cursor::new(&client.buf);
          (cursor.read_message(), cursor.position())
        };
        if res.is_ok() {
          client.buf = client.buf.split_off(pos as usize);
        }
        if let Ok(message) = res {
          client.receive(message, &mut poll.lock());
        }
        if let Err(e) = client.reregister(&mut poll.lock()) {
          println!("could not reregister: {}", e);
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
            let _ = writer.write_message(&t); // FIXME: don't ignore errors
          }
        }
        if let Err(e) = client.reregister(&mut poll.lock()) {
          println!("could not reregister: {}", e);
          return;
        }
      }
    }
  }
  println!("An error occurred when communicating with the server. Shutting down.");
  client.lock().hangup(&mut poll.lock());
}
