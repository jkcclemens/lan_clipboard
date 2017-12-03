use lan_clipboard::*;
use rustls::{ServerSession, Session};
use chrono::{DateTime, Utc};
use mio::*;
use mio::net::TcpStream;
use std::io::{self, Read, BufWriter};
use std::net::SocketAddr;

pub struct Node {
  pub id: u32,
  pub token: Token,
  pub name: Option<String>,
  #[allow(unused)]
  pub address: SocketAddr,
  pub tcp: TcpStream,
  pub session: ServerSession,
  pub tx: Vec<Message>,
  pub buf: Vec<u8>,
  pub registered: bool,
  pub shutting_down: bool,
  pub connected_at: DateTime<Utc>,
  pub last_ping: Option<DateTime<Utc>>
}

impl Node {
  pub fn new(id: u32, tcp: TcpStream, session: ServerSession, address: SocketAddr, token: Token) -> Node {
    Node {
      id,
      token,
      name: None,
      address,
      tcp,
      session,
      tx: Vec::new(),
      buf: Vec::new(),
      registered: false,
      shutting_down: false,
      connected_at: Utc::now(),
      last_ping: None
    }
  }

  pub fn read_to_buf(&mut self) -> io::Result<usize> {
    self.session.read_to_end(&mut self.buf)
  }

  pub fn do_tls_write(&mut self) -> io::Result<usize> {
    let read = self.session.write_tls(&mut self.tcp)?;
    self.session.process_new_packets().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(read)
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
    poll.reregister(&self.tcp, self.token, self.event_set(), PollOpt::edge() | PollOpt::oneshot())
  }

  pub fn writable(&mut self) -> io::Result<()> {
    if self.tx.is_empty() {
      return Ok(());
    }
    {
      let mut writer = BufWriter::new(self.session.by_ref());
      for t in self.tx.drain(..) {
        writer.write_message(&t)
          .map_err(|e| match e {
            MessageError::Io(e) => e,
            MessageError::Protobuf(e) => io::Error::new(io::ErrorKind::Other, e)
          })?;
      }
    }
    Ok(())
  }
}
