use std::collections::HashMap;
use entities::entity::Entity;
use input::Handler;

pub struct Mob<'a> {
  pub entity: Entity,
  pub speed: f32,
  pub stats: HashMap<&'a str, f32>,
}

impl<'a> Mob<'a> {
  pub fn new() -> Self {
    Mob {
      entity: Entity::new(),
      speed: 20_f32,
      stats: HashMap::new(),
    }
  }
  
  pub fn move_mob(&mut self, handler: &mut Handler, rate: f32) -> &Self {
    if handler.read_multi("W") { self.entity.marker.pos.z += self.speed * rate; } // Up
    if handler.read_multi("A") { self.entity.marker.pos.x += self.speed * rate; } // Left
    if handler.read_multi("S") { self.entity.marker.pos.z -= self.speed * rate; } // Down
    if handler.read_multi("D") { self.entity.marker.pos.x -= self.speed * rate; } // Right
    if handler.read_single("Space") {} // Jump
    self
  }
}
