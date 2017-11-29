extern crate lan_clipboard;
extern crate protobuf;
extern crate clipboard;
extern crate rustls;
extern crate chrono;
extern crate mio;
extern crate crypto;
extern crate parking_lot;

use lan_clipboard::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use rustls::{ClientConfig, ClientSession, Session};
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
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.len() < 4 {
    println!("usage: lan_clipboard_client [hostname] [port] [cert file] [client name]");
    return;
  }
  let hostname = &args[0];
  let port: u16 = match args[1].parse() {
    Ok(p) => p,
    Err(e) => {
      println!("Invalid port: {}", e);
      return;
    }
  };
  let addr: SocketAddr = match format!("{}:{}", hostname, port).to_socket_addrs() {
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
  let cert = &args[2];
  let name = args[3..].join(" ");

  let f = match File::open(cert) {
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
          client.do_tls_read();
        }

        match client.read_to_buf() {
          Ok(0) if !client.session.wants_write() => break 'outer,
          Err(_) => break 'outer,
          _ => {}
        }
        let (res, pos) = {
          let mut cursor = std::io::Cursor::new(&mut client.buf);
          (cursor.read_message(), cursor.position())
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
          for t in tx {
            let _ = client.session.write_message(&t); // FIXME: don't ignore errors
          }
        }
        client.reregister(&mut poll.lock());
      }
    }
  }
  println!("An error occurred when communicating with the server. Shutting donw.");
  client.lock().hangup(&mut poll.lock());
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

  fn do_tls_read(&mut self) {
    if self.session.read_tls(&mut self.tcp).is_ok() {
      if let Err(e) = self.session.process_new_packets() {
        panic!("process err (post-read): {}", e);
      }
    }
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
