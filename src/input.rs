use std::collections::HashMap;
use glutin::Event;
use glutin::Event::KeyboardInput as KB;
use glutin::Event::MouseInput as Click;
use glutin::ElementState::{Pressed, Released};
use glutin::VirtualKeyCode;
use glutin::MouseButton;

pub struct Handler {
  pub kb: HashMap<VirtualKeyCode, bool>,
  pub mouse: HashMap<MouseButton, bool>,
  pub cursor_pos: Option<(i32, i32)>,
}

impl Handler {
  pub fn new() -> Self {
    Handler { kb: HashMap::new(), mouse: HashMap::new(), cursor_pos: None }
  }
  
  pub fn event(&mut self, event: &Event) {
    match event {
      &Event::MouseMoved(x,y)      => { self.cursor_pos = Some((x,y)) },
      &Click(Pressed,  mbttn)      => { self.mouse.insert(mbttn, true); },
      &Click(Released, mbttn)      => { self.mouse.insert(mbttn, false); },
      &KB(Pressed,  _, Some(bttn)) => { self.kb.insert(bttn, true); },
      &KB(Released, _, Some(bttn)) => { self.kb.insert(bttn, false); },
      _ => {}
    }
  }
  
  pub fn read_kb_single(&mut self, key: VirtualKeyCode) -> bool {
    match self.kb.insert(key, false) {
      Some(tf) => { return tf; },
      None     => { return false; },
    }
  }
  
  pub fn read_kb_multi(&self, key: VirtualKeyCode) -> bool {
    match self.kb.get(&key) {
      Some(&tf) => { return tf; },
      None      => { return false; },
    }
  }
  
  pub fn read_mouse_single(&mut self, key: MouseButton) -> bool {
    match self.mouse.insert(key, false) {
      Some(tf) => { return tf; },
      None     => { return false; },
    }
  }
  
  pub fn read_mouse_multi(&self, key: MouseButton) -> bool {
    match self.mouse.get(&key) {
      Some(&tf) => { return tf; },
      None      => { return false; },
    }
  }
}
