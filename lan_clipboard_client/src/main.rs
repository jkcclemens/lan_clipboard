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

#[cfg(not(windows))]
use daemonize::Daemonize;

use lan_clipboard::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use rustls::{ClientConfig, ClientSession, Session};
use snap::{Reader as SnappyReader, Writer as SnappyWriter};
use chrono::{Utc, DateTime};
use parking_lot::Mutex;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use mio::*;
use mio::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::collections::HashMap;
use std::sync::Arc;
use std::fs::File;
use std::io::{self, Read};

mod config;
mod cli;

use config::Config;

const CLIENT: Token = Token(0);

#[derive(Default)]
struct State {
  registered: bool,
  hello_sent: bool,
  tree: HashMap<u32, String>,
  shared: Vec<u8>
}

impl State {
  fn update_clipboard(&self) {
    if let Ok(s) = String::from_utf8(self.shared.clone()) {
      if let Ok(mut c) = ClipboardContext::new() {
        c.set_contents(s).ok();
      }
    }
  }
}

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
  let (added, _) = config.root_store.add_pem_file(&mut io::BufReader::new(f)).unwrap();
  if added == 0 {
    println!("No certs found.");
    return;
  }
  let config = Arc::new(config);
  let session = ClientSession::new(&config, hostname);

  handle_daemon(&app_config);

  let poll = Poll::new().unwrap();

  let connection = TcpStream::connect(&addr).expect("could not create tcp stream");
  poll.register(&connection, CLIENT, Ready::readable() | Ready::writable(), PollOpt::edge()).expect("could not register poll");

  let poll = Arc::new(Mutex::new(poll));

  let client = Client::new(connection, session);
  let client = Arc::new(Mutex::new(client));
  Client::send_thread(client.clone(), poll.clone());

  let mut events = Events::with_capacity(1024);

  'outer: loop {
    poll.lock().poll(&mut events, Some(std::time::Duration::from_millis(100))).expect("could not poll");
    for event in events.iter() {
      let mut client = client.lock();

      if !client.state.registered && !client.state.hello_sent {
        let mut hello: Hello = Hello::new();
        hello.set_version(1);
        hello.set_name(name.clone());
        client.queue_message(hello.into(), &mut poll.lock());
        client.state.hello_sent = true;
      }

      if event.readiness().is_readable() {
        if client.session.wants_read() {
          match client.do_tls_read() {
            Ok(0) if !client.session.wants_write() => break 'outer,
            Err(_) => break 'outer,
            _ => {}
          }
        }

        if let Err(_) = client.read_to_buf() {
          break 'outer;
        }
        let (res, pos) = {
          let mut cursor = std::io::Cursor::new(&client.buf);
          (SnappyReader::new(cursor.by_ref()).read_message(), cursor.position())
        };
        if res.is_ok() {
          client.buf = client.buf.split_off(pos as usize);
        }
        if let Ok(message) = res {
          client.receive(message, &mut poll.lock());
        }
        client.reregister(&mut poll.lock());
      }

      if event.readiness().is_writable() {
        if client.session.wants_write() {
          client.do_tls_write();
        }

        if !client.tx.is_empty() {
          let mut tx = Vec::new();
          std::mem::swap(&mut client.tx, &mut tx);
          let mut writer = SnappyWriter::new(client.session.by_ref());
          for t in tx {
            let _ = writer.write_message(&t); // FIXME: don't ignore errors
          }
        }
        client.reregister(&mut poll.lock());
      }
    }
  }
  println!("An error occurred when communicating with the server. Shutting down.");
  client.lock().hangup(&mut poll.lock());
}

#[cfg(not(windows))]
fn handle_daemon(config: &Config) {
  if config.daemon.enabled != Some(true) {
    return;
  }
  let mut d = Daemonize::new();
  if let Some(ref pid_file) = config.daemon.pid_file {
    d = d.pid_file(pid_file);
  }
  if let Some(chown_pid_file) = config.daemon.chown_pid_file {
    d = d.chown_pid_file(chown_pid_file);
  }
  if let Some(user) = config.daemon.user {
    d = d.user(user);
  }
  if let Some(group) = config.daemon.group {
    d = d.group(group);
  }
  d
    .working_directory("./")
    .start()
    .unwrap();
}

#[cfg(windows)]
fn handle_daemon(config: &Config) {
  if config.daemon.enabled == Some(true) {
    println!("daemonization is not supported on Windows");
  }
}

struct Client {
  tcp: TcpStream,
  session: ClientSession,
  state: State,
  tx: Vec<Message>,
  buf: Vec<u8>,
  last_ping: (u32, DateTime<Utc>),
  last_update: Vec<u8>,
  clipboard: ClipboardContext
}

impl Client {
  fn new(tcp: TcpStream, session: ClientSession) -> Client {
    Client {
      tcp,
      session,
      state: State::default(),
      tx: Vec::new(),
      buf: Vec::new(),
      last_ping: (0, Utc::now()),
      last_update: Vec::new(),
      clipboard: ClipboardContext::new().unwrap()
    }
  }

