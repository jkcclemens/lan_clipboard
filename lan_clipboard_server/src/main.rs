extern crate lan_clipboard;
extern crate protobuf;
extern crate integer_encoding;
extern crate rustls;
extern crate mio;
extern crate slab;
extern crate untrusted;
extern crate webpki;
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

use rustls::ServerConfig;
use mio::*;
use mio::net::TcpListener;
use slab::Slab;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::io;
use std::sync::Arc;

mod config;
mod cli;
mod daemon;
mod server;

use config::Config;
use daemon::Daemon;
use server::{Server, SERVER};

fn main() {
  let code = match inner() {
    Ok(_) => 0,
    Err(e) => {
      println!("{}", e);
      1
    }
  };
  std::process::exit(code);
}

fn inner() -> Result<(), String> {
  let app_config: Config = cli::load_config(&cli::process_args())?;

  let bind_addr: SocketAddr = format!("{}:{}", app_config.connection.hostname.as_ref().unwrap(), app_config.connection.port.unwrap()).to_socket_addrs()
    .map_err(|e| format!("invalid hostname:port: {}", e))?
    .next()
    .ok_or_else(|| "no addresses provided".to_string())?;

  let f = File::open(app_config.certificate.file.as_ref().unwrap())
    .map_err(|e| format!("could not open cert: {}", e))?;
  let certs = rustls::internal::pemfile::certs(&mut io::BufReader::new(f))
    .map_err(|e| format!("invalid certificate chain: {:?}", e))?;
  if certs.is_empty() {
    return Err("no certs found".into());
  }
  webpki::EndEntityCert::from(untrusted::Input::from(&certs[0].0))
    .map_err(|e| format!("invalid cert: {:?}", e))?;

  let f = File::open(app_config.certificate.key.as_ref().unwrap())
    .map_err(|e| format!("could not open key: {}", e))?;
  let mut keys = rustls::internal::pemfile::rsa_private_keys(&mut io::BufReader::new(f))
    .map_err(|e| format!("invalid key: {:?}", e))?;
  if keys.is_empty() {
    return Err("no keys found".into());
  }
  let key = keys.remove(0);

  if let Some(d) = Daemon::new(&app_config)? {
    d.start().map_err(|e| format!("could not start as daemon: {}", e))?;
  }

  let mut conn_poll = Poll::new().map_err(|e| format!("could not create poll: {}", e))?;

  let listener = TcpListener::bind(&bind_addr).map_err(|e| format!("could not bind: {}", e))?;
  let mut config = ServerConfig::new();
  config.set_single_cert(certs, key);
  let config = Arc::new(config);

  conn_poll.register(&listener, SERVER, Ready::readable(), PollOpt::edge())
    .map_err(|e| format!("could not register with poll: {}", e))?;

  let mut server = Server {
    listener,
    config,
    nodes: Slab::with_capacity(4),
    state: Default::default()
  };

  let mut conn_events = Events::with_capacity(128);

  loop {
    conn_poll.poll(&mut conn_events, Some(std::time::Duration::from_millis(100)))
      .map_err(|e| format!("could not poll: {}", e))?;
    for event in conn_events.iter() {
      let token = event.token();
      if event.readiness().is_writable() {
        let res = match event.token() {
          SERVER => return Err("server was writable".into()),
          i => server.node_writable(&mut conn_poll, i)
        };
        if let Err(e) = res {
          println!("error writing to node {}: {}\nshutting down that node", token.0, e);
          server.nodes[token.0].shutting_down = true;
          server.hangup(&mut conn_poll, token);
          if let Err(e) = server.remove_node(&mut conn_poll, token) {
            println!("error shutting down node {}: {}", token.0, e);
          }
        }
      }

      if event.readiness().is_readable() {
        let res = match event.token() {
          SERVER => server.accept(&mut conn_poll),
          i => server.node_readable(&mut conn_poll, i)
        };
        match res {
          Err(ref e) if token == SERVER => {
            return Err(format!("error accepting node {}: {}\nshutting down server", token.0, e));
          },
          Err(e) => {
            println!("error reading from node {}: {}\nshutting down that node", token.0, e);
            server.nodes[token.0].shutting_down = true;
            server.hangup(&mut conn_poll, token);
            if let Err(e) = server.remove_node(&mut conn_poll, token) {
              println!("error shutting down node {}: {}", token.0, e);
            }
          },
          _ => {}
        }
      }
    }
  }
}
