use std::collections::HashMap;
use glutin::Event;
use glutin::Event::KeyboardInput as KB;
use glutin::ElementState::{Pressed, Released};
use glutin::VirtualKeyCode::*;

pub struct Handler<'a> {
  pub map: HashMap<&'a str, bool>,
  pub cursor_pos: Option<(i32, i32)>,
}

impl<'a> Handler<'a> {
  pub fn new() -> Self {
    Handler { map: HashMap::new(), cursor_pos: None }
  }
  
  pub fn event(&mut self, event: &Event) {
    match event {
      &Event::MouseMoved(x,y) => { self.cursor_pos = Some((x,y)) },
      &KB(Pressed,  _, Some(Space)) => { self.map.insert("Space", true); },  // Pressed Space
      &KB(Released, _, Some(Space)) => { self.map.insert("Space", false); }, // Released Space
      &KB(Pressed,  _, Some(Down))  => { self.map.insert("Down", true); },  // Pressed Down
      &KB(Released, _, Some(Down))  => { self.map.insert("Down", false); }, // Released Down
      &KB(Pressed,  _, Some(A)) => { self.map.insert("A", true); },  // Pressed A
      &KB(Released, _, Some(A)) => { self.map.insert("A", false); }, // Released A
      &KB(Pressed,  _, Some(D)) => { self.map.insert("D", true); },  // Pressed D
      &KB(Released, _, Some(D)) => { self.map.insert("D", false); }, // Released D
      &KB(Pressed,  _, Some(W)) => { self.map.insert("W", true); },  // Pressed W
      &KB(Released, _, Some(W)) => { self.map.insert("W", false); }, // Released W
      &KB(Pressed,  _, Some(S)) => { self.map.insert("S", true); },  // Pressed S
      &KB(Released, _, Some(S)) => { self.map.insert("S", false); }, // Released S
      _ => {}
    }
  }
  
  pub fn read_single(&mut self, key: &'a str) -> bool {
    match self.map.insert(key, false) {
      Some(tf) => { return tf; },
      None     => { return false; },
    }
  }
  
  pub fn read_multi(&self, key: &'a str) -> bool {
    match self.map.get(key) {
      Some(&tf) => { return tf; },
      None      => { return false; },
    }
  }
}