  fn read_to_buf(&mut self) -> io::Result<usize> {
    self.session.read_to_end(&mut self.buf)
  }

  fn hangup(&mut self, poll: &mut Poll) {
    if let Err(e) = poll.deregister(&self.tcp) {
      println!("error deregistering: {}", e);
    }
    self.tcp.shutdown(std::net::Shutdown::Both).ok();
  }

  /// What IO events we're currently waiting for,
  /// based on wants_read/wants_write.
  fn event_set(&self) -> mio::Ready {
    let rd = self.session.wants_read();
    let wr = self.session.wants_write();

    let mut ready = if rd && wr {
      mio::Ready::readable() | mio::Ready::writable()
    } else if wr {
      mio::Ready::writable()
    } else {
      mio::Ready::readable()
    };

    if !self.tx.is_empty() {
      ready.insert(Ready::writable());
    }
    ready
  }

  fn reregister(&self, poll: &mut Poll) {
    poll.reregister(&self.tcp, CLIENT, self.event_set(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
  }

  fn do_tls_write(&mut self) {
    if let Err(e) = self.session.write_tls(&mut self.tcp) {
      panic!("write err: {}", e);
    }
    if let Err(e) = self.session.process_new_packets() {
      panic!("process err (post-write): {}", e);
    }
  }

  fn do_tls_read(&mut self) -> io::Result<usize> {
    let read = self.session.read_tls(&mut self.tcp)?;
    self.session.process_new_packets().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(read)
  }

  fn queue_message(&mut self, msg: Message, poll: &mut Poll) {
    self.tx.push(msg);
    self.reregister(poll);
  }

  #[allow(unused)]
  fn queue_messages<I>(&mut self, msgs: I, poll: &mut Poll)
    where I: IntoIterator<Item=Message>
  {
    self.tx.extend(msgs);
    self.reregister(poll);
  }

  fn send_thread(client: Arc<Mutex<Client>>, poll: Arc<Mutex<Poll>>) {
    std::thread::spawn(move || {
      let mut sha3 = Sha3::sha3_512();
      loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        if !client.lock().state.registered {
          continue;
        }
        let now = Utc::now();
        let mut client = client.lock();
        if now.signed_duration_since(client.last_ping.1).num_seconds().abs() > 15 {
          let mut poll = poll.lock();
          client.last_ping.1 = now;
          client.last_ping.0 = client.last_ping.0 + 1;
          let mut ping = Ping::new();
          ping.set_seq(client.last_ping.0);
          println!("ping {}", client.last_ping.0);
          client.queue_message(ping.into(), &mut poll);
        }
        let shared = client.state.shared.clone();
        let local = match client.clipboard.get_contents() {
          Ok(c) => c.into_bytes(),
          Err(_) => continue
        };
        let mut local_hash = Vec::with_capacity(64);
        sha3.input(&local);
        sha3.result(&mut local_hash);
        sha3.reset();
        if local == shared || client.last_update == local {
          continue;
        }
        let mut cu = ClipboardUpdate::new();
        cu.set_contents(local.clone());
        client.queue_message(cu.into(), &mut poll.lock());
        client.last_update = local_hash;
      }
    });
  }

  fn receive(&mut self, mut message: Message, _poll: &mut Poll) {
    match message.get_field_type() {
      Message_MessageType::CLIPBOARD_UPDATE => {
        if !message.has_clipboard_update() {
          return;
        }
        let mut cu = message.take_clipboard_update();
        let new = cu.take_contents();
        self.state.shared = new;
        self.state.update_clipboard();
      },
      Message_MessageType::REGISTERED => {
        if !message.has_registered() {
          return;
        }
        let mut registered: Registered = message.take_registered();
        self.state.registered = true;
        self.state.tree = registered.take_tree().take_nodes();
        self.state.shared = registered.get_clipboard().to_vec();
        self.state.update_clipboard();
        println!("Registered as node {} with node tree:\n{:#?}", registered.get_node_id(), self.state.tree);
      },
      Message_MessageType::REJECTED => {
        if !message.has_rejected() {
          return;
        }
        let rejected: Rejected = message.take_rejected();
        println!("Rejected: {:?}", rejected.get_reason());
        return; // FIXME: exit
      },
      Message_MessageType::NODE_UPDATE => {
        if !message.has_node_update() {
          return;
        }
        let node_update: NodeUpdate = message.take_node_update();
        match node_update.get_field_type() {
          NodeUpdate_UpdateType::ADDED => {
            self.state.tree.insert(node_update.get_node_id(), node_update.get_node_name().into());
          },
          NodeUpdate_UpdateType::REMOVED => {
            self.state.tree.remove(&node_update.get_node_id());
          }
        }
        println!("Node {} was {:?}", node_update.get_node_id(), node_update.get_field_type());
        println!("new tree: {:#?}", self.state.tree);
      },
      Message_MessageType::PONG => {
        if !message.has_pong() {
          return;
        }

        let pong = message.take_pong();
        println!("pong {}", pong.get_seq());
      }
      _ => println!("received unsupported message")
    }
  }
}
