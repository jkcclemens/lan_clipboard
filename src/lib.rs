extern crate protobuf;
extern crate integer_encoding;

pub mod packets;

pub use packets::*;

use protobuf::Message as PMessage;
use integer_encoding::{VarIntReader, VarIntWriter};
use std::io::{Read, Write, Error as IoError};

pub type MessageResult<T> = Result<T, MessageError>;

#[derive(Debug)]
pub enum MessageError {
  Io(IoError),
  Protobuf(protobuf::error::ProtobufError)
}

pub trait MessageReader {
  fn read_message(&mut self) -> MessageResult<Message>;
}

pub trait MessageWriter {
  fn write_message(&mut self, msg: &Message) -> MessageResult<()>;
}

impl<T: Read> MessageReader for T {
  fn read_message(&mut self) -> MessageResult<Message> {
    let size: u32 = self.read_varint().map_err(MessageError::Io)?;
    let mut data = Vec::with_capacity(size as usize);
    unsafe { data.set_len(size as usize); }
    self.read_exact(&mut data).map_err(MessageError::Io)?;
    protobuf::parse_from_bytes(&data).map_err(MessageError::Protobuf)
  }
}

impl<T: Write> MessageWriter for T {
  fn write_message(&mut self, msg: &Message) -> MessageResult<()> {
    let size = msg.compute_size();
    self.write_varint(size).map_err(MessageError::Io)?;
    msg.write_to_writer(self).map_err(MessageError::Protobuf)
  }
}

macro_rules! into_msg {
  ($kind:ident, $message_type:ident, $setter:ident) => {
    impl From<$kind> for Message {
      fn from(k: $kind) -> Message {
        let mut m: Message = Message::new();
        m.set_field_type(Message_MessageType::$message_type);
        m.$setter(k);
        m
      }
    }
  }
}

into_msg!(Hello, HELLO, set_hello);
into_msg!(Registered, REGISTERED, set_registered);
into_msg!(Rejected, REJECTED, set_rejected);
into_msg!(NodeTree, NODE_TREE, set_node_tree);
into_msg!(NodeUpdate, NODE_UPDATE, set_node_update);
into_msg!(ClipboardUpdate, CLIPBOARD_UPDATE, set_clipboard_update);
into_msg!(Ping, PING, set_ping);
into_msg!(Pong, PONG, set_pong);
into_msg!(HangingUp, HANGING_UP, set_hanging_up);
