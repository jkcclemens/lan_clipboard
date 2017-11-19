extern crate lan_clipboard;
extern crate protobuf;
extern crate integer_encoding;

use lan_clipboard::*;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, RwLock};

#[derive(Default)]
struct State {
  node_tree: Vec<Node>,
  shared: Vec<u8>
}

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.is_empty() {
    println!("Specify 'hostname:port' to run the server on.");
    return;
  }
  let bind_addr = &args[0]; // ("0.0.0.0", 38153)

  let state = Arc::new(RwLock::new(State::default()));

  let listener = TcpListener::bind(bind_addr).expect("could not bind");
  for connection in listener.incoming() {
    let mut connection = match connection {
      Ok(c) => c,
      Err(e) => {
        println!("could not accept connection: {}", e);
        continue;
      }
    };

    let address = match connection.peer_addr() {
      Ok(a) => a,
      Err(e) => {
        println!("could not get peer address: {}", e);
        continue;
      }
    };

    let t_state = state.clone();
    std::thread::spawn(move || {
      loop {
        let message: Message = match connection.read_message() {
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
          Message_MessageType::HELLO => hello(t_state.clone(), message, &mut connection),
          Message_MessageType::CLIPBOARD_UPDATE => clipboard_update(t_state.clone(), message),
          _ => println!("received other message not yet supported")
        }
      }
      println!("Stream closing, removing from node tree");
      let mut state = t_state.write().unwrap();
      if let Some(pos) = state.node_tree.iter().position(|node| node.address == address) {
        let node = state.node_tree.remove(pos);

        let mut nu = NodeUpdate::new();
        nu.set_field_type(NodeUpdate_UpdateType::REMOVED);
        nu.set_node_id(node.id);

        let message = nu.into();

        for node in state.node_tree.iter_mut() {
          node.stream.write_message(&message).ok();
        }
      }
    });
  }
}

fn hello(state: Arc<RwLock<State>>, mut message: Message, connection: &mut TcpStream) {
  if !message.has_hello() {
    return;
  }
  let mut hello = message.take_hello();

  {
    let state = state.read().unwrap();
    if state.node_tree.name_used(hello.get_name()) {
      let mut r = Rejected::new();
      r.set_reason(Rejected_RejectionReason::BAD_NAME);
      connection.write_message(&r.into()).ok();
      return;
    }
  }
  {
    // make node here to ensure no id conflicts
    let mut state = state.write().unwrap();
    let id = state.node_tree.next_id();
    let node = Node {
      id,
      address: connection.peer_addr().unwrap(),
      stream: connection.try_clone().unwrap(),
      name: hello.take_name()
    };

    let mut node_update: NodeUpdate = NodeUpdate::new();
    node_update.set_field_type(NodeUpdate_UpdateType::ADDED);
    node_update.set_node_id(node.id);
    node_update.set_node_name(node.name.clone());

    let num = node_update.into();

    for n in state.node_tree.iter_mut() {
      n.stream.write_message(&num).ok();
    }

    state.node_tree.push(node);

    let mut n_tree = lan_clipboard::NodeTree::new();
    n_tree.set_nodes(state.node_tree.iter().map(|node| (node.id, node.name.clone())).collect());

    let mut reg: Registered = Registered::new();
    reg.set_node_id(id);
    reg.set_num_nodes(state.node_tree.len() as u32);
    reg.set_tree(n_tree);
    reg.set_clipboard(state.shared.clone());

    connection.write_message(&reg.into()).ok();
  }
}

fn clipboard_update(state: Arc<RwLock<State>>, mut message: Message) {
  if !message.has_clipboard_update() {
    return;
  }

  let mut cu = message.take_clipboard_update();
  let new = cu.take_contents();

  let mut state = state.write().unwrap();
  state.shared = new;

  // keep write locked to ensure correct packet ordering
  let mut update = ClipboardUpdate::new();
  update.set_contents(state.shared.clone());

  let m = update.into();

  for node in state.node_tree.iter_mut() {
    node.stream.write_message(&m).ok();
  }
}

#[derive(Debug)]
struct Node {
  id: u32,
  address: SocketAddr,
  stream: TcpStream,
  name: String
}

trait NodeTree {
  fn next_id(&self) -> u32;

  fn name_used(&self, name: &str) -> bool;
}

impl NodeTree for Vec<Node> {
  fn next_id(&self) -> u32 {
    // FIXME: keep counter and never re-use an id
    self.iter().map(|x| x.id).max().unwrap_or_default() + 1
  }

  fn name_used(&self, name: &str) -> bool {
    let lower = name.to_lowercase();
    self.iter().any(|x| x.name.to_lowercase() == lower)
  }
}
