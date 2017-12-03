use state::State;
use lan_clipboard::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use rustls::{ClientSession, Session};
use snap::Writer as SnappyWriter;
use chrono::{Utc, DateTime};
use parking_lot::Mutex;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use mio::*;
use mio::net::TcpStream;
use std::net::Shutdown;
use std::sync::Arc;
use std::io::{self, Read, Write};
use std::time::Duration;
use std::thread;

pub const CLIENT: Token = Token(0);

pub struct Client {
  pub tcp: TcpStream,
  pub session: ClientSession,
  pub state: State,
  pub tx: Vec<Message>,
  pub buf: Vec<u8>,
  pub last_ping: (u32, DateTime<Utc>),
  pub last_update_hash: Vec<u8>,
  pub clipboard: ClipboardContext
}

impl Client {
  pub fn new(tcp: TcpStream, session: ClientSession) -> Result<Client, String> {
    Ok(Client {
      tcp,
      session,
      state: State::default(),
      tx: Vec::new(),
      buf: Vec::new(),
      last_ping: (0, Utc::now()),
      last_update_hash: Vec::new(),
      clipboard: ClipboardContext::new().map_err(|e| e.to_string())?
    })
  }

  pub fn read_to_buf(&mut self) -> io::Result<usize> {
    self.session.read_to_end(&mut self.buf)
  }

  pub fn hangup(&mut self, poll: &mut Poll) {
    if let Err(e) = poll.deregister(&self.tcp) {
      println!("error deregistering: {}", e);
    }
    self.tcp.shutdown(Shutdown::Both).ok();
  }

  /// What IO events we're currently waiting for,
  /// based on wants_read/wants_write.
  pub fn event_set(&self) -> Ready {
    let rd = self.session.wants_read();
    let wr = self.session.wants_write();

    let mut ready = if rd && wr {
      Ready::readable() | Ready::writable()
    } else if wr {
      Ready::writable()
    } else {
      Ready::readable()
    };

    if !self.tx.is_empty() {
      ready.insert(Ready::writable());
    }
    ready
  }

  pub fn reregister(&self, poll: &mut Poll) -> io::Result<()> {
    poll.reregister(&self.tcp, CLIENT, self.event_set(), PollOpt::edge() | PollOpt::oneshot())
  }

  pub fn do_tls_write(&mut self) {
    if let Err(e) = self.session.write_tls(&mut self.tcp) {
      panic!("write err: {}", e);
    }
    if let Err(e) = self.session.process_new_packets() {
      panic!("process err (post-write): {}", e);
    }
  }

  pub fn do_tls_read(&mut self) -> io::Result<usize> {
    let read = self.session.read_tls(&mut self.tcp)?;
    self.session.process_new_packets().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(read)
  }

  pub fn queue_message(&mut self, msg: Message, poll: &mut Poll) -> io::Result<()> {
    self.tx.push(msg);
    self.reregister(poll)
  }

  #[allow(unused)]
  pub fn queue_messages<I>(&mut self, msgs: I, poll: &mut Poll) -> io::Result<()>
    where I: IntoIterator<Item=Message>
  {
    self.tx.extend(msgs);
    self.reregister(poll)
  }

  pub fn send_thread(client: Arc<Mutex<Client>>, poll: Arc<Mutex<Poll>>) {
    thread::spawn(move || {
      let mut sha3 = Sha3::sha3_512();
      loop {
        thread::sleep(Duration::from_millis(250));
        if !client.lock().state.registered {
          continue;
        }
        let now = Utc::now();
        let mut client = client.lock();
        if now.signed_duration_since(client.last_ping.1).num_seconds().abs() > 15 {
          let mut poll = poll.lock();
          client.last_ping.1 = now;
          client.last_ping.0 += 1;
          let mut ping = Ping::new();
          ping.set_seq(client.last_ping.0);
          println!("ping {}", client.last_ping.0);
          if let Err(e) = client.queue_message(ping.into(), &mut poll) {
            println!("could not queue message: {}", e);
          }
        }
        let shared = client.state.shared.clone();
        let local = match client.clipboard.get_contents() {
          Ok(c) => c.into_bytes(),
          Err(_) => continue
        };
        let mut local_hash = Vec::with_capacity(64);
        unsafe { local_hash.set_len(64); }
        sha3.input(&local);
        sha3.result(&mut local_hash);
        sha3.reset();
        if local == shared || client.last_update_hash == local_hash {
          continue;
        }
        let (compressed, data) = if local.len() > 17 {
          let mut data = Vec::new();
          if let Err(e) = SnappyWriter::new(&mut data).write_all(&local) {
            println!("error compressing data: {}", e);
            continue;
          }
          (true, data)
        } else {
          (false, local.clone())
        };
        let mut cu = ClipboardUpdate::new();
        cu.set_contents(data);
        cu.set_compressed(compressed);
        if let Err(e) = client.queue_message(cu.into(), &mut poll.lock()) {
          println!("could not queue message: {}", e);
        }
        client.last_update_hash = local_hash;
      }
    });
  }

  pub fn receive(&mut self, mut message: Message, _poll: &mut Poll) {
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
