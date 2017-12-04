use lan_clipboard::*;
use rustls::{ServerConfig, ServerSession, Session};
use snap::{self, Reader as SnappyReader, Writer as SnappyWriter};
use chrono::Utc;
use mio::*;
use mio::net::TcpListener;
use slab::{Slab, VacantEntry};
use std::io::{self, Read, Write, Cursor};
use std::net::Shutdown;
use std::sync::Arc;

use config::Config;

pub mod node;
pub mod state;

use self::node::Node;
use self::state::State;

pub const SERVER: Token = Token(1024);

pub struct Server {
  pub listener: TcpListener,
  pub config: Arc<ServerConfig>,
  pub app_config: Config,
  pub nodes: Slab<Node>,
  pub state: State
}

impl Server {
  pub fn accept(&mut self, poll: &mut Poll) -> io::Result<()> {
    let (sock, addr) = self.listener.accept()?;
    if let Some(max_clients) = self.app_config.connection.max_clients {
      if self.nodes.len() >= max_clients {
        sock.shutdown(Shutdown::Both).ok();
        return Ok(());
      }
    }
    let entry: VacantEntry<_> = self.nodes.vacant_entry();
    let key = entry.key();
    let token = Token(key);
    poll.register(&sock, token, Ready::readable() | Ready::writable(), PollOpt::edge())?;
    let session = ServerSession::new(&self.config);
    let node = Node::new(key as u32, sock, session, addr, token);
    entry.insert(node);
    Ok(())
  }

  pub fn hangup(&mut self, poll: &mut Poll, tok: Token) {
    let node: &mut Node = &mut self.nodes[tok.0];
    if let Err(e) = poll.deregister(&node.tcp) {
      println!("error deregistering: {}", e);
    }
    node.tcp.shutdown(Shutdown::Both).ok();
  }

  pub fn remove_node(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    if !self.nodes.contains(tok.0) {
      return Ok(());
    }
    let node = self.nodes.remove(tok.0);

    if !node.registered {
      return Ok(());
    }

    let mut removal: NodeUpdate = NodeUpdate::new();
    removal.set_field_type(NodeUpdate_UpdateType::REMOVED);
    removal.set_node_id(node.id);
    if let Some(name) = node.name {
      removal.set_node_name(name);
    }

    let msg: Message = removal.into();

    for (_, node) in self.nodes.iter_mut() {
      node.queue_message(msg.clone(), poll)?;
    }

    Ok(())
  }

  pub fn node_readable(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    let res = {
      let node = &mut self.nodes[tok.0];
      if node.shutting_down {
        // ignore reading if we're hanging up on them
        return Ok(());
      }
      if node.session.wants_read() {
        match node.do_tls_read() {
          Ok(0) if !node.session.wants_write() => return Err(io::Error::from(io::ErrorKind::BrokenPipe)),
          Err(e) => return Err(e),
          _ => {}
        }
      }
      node.read_to_buf()?;
      if node.buf.len() > self.app_config.connection.max_message_size as usize {
        node.shutting_down = true;
        let mut hup: HangingUp = HangingUp::new();
        hup.set_reason(HangingUp_HangUpReason::MESSAGE_TOO_LARGE);
        node.queue_message(hup.into(), poll)?;
        return Ok(());
      }
      let (res, pos) = {
        let mut cursor = Cursor::new(&node.buf);
        (cursor.read_message(), cursor.position())
      };
      if let Err(MessageError::Protobuf(_)) = res {
        node.shutting_down = true;
        let mut hup: HangingUp = HangingUp::new();
        hup.set_reason(HangingUp_HangUpReason::INVALID_MESSAGE);
        node.queue_message(hup.into(), poll)?;
        return Ok(());
      }
      if res.is_ok() {
        node.buf = node.buf.split_off(pos as usize);
      }
      res
    };
    if let Ok(message) = res {
      match message.get_field_type() {
        Message_MessageType::HELLO => self.hello(tok, message, poll)?,
        Message_MessageType::CLIPBOARD_UPDATE => self.clipboard_update(message, poll)?,
        Message_MessageType::PING => self.ping(tok, message, poll)?,
        _ => {}
      }
    }
    self.nodes[tok.0].reregister(poll)
  }

