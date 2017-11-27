extern crate lan_clipboard;
extern crate protobuf;
extern crate integer_encoding;
extern crate native_tls;
extern crate mio;
extern crate slab;
extern crate bytes;

use lan_clipboard::*;
use native_tls::{Pkcs12, TlsAcceptor, TlsStream};
use mio::*;
use mio::net::{TcpListener, TcpStream};
use slab::Slab;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::io::{self, Read};

const SERVER: Token = Token(1024);

#[derive(Default)]
struct State {
  shared: Vec<u8>
}

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.is_empty() {
    println!("usage: server [hostname:port] [pkcs12 archive] [archive password]");
    return;
  }
  let bind_addr: SocketAddr = match args[0].to_socket_addrs() {
    Ok(mut b) => match b.next() {
      Some(b) => b,
      None => {
        println!("No addresses provided.");
        return;
      }
    },
    Err(e) => {
      println!("Invalid hostname:port: {}", e);
      return;
    }
  }; // ("0.0.0.0", 38153)
  let archive = &args[1];
  let passwd = &args[2];

  let mut f = match File::open(archive) {
    Ok(f) => f,
    Err(e) => {
      println!("could not open pkcs12 archive: {}", e);
      return;
    }
  };
  let mut pkcs = Vec::new();
  if let Err(e) = f.read_to_end(&mut pkcs) {
    println!("could not read pkcs12 archive: {}", e);
    return;
  }
  let pkcs = match Pkcs12::from_der(&pkcs, passwd) {
    Ok(p) => p,
    Err(e) => {
      println!("could not parse pkcs12 archive: {}", e);
      return;
    }
  };

  let mut conn_poll = Poll::new().unwrap();

  let listener = TcpListener::bind(&bind_addr).expect("could not bind");
  let acceptor = TlsAcceptor::builder(pkcs).unwrap().build().unwrap();

  conn_poll.register(&listener, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

  let mut server = Server {
    listener: listener,
    acceptor: acceptor,
    nodes: Slab::with_capacity(4),
    state: Default::default()
  };

  let mut conn_events = Events::with_capacity(128);

  loop {
    conn_poll.poll(&mut conn_events, Some(std::time::Duration::from_millis(100))).unwrap();
    for event in conn_events.iter() {
      if event.readiness().is_readable() {
        let res = match event.token() {
          SERVER => server.accept(&mut conn_poll),
          i => server.node_readable(&mut conn_poll, i)
        };
        if let Err(e) = res {
          println!("error reading: {}", e);
        }
      }

      if event.readiness().is_writable() {
        let res = match event.token() {
          SERVER => panic!("server was writable"),
          i => server.node_writable(&mut conn_poll, i)
        };
        if let Err(e) = res {
          println!("error writing: {}", e);
        }
      }
    }
  }
}

struct Server {
  listener: TcpListener,
  acceptor: TlsAcceptor,
  nodes: Slab<Node>,
  state: State
}

impl Server {
  fn accept(&mut self, poll: &mut Poll) -> io::Result<()> {
    let (sock, addr) = self.listener.accept()?;
    let entry: slab::VacantEntry<_> = self.nodes.vacant_entry();
    let key = entry.key();
    let token = Token(key);
    poll.register(&sock, token, Ready::readable() | Ready::writable(), PollOpt::edge()).unwrap();
    let mut node = Node::new(key as u32, sock, addr, token);
    node.try_start_tls(&self.acceptor)?;
    entry.insert(node);
    Ok(())
  }

  fn node_readable(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    let message = {
      let node = &mut self.nodes[tok.0];
      if !node.try_resume_tls()? {
        return Ok(());
      }
      let message = {
        let stream = node.stream.as_mut().unwrap();
        match stream.read_message() {
          Ok(m) => m,
          Err(_) => return Ok(()) // wait until we have a message
        }
      };
      node.reregister(poll, Ready::writable(), PollOpt::edge() | PollOpt::oneshot());
      message
    };
    match message.get_field_type() {
      Message_MessageType::HELLO => self.hello(tok, message, poll),
      Message_MessageType::CLIPBOARD_UPDATE => self.clipboard_update(message, poll),
      Message_MessageType::PING => self.ping(tok, message, poll),
      _ => {}
    }
    Ok(())
  }

