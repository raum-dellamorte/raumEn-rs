use std::collections::HashMap;
use entities::entity::PosMarker;
use input::Handler;

pub struct Mob<'a> {
  pub marker: PosMarker,
  pub stats: HashMap<&'a str, f32>,
}

impl<'a> Mob<'a> {
  pub fn new() -> Self {
    Mob {
      marker: PosMarker::new(),
      stats: HashMap::new(),
    }
  }
  
  pub fn move_mob(&mut self, handler: &mut Handler) -> &Self {
    if handler.read_multi("W") { self.marker.pos.z += 0.01_f32; } // Up
    if handler.read_multi("A") { self.marker.pos.x += 0.01_f32; } // Left
    if handler.read_multi("S") { self.marker.pos.z -= 0.01_f32; } // Down
    if handler.read_multi("D") { self.marker.pos.x -= 0.01_f32; } // Right
    if handler.read_single("Space") {} // Jump
    self
  }
}
