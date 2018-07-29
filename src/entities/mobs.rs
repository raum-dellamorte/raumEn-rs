
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use glutin::VirtualKeyCode::*;
use glutin::MouseButton as MB;

use entities::position::PosMarker;
use input::Handler;
use input::KeyCode as KC;
use input::KeyCodes as KCS;

pub struct Mob {
  pub model_name: String,
  pub name: String,
  pub pos: Arc<Mutex<PosMarker>>,
  pub speed: f32,
  pub stats: HashMap<String, f32>,
}

impl Mob {
  pub fn new(model_name: &str, name: &str, pos: Arc<Mutex<PosMarker>>) -> Self {
    Mob {
      model_name: model_name.to_string(),
      name: name.to_string(),
      pos: pos,
      speed: 20_f32,
      stats: HashMap::new(),
    }
  }
  pub fn move_mob(&mut self, handler: &mut Handler, rate: f32) -> &Self {
    let (mx, my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_f64, 0_f64),
    };
    let mut marker = self.pos.lock().unwrap();
    if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { marker.pos.z += self.speed * rate; } // Up
    if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { marker.pos.x += self.speed * rate; } // Left
    if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { marker.pos.z -= self.speed * rate; } // Down
    if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { marker.pos.x -= self.speed * rate; } // Right
    if handler.read_kb_single(KC::new(Space))              {} // Jump
    if handler.read_mouse_single(MB::Left)                 { println!("mouse x: {} y: {}", mx, my); } // Fire/Select
    self
  }
}
