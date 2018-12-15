
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use glutin::VirtualKeyCode::*;
use glutin::MouseButton as MB;

use entities::position::PosMarker;
use input::Handler;
use input::KeyCode as KC;
use input::KeyCodes as KCS;
use terrain::World;

pub struct Mob {
  pub name: String,
  pub entity: String,
  pub pos: Arc<Mutex<PosMarker>>,
  pub speed: f32,
  pub turn: f32,
  pub stats: HashMap<String, f32>,
}

impl Mob {
  pub fn new(entity: &str, pos: Arc<Mutex<PosMarker>>) -> Self {
    Mob {
      name: "".to_string(),
      entity: entity.to_string(),
      pos: pos,
      speed: 20_f32,
      turn: 180_f32,
      stats: HashMap::new(),
    }
  }
  pub fn move_mob(&mut self, handler: &mut Handler, world_arc: Arc<Mutex<World>>) -> &Self {
    let rate = handler.timer.delta;
    if rate > 0.07 { return self; }
    let (mx, my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_f64, 0_f64),
    };
    let mut marker = self.pos.lock().unwrap();
    marker.prep(world_arc.clone());
    if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { marker.forward( self.speed, rate, world_arc.clone()); } // Up
    if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { marker.forward(-self.speed, rate, world_arc.clone()); } // Down
    if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { marker.inc_rot(0.0, self.turn * rate, 0.0); } // Left
    if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { marker.inc_rot(0.0,-self.turn * rate, 0.0); } // Right
    if handler.read_kb_single(KC::new(Space))              { marker.jump() } // Jump
    if handler.read_mouse_single(MB::Left)                 { println!("mouse x: {} y: {}", mx, my); } // Fire/Select
    marker.move_to_new_pos(rate);
    self
  }
}
