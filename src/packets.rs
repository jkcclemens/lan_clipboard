// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub field_type: Message_MessageType,
    // message oneof groups
    value: ::std::option::Option<Message_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

#[derive(Clone,PartialEq)]
pub enum Message_oneof_value {
    hello(Hello),
    registered(Registered),
    rejected(Rejected),
    node_tree(NodeTree),
    node_update(NodeUpdate),
    clipboard_update(ClipboardUpdate),
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // .Message.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = Message_MessageType::HELLO;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Message_MessageType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Message_MessageType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Message_MessageType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Message_MessageType {
        &mut self.field_type
    }

    // .Hello hello = 2;

    pub fn clear_hello(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_hello(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::hello(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hello(&mut self, v: Hello) {
        self.value = ::std::option::Option::Some(Message_oneof_value::hello(v))
    }

    // Mutable pointer to the field.
    pub fn mut_hello(&mut self) -> &mut Hello {
        if let ::std::option::Option::Some(Message_oneof_value::hello(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Message_oneof_value::hello(Hello::new()));
        }
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::hello(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hello(&mut self) -> Hello {
        if self.has_hello() {
            match self.value.take() {
                ::std::option::Option::Some(Message_oneof_value::hello(v)) => v,
                _ => panic!(),
            }
        } else {
            Hello::new()
        }
    }

    pub fn get_hello(&self) -> &Hello {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::hello(ref v)) => v,
            _ => Hello::default_instance(),
        }
    }

    // .Registered registered = 3;

    pub fn clear_registered(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_registered(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::registered(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_registered(&mut self, v: Registered) {
        self.value = ::std::option::Option::Some(Message_oneof_value::registered(v))
    }

    // Mutable pointer to the field.
    pub fn mut_registered(&mut self) -> &mut Registered {
        if let ::std::option::Option::Some(Message_oneof_value::registered(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Message_oneof_value::registered(Registered::new()));
        }
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::registered(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_registered(&mut self) -> Registered {
        if self.has_registered() {
            match self.value.take() {
                ::std::option::Option::Some(Message_oneof_value::registered(v)) => v,
                _ => panic!(),
            }
        } else {
            Registered::new()
        }
    }

    pub fn get_registered(&self) -> &Registered {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::registered(ref v)) => v,
            _ => Registered::default_instance(),
        }
    }

    // .Rejected rejected = 4;

    pub fn clear_rejected(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_rejected(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::rejected(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rejected(&mut self, v: Rejected) {
        self.value = ::std::option::Option::Some(Message_oneof_value::rejected(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rejected(&mut self) -> &mut Rejected {
        if let ::std::option::Option::Some(Message_oneof_value::rejected(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Message_oneof_value::rejected(Rejected::new()));
        }
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::rejected(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rejected(&mut self) -> Rejected {
        if self.has_rejected() {
            match self.value.take() {
                ::std::option::Option::Some(Message_oneof_value::rejected(v)) => v,
                _ => panic!(),
            }
        } else {
            Rejected::new()
        }
    }

    pub fn get_rejected(&self) -> &Rejected {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::rejected(ref v)) => v,
            _ => Rejected::default_instance(),
        }
    }

    // .NodeTree node_tree = 5;

    pub fn clear_node_tree(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_node_tree(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::node_tree(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_node_tree(&mut self, v: NodeTree) {
        self.value = ::std::option::Option::Some(Message_oneof_value::node_tree(v))
    }

    // Mutable pointer to the field.
    pub fn mut_node_tree(&mut self) -> &mut NodeTree {
        if let ::std::option::Option::Some(Message_oneof_value::node_tree(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Message_oneof_value::node_tree(NodeTree::new()));
        }
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::node_tree(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_node_tree(&mut self) -> NodeTree {
        if self.has_node_tree() {
            match self.value.take() {
                ::std::option::Option::Some(Message_oneof_value::node_tree(v)) => v,
                _ => panic!(),
            }
        } else {
            NodeTree::new()
        }
    }

    pub fn get_node_tree(&self) -> &NodeTree {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::node_tree(ref v)) => v,
            _ => NodeTree::default_instance(),
        }
    }

    // .NodeUpdate node_update = 6;

    pub fn clear_node_update(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_node_update(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::node_update(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_node_update(&mut self, v: NodeUpdate) {
        self.value = ::std::option::Option::Some(Message_oneof_value::node_update(v))
    }

    // Mutable pointer to the field.
    pub fn mut_node_update(&mut self) -> &mut NodeUpdate {
        if let ::std::option::Option::Some(Message_oneof_value::node_update(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Message_oneof_value::node_update(NodeUpdate::new()));
        }
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::node_update(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_node_update(&mut self) -> NodeUpdate {
        if self.has_node_update() {
            match self.value.take() {
                ::std::option::Option::Some(Message_oneof_value::node_update(v)) => v,
                _ => panic!(),
            }
        } else {
            NodeUpdate::new()
        }
    }

    pub fn get_node_update(&self) -> &NodeUpdate {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::node_update(ref v)) => v,
            _ => NodeUpdate::default_instance(),
        }
    }

    // .ClipboardUpdate clipboard_update = 7;

    pub fn clear_clipboard_update(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_clipboard_update(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::clipboard_update(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_clipboard_update(&mut self, v: ClipboardUpdate) {
        self.value = ::std::option::Option::Some(Message_oneof_value::clipboard_update(v))
    }

    // Mutable pointer to the field.
    pub fn mut_clipboard_update(&mut self) -> &mut ClipboardUpdate {
        if let ::std::option::Option::Some(Message_oneof_value::clipboard_update(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Message_oneof_value::clipboard_update(ClipboardUpdate::new()));
        }
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::clipboard_update(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_clipboard_update(&mut self) -> ClipboardUpdate {
        if self.has_clipboard_update() {
            match self.value.take() {
                ::std::option::Option::Some(Message_oneof_value::clipboard_update(v)) => v,
                _ => panic!(),
            }
        } else {
            ClipboardUpdate::new()
        }
    }

    pub fn get_clipboard_update(&self) -> &ClipboardUpdate {
        match self.value {
            ::std::option::Option::Some(Message_oneof_value::clipboard_update(ref v)) => v,
            _ => ClipboardUpdate::default_instance(),
        }
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        if let Some(Message_oneof_value::hello(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_value::registered(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_value::rejected(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_value::node_tree(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_value::node_update(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_value::clipboard_update(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Message_oneof_value::hello(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Message_oneof_value::registered(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Message_oneof_value::rejected(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Message_oneof_value::node_tree(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Message_oneof_value::node_update(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Message_oneof_value::clipboard_update(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != Message_MessageType::HELLO {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Message_oneof_value::hello(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_value::registered(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_value::rejected(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_value::node_tree(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_value::node_update(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_value::clipboard_update(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Message_MessageType::HELLO {
            os.write_enum(1, self.field_type.value())?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Message_oneof_value::hello(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_value::registered(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_value::rejected(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_value::node_tree(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_value::node_update(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_value::clipboard_update(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Message_MessageType>>(
                    "type",
                    Message::get_field_type_for_reflect,
                    Message::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Hello>(
                    "hello",
                    Message::has_hello,
                    Message::get_hello,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Registered>(
                    "registered",
                    Message::has_registered,
                    Message::get_registered,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Rejected>(
                    "rejected",
                    Message::has_rejected,
                    Message::get_rejected,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, NodeTree>(
                    "node_tree",
                    Message::has_node_tree,
                    Message::get_node_tree,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, NodeUpdate>(
                    "node_update",
                    Message::has_node_update,
                    Message::get_node_update,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ClipboardUpdate>(
                    "clipboard_update",
                    Message::has_clipboard_update,
                    Message::get_clipboard_update,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_hello();
        self.clear_registered();
        self.clear_rejected();
        self.clear_node_tree();
        self.clear_node_update();
        self.clear_clipboard_update();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_MessageType {
    HELLO = 0,
    REGISTERED = 1,
    REJECTED = 2,
    NODE_TREE = 3,
    NODE_UPDATE = 4,
    CLIPBOARD_UPDATE = 5,
}

impl ::protobuf::ProtobufEnum for Message_MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_MessageType> {
        match value {
            0 => ::std::option::Option::Some(Message_MessageType::HELLO),
            1 => ::std::option::Option::Some(Message_MessageType::REGISTERED),
            2 => ::std::option::Option::Some(Message_MessageType::REJECTED),
            3 => ::std::option::Option::Some(Message_MessageType::NODE_TREE),
            4 => ::std::option::Option::Some(Message_MessageType::NODE_UPDATE),
            5 => ::std::option::Option::Some(Message_MessageType::CLIPBOARD_UPDATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_MessageType] = &[
            Message_MessageType::HELLO,
            Message_MessageType::REGISTERED,
            Message_MessageType::REJECTED,
            Message_MessageType::NODE_TREE,
            Message_MessageType::NODE_UPDATE,
            Message_MessageType::CLIPBOARD_UPDATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Message_MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_MessageType {
}

impl ::std::default::Default for Message_MessageType {
    fn default() -> Self {
        Message_MessageType::HELLO
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClipboardUpdate {
    // message fields
    pub contents: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClipboardUpdate {}

impl ClipboardUpdate {
    pub fn new() -> ClipboardUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClipboardUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ClipboardUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClipboardUpdate,
        };
        unsafe {
            instance.get(ClipboardUpdate::new)
        }
    }

    // bytes contents = 1;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: ::std::vec::Vec<u8>) {
        self.contents = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.contents
    }

    // Take field
    pub fn take_contents(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.contents, ::std::vec::Vec::new())
    }

    pub fn get_contents(&self) -> &[u8] {
        &self.contents
    }

    fn get_contents_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.contents
    }

    fn mut_contents_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.contents
    }
}

impl ::protobuf::Message for ClipboardUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.contents)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.contents.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.contents);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.contents.is_empty() {
            os.write_bytes(1, &self.contents)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClipboardUpdate {
    fn new() -> ClipboardUpdate {
        ClipboardUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClipboardUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "contents",
                    ClipboardUpdate::get_contents_for_reflect,
                    ClipboardUpdate::mut_contents_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClipboardUpdate>(
                    "ClipboardUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClipboardUpdate {
    fn clear(&mut self) {
        self.clear_contents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClipboardUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClipboardUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Hello {
    // message fields
    pub version: u32,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Hello {}

impl Hello {
    pub fn new() -> Hello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Hello {
        static mut instance: ::protobuf::lazy::Lazy<Hello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Hello,
        };
        unsafe {
            instance.get(Hello::new)
        }
    }

    // uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for Hello {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_uint32(1, self.version)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Hello {
    fn new() -> Hello {
        Hello::new()
    }

    fn descriptor_static(_: ::std::option::Option<Hello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Hello::get_version_for_reflect,
                    Hello::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Hello::get_name_for_reflect,
                    Hello::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Hello>(
                    "Hello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Hello {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Hello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Hello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Registered {
    // message fields
    pub node_id: u32,
    pub num_nodes: u32,
    pub tree: ::protobuf::SingularPtrField<NodeTree>,
    pub clipboard: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Registered {}

impl Registered {
    pub fn new() -> Registered {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Registered {
        static mut instance: ::protobuf::lazy::Lazy<Registered> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Registered,
        };
        unsafe {
            instance.get(Registered::new)
        }
    }

    // uint32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: u32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &u32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.node_id
    }

    // uint32 num_nodes = 2;

    pub fn clear_num_nodes(&mut self) {
        self.num_nodes = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_nodes(&mut self, v: u32) {
        self.num_nodes = v;
    }

    pub fn get_num_nodes(&self) -> u32 {
        self.num_nodes
    }

    fn get_num_nodes_for_reflect(&self) -> &u32 {
        &self.num_nodes
    }

    fn mut_num_nodes_for_reflect(&mut self) -> &mut u32 {
        &mut self.num_nodes
    }

    // .NodeTree tree = 3;

    pub fn clear_tree(&mut self) {
        self.tree.clear();
    }

    pub fn has_tree(&self) -> bool {
        self.tree.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tree(&mut self, v: NodeTree) {
        self.tree = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree(&mut self) -> &mut NodeTree {
        if self.tree.is_none() {
            self.tree.set_default();
        }
        self.tree.as_mut().unwrap()
    }

    // Take field
    pub fn take_tree(&mut self) -> NodeTree {
        self.tree.take().unwrap_or_else(|| NodeTree::new())
    }

    pub fn get_tree(&self) -> &NodeTree {
        self.tree.as_ref().unwrap_or_else(|| NodeTree::default_instance())
    }

    fn get_tree_for_reflect(&self) -> &::protobuf::SingularPtrField<NodeTree> {
        &self.tree
    }

    fn mut_tree_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<NodeTree> {
        &mut self.tree
    }

    // bytes clipboard = 4;

    pub fn clear_clipboard(&mut self) {
        self.clipboard.clear();
    }

    // Param is passed by value, moved
    pub fn set_clipboard(&mut self, v: ::std::vec::Vec<u8>) {
        self.clipboard = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clipboard(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.clipboard
    }

    // Take field
    pub fn take_clipboard(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.clipboard, ::std::vec::Vec::new())
    }

    pub fn get_clipboard(&self) -> &[u8] {
        &self.clipboard
    }

    fn get_clipboard_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.clipboard
    }

    fn mut_clipboard_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.clipboard
    }
}

impl ::protobuf::Message for Registered {
    fn is_initialized(&self) -> bool {
        for v in &self.tree {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.node_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_nodes = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tree)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.clipboard)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_nodes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.num_nodes, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tree.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.clipboard.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.clipboard);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_uint32(1, self.node_id)?;
        }
        if self.num_nodes != 0 {
            os.write_uint32(2, self.num_nodes)?;
        }
        if let Some(ref v) = self.tree.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.clipboard.is_empty() {
            os.write_bytes(4, &self.clipboard)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Registered {
    fn new() -> Registered {
        Registered::new()
    }

    fn descriptor_static(_: ::std::option::Option<Registered>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_id",
                    Registered::get_node_id_for_reflect,
                    Registered::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_nodes",
                    Registered::get_num_nodes_for_reflect,
                    Registered::mut_num_nodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NodeTree>>(
                    "tree",
                    Registered::get_tree_for_reflect,
                    Registered::mut_tree_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "clipboard",
                    Registered::get_clipboard_for_reflect,
                    Registered::mut_clipboard_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Registered>(
                    "Registered",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Registered {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_num_nodes();
        self.clear_tree();
        self.clear_clipboard();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Registered {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Registered {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeTree {
    // message fields
    pub nodes: ::std::collections::HashMap<u32, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeTree {}

impl NodeTree {
    pub fn new() -> NodeTree {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeTree {
        static mut instance: ::protobuf::lazy::Lazy<NodeTree> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeTree,
        };
        unsafe {
            instance.get(NodeTree::new)
        }
    }

    // repeated .NodeTree.NodesEntry nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::std::collections::HashMap<u32, ::std::string::String>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::std::collections::HashMap<u32, ::std::string::String> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::std::collections::HashMap<u32, ::std::string::String> {
        ::std::mem::replace(&mut self.nodes, ::std::collections::HashMap::new())
    }

    pub fn get_nodes(&self) -> &::std::collections::HashMap<u32, ::std::string::String> {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::std::collections::HashMap<u32, ::std::string::String> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::std::collections::HashMap<u32, ::std::string::String> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for NodeTree {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.nodes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeString>(1, &self.nodes);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeString>(1, &self.nodes, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeTree {
    fn new() -> NodeTree {
        NodeTree::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeTree>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeString>(
                    "nodes",
                    NodeTree::get_nodes_for_reflect,
                    NodeTree::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeTree>(
                    "NodeTree",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeTree {
    fn clear(&mut self) {
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeTree {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeTree {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeUpdate {
    // message fields
    pub field_type: NodeUpdate_UpdateType,
    pub node_id: u32,
    pub node_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeUpdate {}

impl NodeUpdate {
    pub fn new() -> NodeUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeUpdate {
        static mut instance: ::protobuf::lazy::Lazy<NodeUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeUpdate,
        };
        unsafe {
            instance.get(NodeUpdate::new)
        }
    }

    // .NodeUpdate.UpdateType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = NodeUpdate_UpdateType::ADDED;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: NodeUpdate_UpdateType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> NodeUpdate_UpdateType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &NodeUpdate_UpdateType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut NodeUpdate_UpdateType {
        &mut self.field_type
    }

    // uint32 node_id = 2;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: u32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &u32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.node_id
    }

    // string node_name = 3;

    pub fn clear_node_name(&mut self) {
        self.node_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_name(&mut self, v: ::std::string::String) {
        self.node_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_name(&mut self) -> &mut ::std::string::String {
        &mut self.node_name
    }

    // Take field
    pub fn take_node_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node_name, ::std::string::String::new())
    }

    pub fn get_node_name(&self) -> &str {
        &self.node_name
    }

    fn get_node_name_for_reflect(&self) -> &::std::string::String {
        &self.node_name
    }

    fn mut_node_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.node_name
    }
}

impl ::protobuf::Message for NodeUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.node_id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != NodeUpdate_UpdateType::ADDED {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.node_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.node_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != NodeUpdate_UpdateType::ADDED {
            os.write_enum(1, self.field_type.value())?;
        }
        if self.node_id != 0 {
            os.write_uint32(2, self.node_id)?;
        }
        if !self.node_name.is_empty() {
            os.write_string(3, &self.node_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeUpdate {
    fn new() -> NodeUpdate {
        NodeUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NodeUpdate_UpdateType>>(
                    "type",
                    NodeUpdate::get_field_type_for_reflect,
                    NodeUpdate::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_id",
                    NodeUpdate::get_node_id_for_reflect,
                    NodeUpdate::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_name",
                    NodeUpdate::get_node_name_for_reflect,
                    NodeUpdate::mut_node_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeUpdate>(
                    "NodeUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeUpdate {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_node_id();
        self.clear_node_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NodeUpdate_UpdateType {
    ADDED = 0,
    REMOVED = 1,
}

impl ::protobuf::ProtobufEnum for NodeUpdate_UpdateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NodeUpdate_UpdateType> {
        match value {
            0 => ::std::option::Option::Some(NodeUpdate_UpdateType::ADDED),
            1 => ::std::option::Option::Some(NodeUpdate_UpdateType::REMOVED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NodeUpdate_UpdateType] = &[
            NodeUpdate_UpdateType::ADDED,
            NodeUpdate_UpdateType::REMOVED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NodeUpdate_UpdateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NodeUpdate_UpdateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NodeUpdate_UpdateType {
}

impl ::std::default::Default for NodeUpdate_UpdateType {
    fn default() -> Self {
        NodeUpdate_UpdateType::ADDED
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeUpdate_UpdateType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Rejected {
    // message fields
    pub reason: Rejected_RejectionReason,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Rejected {}

impl Rejected {
    pub fn new() -> Rejected {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Rejected {
        static mut instance: ::protobuf::lazy::Lazy<Rejected> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Rejected,
        };
        unsafe {
            instance.get(Rejected::new)
        }
    }

    // .Rejected.RejectionReason reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason = Rejected_RejectionReason::BAD_VERSION;
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: Rejected_RejectionReason) {
        self.reason = v;
    }

    pub fn get_reason(&self) -> Rejected_RejectionReason {
        self.reason
    }

    fn get_reason_for_reflect(&self) -> &Rejected_RejectionReason {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut Rejected_RejectionReason {
        &mut self.reason
    }
}

impl ::protobuf::Message for Rejected {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.reason = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.reason != Rejected_RejectionReason::BAD_VERSION {
            my_size += ::protobuf::rt::enum_size(1, self.reason);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.reason != Rejected_RejectionReason::BAD_VERSION {
            os.write_enum(1, self.reason.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Rejected {
    fn new() -> Rejected {
        Rejected::new()
    }

    fn descriptor_static(_: ::std::option::Option<Rejected>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Rejected_RejectionReason>>(
                    "reason",
                    Rejected::get_reason_for_reflect,
                    Rejected::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Rejected>(
                    "Rejected",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Rejected {
    fn clear(&mut self) {
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Rejected {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Rejected {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Rejected_RejectionReason {
    BAD_VERSION = 0,
    BAD_NAME = 1,
    BAD_MESSAGE = 2,
}

impl ::protobuf::ProtobufEnum for Rejected_RejectionReason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Rejected_RejectionReason> {
        match value {
            0 => ::std::option::Option::Some(Rejected_RejectionReason::BAD_VERSION),
            1 => ::std::option::Option::Some(Rejected_RejectionReason::BAD_NAME),
            2 => ::std::option::Option::Some(Rejected_RejectionReason::BAD_MESSAGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Rejected_RejectionReason] = &[
            Rejected_RejectionReason::BAD_VERSION,
            Rejected_RejectionReason::BAD_NAME,
            Rejected_RejectionReason::BAD_MESSAGE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Rejected_RejectionReason>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Rejected_RejectionReason", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Rejected_RejectionReason {
}

impl ::std::default::Default for Rejected_RejectionReason {
    fn default() -> Self {
        Rejected_RejectionReason::BAD_VERSION
    }
}

impl ::protobuf::reflect::ProtobufValue for Rejected_RejectionReason {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rpackets.proto\"\xbb\x03\n\x07Message\x12(\n\x04type\x18\x01\x20\x01(\
    \x0e2\x14.Message.MessageTypeR\x04type\x12\x1e\n\x05hello\x18\x02\x20\
    \x01(\x0b2\x06.HelloH\0R\x05hello\x12-\n\nregistered\x18\x03\x20\x01(\
    \x0b2\x0b.RegisteredH\0R\nregistered\x12'\n\x08rejected\x18\x04\x20\x01(\
    \x0b2\t.RejectedH\0R\x08rejected\x12(\n\tnode_tree\x18\x05\x20\x01(\x0b2\
    \t.NodeTreeH\0R\x08nodeTree\x12.\n\x0bnode_update\x18\x06\x20\x01(\x0b2\
    \x0b.NodeUpdateH\0R\nnodeUpdate\x12=\n\x10clipboard_update\x18\x07\x20\
    \x01(\x0b2\x10.ClipboardUpdateH\0R\x0fclipboardUpdate\"l\n\x0bMessageTyp\
    e\x12\t\n\x05HELLO\x10\0\x12\x0e\n\nREGISTERED\x10\x01\x12\x0c\n\x08REJE\
    CTED\x10\x02\x12\r\n\tNODE_TREE\x10\x03\x12\x0f\n\x0bNODE_UPDATE\x10\x04\
    \x12\x14\n\x10CLIPBOARD_UPDATE\x10\x05B\x07\n\x05value\"-\n\x0fClipboard\
    Update\x12\x1a\n\x08contents\x18\x01\x20\x01(\x0cR\x08contents\"5\n\x05H\
    ello\x12\x18\n\x07version\x18\x01\x20\x01(\rR\x07version\x12\x12\n\x04na\
    me\x18\x02\x20\x01(\tR\x04name\"\x7f\n\nRegistered\x12\x17\n\x07node_id\
    \x18\x01\x20\x01(\rR\x06nodeId\x12\x1b\n\tnum_nodes\x18\x02\x20\x01(\rR\
    \x08numNodes\x12\x1d\n\x04tree\x18\x03\x20\x01(\x0b2\t.NodeTreeR\x04tree\
    \x12\x1c\n\tclipboard\x18\x04\x20\x01(\x0cR\tclipboard\"p\n\x08NodeTree\
    \x12*\n\x05nodes\x18\x01\x20\x03(\x0b2\x14.NodeTree.NodesEntryR\x05nodes\
    \x1a8\n\nNodesEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\
    \n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"\x94\x01\n\nNodeUpda\
    te\x12*\n\x04type\x18\x01\x20\x01(\x0e2\x16.NodeUpdate.UpdateTypeR\x04ty\
    pe\x12\x17\n\x07node_id\x18\x02\x20\x01(\rR\x06nodeId\x12\x1b\n\tnode_na\
    me\x18\x03\x20\x01(\tR\x08nodeName\"$\n\nUpdateType\x12\t\n\x05ADDED\x10\
    \0\x12\x0b\n\x07REMOVED\x10\x01\"\x80\x01\n\x08Rejected\x121\n\x06reason\
    \x18\x01\x20\x01(\x0e2\x19.Rejected.RejectionReasonR\x06reason\"A\n\x0fR\
    ejectionReason\x12\x0f\n\x0bBAD_VERSION\x10\0\x12\x0c\n\x08BAD_NAME\x10\
    \x01\x12\x0f\n\x0bBAD_MESSAGE\x10\x02b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