  pub fn node_writable(&mut self, poll: &mut Poll, tok: Token) -> io::Result<()> {
    let remove_ready = {
      let node = &mut self.nodes[tok.0];
      if node.session.wants_write() {
        match node.do_tls_write() {
          Ok(0) if !node.session.wants_read() => return Err(io::Error::from(io::ErrorKind::BrokenPipe)),
          Err(e) => return Err(e),
          _ => {}
        }
      }
      node.writable()?;
      let remove_ready = node.tx.is_empty() && !node.session.wants_write();
      if !node.shutting_down || !remove_ready {
        return node.reregister(poll);
      }
      remove_ready
    };
    if remove_ready {
      self.hangup(poll, tok);
      self.remove_node(poll, tok)
    } else {
      Ok(())
    }
  }

  fn hello(&mut self, token: Token, mut message: Message, poll: &mut Poll) -> io::Result<()> {
    if !message.has_hello() {
      return Ok(());
    }
    let mut hello = message.take_hello();

    {
      let lower = Some(hello.get_name().to_lowercase());
      if self.nodes.iter().any(|(_, x)| x.name.as_ref().map(|z| z.to_lowercase()) == lower) {
        let mut r = Rejected::new();
        r.set_reason(Rejected_RejectionReason::BAD_NAME);
        self.nodes[token.0].queue_message(r.into(), poll)?;
        self.nodes[token.0].shutting_down = true;
        return Ok(());
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
      n.queue_message(num.clone(), poll)?;
    }

    let mut n_tree: NodeTree = NodeTree::new();
    n_tree.set_nodes(self.nodes.iter()
      .filter(|&(_, node)| node.token == token || node.registered)
      .map(|(_, node)| (node.id, node.name.clone().unwrap_or_default())).collect());
    let mut reg: Registered = Registered::new();
    reg.set_node_id(token.0 as u32);
    reg.set_num_nodes(n_tree.nodes.len() as u32);
    reg.set_tree(n_tree);
    reg.set_clipboard(Vec::new());
    reg.set_max_message_size(self.app_config.connection.max_message_size);

    self.nodes[token.0].registered = true;
    self.nodes[token.0].queue_message(reg.into(), poll)
  }

  fn ping(&mut self, token: Token, message: Message, poll: &mut Poll) -> io::Result<()> {
    if !message.has_ping() {
      return Ok(());
    }

    let mut pong = Pong::new();
    pong.set_rand(message.get_ping().get_rand());

    self.nodes[token.0].last_ping = Some(Utc::now());
    self.nodes[token.0].queue_message(pong.into(), poll)
  }

  fn clipboard_update(&mut self, mut message: Message, poll: &mut Poll) -> io::Result<()> {
    if !message.has_clipboard_update() {
      return Ok(());
    }

    let mut cu = message.take_clipboard_update();
    let new = if cu.get_compressed() {
      let contents = cu.take_contents();
      let mut data: Vec<u8> = Vec::with_capacity(snap::decompress_len(&contents)?);
      SnappyReader::new(&*contents).read_to_end(&mut data)?;
      data
    } else {
      cu.take_contents()
    };

    self.state.shared = new;

    let mut update = ClipboardUpdate::new();
    let (compressed, data) = if self.state.shared.len() > 17 {
      let mut data = Vec::new();
      SnappyWriter::new(&mut data).write_all(&self.state.shared)?;
      (true, data)
    } else {
      (false, self.state.shared.clone())
    };
    update.set_contents(data);
    update.set_compressed(compressed);

    let m: Message = update.into();

    for (_, node) in self.nodes.iter_mut() {
      node.queue_message(m.clone(), poll)?;
    }

    Ok(())
  }
}
