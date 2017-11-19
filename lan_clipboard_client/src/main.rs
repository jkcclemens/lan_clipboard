extern crate lan_clipboard;
extern crate protobuf;
extern crate clipboard;
extern crate native_tls;
extern crate chrono;

use lan_clipboard::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use native_tls::{TlsConnector, TlsStream, Certificate};
use chrono::Utc;
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::fs::File;
use std::io::Read;

#[derive(Default)]
struct State {
  registered: bool,
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

  let state = Arc::new(RwLock::new(State::default()));

  let mut builder = TlsConnector::builder().unwrap();
  if let Err(e) = builder.add_root_certificate(cert) {
    println!("could not add cert file: {}", e);
    return;
  }
  let connector = builder.build().unwrap();
  let connection = TcpStream::connect((hostname.as_str(), port)).unwrap();
  let connection = Arc::new(Mutex::new(connector.connect(hostname, connection).unwrap()));
  connection.lock().unwrap().get_ref().set_nonblocking(true).unwrap();

  let mut hello: Hello = Hello::new();
  hello.set_version(1);
  hello.set_name(name);

  receive(state.clone(), connection.clone());
  send(state.clone(), connection.clone());

  connection.lock().unwrap().write_message(&hello.into()).unwrap();

  let _ = std::io::stdin().read_line(&mut String::new());
}

fn send(state: Arc<RwLock<State>>, stream: Arc<Mutex<TlsStream<TcpStream>>>) {
  let mut ctx = ClipboardContext::new().unwrap();
  let mut last_ping = (0, Utc::now());
  std::thread::spawn(move || {
    loop {
      std::thread::sleep(std::time::Duration::from_millis(250));
      let reg = {
        let state = state.read().unwrap();
        state.registered
      };
      if !reg {
        continue;
      }
      let now = Utc::now();
      if now.signed_duration_since(last_ping.1).num_seconds().abs() > 15 {
        last_ping.1 = now;
        last_ping.0 = last_ping.0 + 1;
        let mut ping = Ping::new();
        ping.set_seq(last_ping.0);
        stream.lock().unwrap().write_message(&ping.into()).ok();
      }
      let shared = {
        let state = state.read().unwrap();
        state.shared.clone()
      };
      let local = match ctx.get_contents() {
        Ok(c) => c.into_bytes(),
        Err(_) => continue
      };
      if local == shared {
        continue;
      }
      let mut cu = ClipboardUpdate::new();
      cu.set_contents(local);
      stream.lock().unwrap().write_message(&cu.into()).ok();
    }
  });
}

fn receive(state: Arc<RwLock<State>>, stream: Arc<Mutex<TlsStream<TcpStream>>>) {
  std::thread::spawn(move || {
    loop {
      let mut message: Message = match stream.lock().unwrap().read_message() {
        Ok(m) => m,
        Err(MessageError::Io(ref e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
          std::thread::sleep(std::time::Duration::from_millis(250));
          continue;
        },
        Err(MessageError::Io(e)) => {
          println!("could not read from connection: {}\nclosing stream", e);
          break;
        },
        Err(MessageError::Protobuf(e)) => {
          println!("could not parse message: {}", e);
          continue;
        }
      };
      match message.get_field_type() {
        Message_MessageType::CLIPBOARD_UPDATE => {
          if !message.has_clipboard_update() {
            continue;
          }
          let mut cu = message.take_clipboard_update();
          let new = cu.take_contents();
          let mut state = state.write().unwrap();
          state.shared = new;
          state.update_clipboard();
        },
        Message_MessageType::REGISTERED => {
          if !message.has_registered() {
            continue;
          }
          let mut registered: Registered = message.take_registered();
          let mut state = state.write().unwrap();
          state.registered = true;
          state.tree = registered.take_tree().take_nodes();
          state.shared = registered.get_clipboard().to_vec();
          state.update_clipboard();
          println!("Registered as node {} with node tree:\n{:#?}", registered.get_node_id(), state.tree);
        },
        Message_MessageType::REJECTED => {
          if !message.has_rejected() {
            continue;
          }
          let rejected: Rejected = message.take_rejected();
          println!("Rejected: {:?}", rejected.get_reason());
          break;
        },
        Message_MessageType::NODE_UPDATE => {
          if !message.has_node_update() {
            continue;
          }
          let node_update: NodeUpdate = message.take_node_update();
          let mut state = state.write().unwrap();
          match node_update.get_field_type() {
            NodeUpdate_UpdateType::ADDED => {
              state.tree.insert(node_update.get_node_id(), node_update.get_node_name().into());
            },
            NodeUpdate_UpdateType::REMOVED => {
              state.tree.remove(&node_update.get_node_id());
            }
          }
          println!("Node {} was {:?}", node_update.get_node_id(), node_update.get_field_type());
          println!("new tree: {:#?}", state.tree);
        },
        _ => println!("received unsupported message")
      }
    }
  });
}
