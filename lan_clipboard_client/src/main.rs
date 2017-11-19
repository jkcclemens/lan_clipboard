extern crate lan_clipboard;
extern crate protobuf;
extern crate clipboard;

use lan_clipboard::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
struct State {
  registered: bool,
  tree: HashMap<u32, String>,
  shared: Vec<u8>
}

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.len() < 2 {
    println!("Specify a server 'hostname:port' and a client name");
    return;
  }
  let server = &args[0];
  let name = args[1..].join(" ");

  let state = Arc::new(RwLock::new(State::default()));

  let mut connection = TcpStream::connect(server).unwrap();

  let mut hello: Hello = Hello::new();
  hello.set_version(1);
  hello.set_name(name);

  receive(state.clone(), connection.try_clone().unwrap());
  send(state.clone(), connection.try_clone().unwrap());

  connection.write_message(&hello.into()).unwrap();

  let _ = std::io::stdin().read_line(&mut String::new());
}

fn send(state: Arc<RwLock<State>>, mut stream: TcpStream) {
  let mut ctx = ClipboardContext::new().unwrap();
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
      stream.write_message(&cu.into()).ok();
    }
  });
}

fn receive(state: Arc<RwLock<State>>, mut stream: TcpStream) {
  std::thread::spawn(move || {
    loop {
      let mut message: Message = match stream.read_message() {
        Ok(m) => m,
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
          if let Ok(s) = String::from_utf8(state.shared.clone()) {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(s).unwrap();
          }
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
