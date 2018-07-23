
use std::collections::HashMap;
use glutin::VirtualKeyCode::*;
use glutin::MouseButton as MB;

use entities::Entity;
use model::loader::Loader;
use input::Handler;
use input::KeyCode as KC;
use input::KeyCodes as KCS;

pub struct Mob {
  pub entity: Entity,
  pub speed: f32,
  pub stats: HashMap<String, f32>,
}

impl Mob {
  pub fn new(name: &str) -> Self {
    Mob {
      entity: Entity::new(name),
      speed: 20_f32,
      stats: HashMap::new(),
    }
  }
  pub fn name(&self) -> &str {
    self.entity.name()
  }
  pub fn init(&mut self, loader: &mut Loader) -> &mut Self {
    self.entity.init(loader); self
  }
  pub fn move_mob(&mut self, handler: &mut Handler, rate: f32) -> &Self {
    let (mx, my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_f64, 0_f64),
    };
    if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { self.entity.marker.pos.z += self.speed * rate; } // Up
    if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { self.entity.marker.pos.x += self.speed * rate; } // Left
    if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { self.entity.marker.pos.z -= self.speed * rate; } // Down
    if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { self.entity.marker.pos.x -= self.speed * rate; } // Right
    if handler.read_kb_single(KC::new(Space))              {} // Jump
    if handler.read_mouse_single(MB::Left)                 { println!("mouse x: {} y: {}", mx, my); } // Fire/Select
    self
  }
}
