use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
  pub registered: bool,
  pub hello_sent: bool,
  pub tree: HashMap<u32, String>,
  pub shared: Vec<u8>
}

impl State {
  pub fn update_clipboard(&self) {
    if let Ok(s) = String::from_utf8(self.shared.clone()) {
      if let Ok(mut c) = ClipboardContext::new() {
        if let Err(e) = c.set_contents(s) {
          warn!("could not set clipboard: {}", e);
        }
      }
    }
  }
}
