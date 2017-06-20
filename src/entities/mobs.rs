use std::collections::HashMap;
use entities::entity::Entity;
use input::Handler;
use glutin::VirtualKeyCode::*;
use glutin::MouseButton;

pub struct Mob<'a> {
  pub entity: Entity,
  pub speed: f32,
  pub stats: HashMap<&'a str, f32>,
}

impl<'a> Mob<'a> {
  pub fn new(name: &str) -> Self {
    Mob {
      entity: Entity::new(name),
      speed: 20_f32,
      stats: HashMap::new(),
    }
  }
  
  pub fn move_mob(&mut self, handler: &mut Handler, rate: f32) -> &Self {
    let (mx, my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_i32, 0_i32),
    };
    if handler.read_kb_multi(W) { self.entity.marker.pos.z += self.speed * rate; } // Up
    if handler.read_kb_multi(A) { self.entity.marker.pos.x += self.speed * rate; } // Left
    if handler.read_kb_multi(S) { self.entity.marker.pos.z -= self.speed * rate; } // Down
    if handler.read_kb_multi(D) { self.entity.marker.pos.x -= self.speed * rate; } // Right
    if handler.read_kb_single(Space) {} // Jump
    if handler.read_mouse_single(MouseButton::Left) { println!("mouse x: {} y: {}", mx, my); } // Fire/Select
    self
  }
}
