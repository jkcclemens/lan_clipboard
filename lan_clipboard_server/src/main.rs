extern crate lan_clipboard;
extern crate protobuf;
extern crate integer_encoding;
extern crate rustls;
extern crate mio;
extern crate slab;
extern crate untrusted;
extern crate webpki;

use lan_clipboard::*;
use rustls::{ServerConfig, ServerSession, Session};
use mio::*;
use mio::net::{TcpListener, TcpStream};
use slab::Slab;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;

const SERVER: Token = Token(1024);

#[derive(Default)]
struct State {
  shared: Vec<u8>
}

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.is_empty() {
    println!("usage: lan_clipboard_server [hostname:port] [certificate pem] [key pem]");
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
  let certs = &args[1];
  let keys = &args[2];

  let f = match File::open(certs) {
    Ok(f) => f,
    Err(e) => {
      println!("could not open cert: {}", e);
      return;
    }
  };
  let certs = rustls::internal::pemfile::certs(&mut io::BufReader::new(f)).unwrap();
  if certs.is_empty() {
    println!("No certs found.");
    return;
  }
  if let Err(e) = webpki::EndEntityCert::from(untrusted::Input::from(&certs[0].0)) {
    println!("invalid cert: {:?}", e);
    return;
  }

  let f = match File::open(keys) {
    Ok(f) => f,
    Err(e) => {
      println!("could not open key: {}", e);
      return;
    }
  };
  let mut keys = rustls::internal::pemfile::rsa_private_keys(&mut io::BufReader::new(f)).unwrap();
  if keys.is_empty() {
    println!("No keys found.");
    return;
  }
  let key = keys.remove(0);

  let mut conn_poll = Poll::new().unwrap();

  let listener = TcpListener::bind(&bind_addr).expect("could not bind");
  let mut config = ServerConfig::new();
  config.set_single_cert(certs, key);
  let config = Arc::new(config);

  conn_poll.register(&listener, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

  let mut server = Server {
    listener,
    config,
    nodes: Slab::with_capacity(4),
    state: Default::default()
  };

  let mut conn_events = Events::with_capacity(128);

  loop {
    conn_poll.poll(&mut conn_events, Some(std::time::Duration::from_millis(100))).unwrap();
    for event in conn_events.iter() {
      if event.readiness().is_writable() {
        let res = match event.token() {
          SERVER => panic!("server was writable"),
          i => server.node_writable(&mut conn_poll, i)
        };
        if let Err(e) = res {
          println!("error writing from node {}: {}\nshutting down that node", event.token().0, e);
          server.nodes[event.token().0].shutting_down = true;
          server.hangup(&mut conn_poll, event.token());
          server.nodes.retain(|token, _| token != event.token().0);
        }
      }

      if event.readiness().is_readable() {
        let res = match event.token() {
          SERVER => server.accept(&mut conn_poll),
          i => server.node_readable(&mut conn_poll, i)
        };
        if let Err(e) = res {
          println!("error reading from node {}: {}\nshutting down that node", event.token().0, e);
          server.nodes[event.token().0].shutting_down = true;
          server.hangup(&mut conn_poll, event.token());
          server.nodes.retain(|token, _| token != event.token().0);
        }
      }
    }
  }
}

struct Server {
  listener: TcpListener,
  config: Arc<ServerConfig>,
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
    let session = ServerSession::new(&self.config);
    let node = Node::new(key as u32, sock, session, addr, token);
    entry.insert(node);
    Ok(())
  }

  fn hangup(&mut self, poll: &mut Poll, tok: Token) {
    let node: &mut Node = &mut self.nodes[tok.0];
    if let Err(e) = poll.deregister(&node.tcp) {
      println!("error deregistering: {}", e);
    }
    node.tcp.shutdown(std::net::Shutdown::Both).ok();
  }

  fn node_readable(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    let res = {
      let node = &mut self.nodes[tok.0];
      if node.session.wants_read() {
        match node.do_tls_read() {
          Ok(0) if !node.session.wants_write() => return Err(io::Error::from(io::ErrorKind::BrokenPipe)),
          Err(e) => return Err(e),
          _ => {}
        }
      }
      node.read_to_buf()?;
      let (res, pos) = {
        let mut cursor = std::io::Cursor::new(&node.buf);
        (cursor.read_message(), cursor.position())
      };
      if res.is_ok() {
        node.buf = node.buf.split_off(pos as usize);
      }
      res
    };
    if let Ok(message) = res {
      match message.get_field_type() {
        Message_MessageType::HELLO => self.hello(tok, message, poll),
        Message_MessageType::CLIPBOARD_UPDATE => self.clipboard_update(message, poll),
        Message_MessageType::PING => self.ping(tok, message, poll),
        _ => {}
      }
    }
    self.nodes[tok.0].reregister(poll);
    Ok(())
  }

  fn node_writable(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    let node = &mut self.nodes[tok.0];
    if node.session.wants_write() {
      node.do_tls_write();
    }
    let res = node.writable();
    node.reregister(poll);
    res
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

    for (_, n) in self.nodes.iter_mut().filter(|x| x.0 != token.0) {
      n.queue_message(num.clone(), poll);
    }

    let mut n_tree = lan_clipboard::NodeTree::new();
    n_tree.set_nodes(self.nodes.iter().map(|(_, node)| (node.id, node.name.clone().unwrap_or_default())).collect());

    let mut reg: Registered = Registered::new();
    reg.set_node_id(token.0 as u32);
    reg.set_num_nodes(self.nodes.len() as u32);
    reg.set_tree(n_tree);
    reg.set_clipboard(Vec::new());

    self.nodes[token.0].queue_message(reg.into(), poll);
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

    let mut update = ClipboardUpdate::new();
    update.set_contents(self.state.shared.clone());

    let m: Message = update.into();

    for (_, node) in self.nodes.iter_mut() {
      println!("sending update to node {}", node.id);
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
  tcp: TcpStream,
  session: ServerSession,
  tx: Vec<Message>,
  buf: Vec<u8>,
  shutting_down: bool
}

impl Node {
  fn new(id: u32, tcp: TcpStream, session: ServerSession, address: SocketAddr, token: Token) -> Node {
    Node {
      id,
      token,
      name: None,
      address,
      tcp,
      session,
      tx: Vec::new(),
      buf: Vec::new(),
      shutting_down: false
    }
  }

  fn read_to_buf(&mut self) -> io::Result<usize> {
    self.session.read_to_end(&mut self.buf)
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
    poll.reregister(&self.tcp, self.token, self.event_set(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
  }

  fn writable(&mut self) -> io::Result<()> {
    if self.tx.is_empty() {
      return Ok(());
    }
    {
      for t in self.tx.drain(..) {
        self.session.write_message(&t)
          .map_err(|e| match e {
            MessageError::Io(e) => e,
            MessageError::Protobuf(e) => io::Error::new(io::ErrorKind::Other, e)
          })?;
      }
    }
    Ok(())
  }
}
