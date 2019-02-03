pub mod mobs;
pub mod position;

pub use entities::mobs::Mob;
pub use entities::position::PosMarker;

use std::sync::{Arc, Mutex};

pub struct Entity {
  pub name: String,
  pub model: String,
  pub material: String,
  pub instances: Vec<EntityInstance>,
}

impl Entity {
  pub fn new(name: &str, model: &str, material: &str) -> Self {
    Entity {
      name: name.to_string(),
      model: model.to_string(),
      material: material.to_string(),
      instances: Vec::new(),
    }
  }
  pub fn new_instance(&mut self) -> &mut EntityInstance {
    let id = self.instances.len() as u32;
    self.instances.push(EntityInstance::new(id));
    self.instances.get_mut(id as usize).unwrap()
  }
  pub fn first(&self) -> &EntityInstance {
    if self.instances.len() == 0 { panic!("No instances of Entity<{}>", &self.name) }
    &self.instances[0]
  }
  pub fn first_mut(&mut self) -> &mut EntityInstance {
    if self.instances.len() == 0 { panic!("No instances of Entity<{}>", &self.name) }
    self.instances.get_mut(0).unwrap()
  }
}

pub struct EntityInstance {
  pub id: u32,
  pub marker: Arc<Mutex<PosMarker>>,
}
impl EntityInstance {
  pub fn new(id: u32) -> Self {
    EntityInstance {
      id: id,
      marker: Arc::new(Mutex::new(PosMarker::new())),
    }
  }
  pub fn create_mob(&self, name: &str) -> Mob {
    // println!("creating mob");
    Mob::new(name, self.marker.clone())
  }
  pub fn set_pos(&self, x: f32, y: f32, z: f32) {
    let marker = self.marker.clone();
    let mut marker = marker.lock().unwrap();
    marker.pos.from_f32(x, y, z);
  }
}
