extern crate lan_clipboard;
extern crate protobuf;
extern crate clipboard;
extern crate native_tls;
extern crate chrono;
extern crate mio;

use lan_clipboard::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use native_tls::{TlsConnector, TlsStream, Certificate, MidHandshakeTlsStream};
use chrono::{Utc, DateTime};
use mio::*;
use mio::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
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
    println!("usage: client [hostname] [port] [cert file] [client name]");
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

  let mut f = match File::open(cert) {
    Ok(f) => f,
    Err(e) => {
      println!("could not open cert file: {}", e);
      return;
    }
  };

  let mut data = Vec::new();
  if let Err(e) = f.read_to_end(&mut data) {
    println!("could not read cert file: {}", e);
    return;
  }

  let cert = match Certificate::from_der(&data) {
    Ok(c) => c,
    Err(e) => {
      println!("could not parse cert file: {}", e);
      return;
    }
  };

  let mut builder = TlsConnector::builder().unwrap();
  if let Err(e) = builder.add_root_certificate(cert) {
    println!("could not add cert file: {}", e);
    return;
  }

  let mut poll = Poll::new().unwrap();

  let connector = builder.build().unwrap();
  let connection = TcpStream::connect(&addr).unwrap();
  poll.register(&connection, CLIENT, Ready::readable() | Ready::writable(), PollOpt::edge()).unwrap();

  let poll = Arc::new(Mutex::new(poll));

  let mut client = Client::new(connection);
  client.try_start_tls(&connector, hostname).unwrap();
  let client = Arc::new(RwLock::new(client));
  Client::send_thread(client.clone(), poll.clone());

  let mut events = Events::with_capacity(1024);

  loop {
    poll.lock().unwrap().poll(&mut events, Some(std::time::Duration::from_millis(100))).unwrap();
    for event in events.iter() {
      let mut client = client.write().unwrap();
      if !client.try_resume_tls().unwrap() {
        continue;
      }

      if !client.state.registered && !client.state.hello_sent {
        let mut hello: Hello = Hello::new();
        hello.set_version(1);
        hello.set_name(name.clone());
        client.queue_message(hello.into(), &mut poll.lock().unwrap());
        client.state.hello_sent = true;
      }

      println!("tx: {:?}", client.tx);

      if event.readiness().is_writable() {
        if client.tx.is_empty() {
          client.reregister(&mut poll.lock().unwrap(), Ready::readable(), PollOpt::edge());
          continue;
        }
        {
          let mut tx = Vec::new();
          std::mem::swap(&mut client.tx, &mut tx);
          let stream = client.tls.as_mut().unwrap();
          for t in tx {
            let _ = stream.write_message(&t); // FIXME: don't ignore errors
          }
        }
        client.reregister(&mut poll.lock().unwrap(), Ready::readable(), PollOpt::edge());
      }

      if event.readiness().is_readable() {
        let message = match client.tls.as_mut().unwrap().read_message() {
          Ok(m) => m,
          Err(_) => continue // wait until we have a message
        };
        client.receive(message, &mut poll.lock().unwrap());
      }
    }
  }
}

struct Client {
  tcp: Option<TcpStream>,
  mid: Option<MidHandshakeTlsStream<TcpStream>>,
  tls: Option<TlsStream<TcpStream>>,
  state: State,
  tx: Vec<Message>,
  last_ping: (u32, DateTime<Utc>),
  clipboard: ClipboardContext
}

impl Client {
  fn new(tcp: TcpStream) -> Client {
    Client {
      tcp: Some(tcp),
      mid: None,
      tls: None,
      state: State::default(),
      tx: Vec::new(),
      last_ping: (0, Utc::now()),
      clipboard: ClipboardContext::new().unwrap()
    }
  }

  fn try_start_tls(&mut self, connector: &TlsConnector, hostname: &str) -> io::Result<bool> {
    match self.tcp.take() {
      Some(tcp) => {
        match connector.connect(hostname, tcp) {
          Ok(s) => {
            self.tls = Some(s);
            Ok(true)
          },
          Err(native_tls::HandshakeError::Interrupted(s)) => {
            self.mid = Some(s);
            Ok(false)
          },
          Err(e) => Err(io::Error::new(io::ErrorKind::Other, e))
        }
      },
      None => Ok(true)
    }
  }

  fn try_resume_tls(&mut self) -> io::Result<bool> {
    match self.mid.take() {
      Some(tcp) => {
        match tcp.handshake() {
          Ok(s) => {
            self.tls = Some(s);
            Ok(true)
          },
          Err(native_tls::HandshakeError::Interrupted(s)) => {
            self.mid = Some(s);
            Ok(false)
          },
          Err(e) => Err(io::Error::new(io::ErrorKind::Other, e))
        }
      },
      None => Ok(true)
    }
  }


  fn queue_message(&mut self, msg: Message, poll: &mut Poll) {
    self.tx.push(msg);
    self.reregister(poll, Ready::writable(), PollOpt::edge() | PollOpt::oneshot());
  }

  #[allow(unused)]
  fn queue_messages<I>(&mut self, msgs: I, poll: &mut Poll)
    where I: IntoIterator<Item=Message>
  {
    self.tx.extend(msgs);
    self.reregister(poll, Ready::writable(), PollOpt::edge() | PollOpt::oneshot());
  }

  fn reregister(&mut self, poll: &mut Poll, ready: Ready, opts: PollOpt) {
    poll.reregister(self.tls.as_ref().unwrap().get_ref(), CLIENT, ready, opts).unwrap();
  }

  fn send_thread(client: Arc<RwLock<Client>>, poll: Arc<Mutex<Poll>>) {
    std::thread::spawn(move || {
      loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        if !client.read().unwrap().state.registered {
          continue;
        }
        let now = Utc::now();
        let mut client = client.write().unwrap();
        if now.signed_duration_since(client.last_ping.1).num_seconds().abs() > 15 {
          let mut poll = poll.lock().unwrap();
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
        if local == shared {
          continue;
        }
        let mut cu = ClipboardUpdate::new();
        cu.set_contents(local);
        client.queue_message(cu.into(), &mut poll.lock().unwrap());
      }
    });
  }

  fn receive(&mut self, mut message: Message, poll: &mut Poll) {
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