  fn node_writable(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    self.nodes[tok.0].writable(poll)
  }

  fn hello(&mut self, token: Token, mut message: Message, poll: &mut Poll) {
    if !message.has_hello() {
      return;
    }
    let mut hello = message.take_hello();

    {
      let lower = Some(hello.get_name().to_lowercase());
      if self.nodes.iter().any(|(_, x)| x.name.as_ref().map(|z| z.to_lowercase()) == lower) {
        let mut r = Rejected::new();
        r.set_reason(Rejected_RejectionReason::BAD_NAME);
        self.nodes[token.0].queue_message(r.into(), poll);
        return;
      }

      self.nodes[token.0].name = Some(hello.take_name());
    }

    let num: Message = {
      let node = &mut self.nodes[token.0];

      let mut node_update: NodeUpdate = NodeUpdate::new();
      node_update.set_field_type(NodeUpdate_UpdateType::ADDED);
      node_update.set_node_id(node.id);
      node_update.set_node_name(node.name.clone().unwrap_or_default());

      node_update.into()
    };

    for (_, n) in self.nodes.iter_mut() {
      n.queue_message(num.clone(), poll);
    }

    let mut n_tree = lan_clipboard::NodeTree::new();
    n_tree.set_nodes(self.nodes.iter().map(|(_, node)| (node.id, node.name.clone().unwrap_or_default())).collect());

    let mut reg: Registered = Registered::new();
    reg.set_node_id(token.0 as u32);
    reg.set_num_nodes(self.nodes.len() as u32);
    reg.set_tree(n_tree);
    reg.set_clipboard(Vec::new());

    let message: Message = reg.into();
    self.nodes[token.0].queue_message(message, poll);
  }

  fn ping(&mut self, token: Token, mut message: Message, poll: &mut Poll) {
    if !message.has_ping() {
      return;
    }

    let ping = message.take_ping();
    let seq = ping.get_seq();

    let mut pong = Pong::new();
    pong.set_seq(seq);

    self.nodes[token.0].queue_message(pong.into(), poll);
  }

  fn clipboard_update(&mut self, mut message: Message, poll: &mut Poll) {
    if !message.has_clipboard_update() {
      return;
    }

    let mut cu = message.take_clipboard_update();
    let new = cu.take_contents();

    self.state.shared = new;

    // keep write locked to ensure correct packet ordering
    let mut update = ClipboardUpdate::new();
    update.set_contents(self.state.shared.clone());

    let m: Message = update.into();

    for (_, node) in self.nodes.iter_mut() {
      node.queue_message(m.clone(), poll);
    }
  }
}

struct Node {
  id: u32,
  token: Token,
  name: Option<String>,
  #[allow(unused)]
  address: SocketAddr,
  tcp: Option<TcpStream>,
  mid: Option<native_tls::MidHandshakeTlsStream<TcpStream>>,
  stream: Option<TlsStream<TcpStream>>,
  tx: Vec<Message>
}

impl Node {
  fn new(id: u32, tcp: TcpStream, address: SocketAddr, token: Token) -> Node {
    Node {
      id,
      token,
      name: None,
      address,
      tcp: Some(tcp),
      mid: None,
      stream: None,
      tx: Vec::new()
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
    poll.reregister(self.stream.as_ref().unwrap().get_ref(), self.token, ready, opts).unwrap();
  }

  fn try_start_tls(&mut self, acceptor: &TlsAcceptor) -> io::Result<bool> {
    match self.tcp.take() {
      Some(tcp) => {
        match acceptor.accept(tcp) {
          Ok(s) => {
            self.stream = Some(s);
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
            self.stream = Some(s);
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

  fn writable(&mut self, poll: &mut Poll) -> io::Result<()> {
    if !self.try_resume_tls()? {
      return Ok(());
    }
    if self.tx.is_empty() {
      self.reregister(poll, Ready::readable(), PollOpt::edge());
      return Ok(());
    }
    {
      let stream = self.stream.as_mut().unwrap();
      for t in self.tx.drain(..) {
        let _ = stream.write_message(&t); // FIXME: don't ignore errors
      }
    }
    self.reregister(poll, Ready::readable(), PollOpt::edge());
    Ok(())
  }
}
