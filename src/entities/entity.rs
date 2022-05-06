
use entities::{Mob, PosMarker};
use util::{Vector3f, Rc, RefCell, };

pub struct Entity {
  pub name: String,
  pub model: String,
  pub material: String,
  pub instances: Vec<Rc<RefCell<EntityInstance>>>,
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
  // pub fn handler_do<F>(&mut self, f: F)
  //   where F: Fn(&mut Handler) -> ()
  // {
  //   let mut h = self.take_handler();
  //   f(&mut h);
  //   self.return_handler(h);
  // }
  pub fn new_instance_do<F>(&mut self, f: F) where F: Fn(&mut EntityInstance) {
    let id = self.instances.len() as u32;
    let out = Rc::new(RefCell::new(EntityInstance::new(id)));
    self.instances.push(out.clone());
    f(&mut out.borrow_mut());
  }
  pub fn first(&self) -> Rc<RefCell<EntityInstance>> {
    if self.instances.is_empty() { panic!("No instances of Entity<{}>", &self.name) }
    self.instances[0].clone()
  }
}

pub struct EntityInstance {
  pub id: u32,
  pub marker: Rc<RefCell<PosMarker>>,
  pub color_id: RefCell<Vector3f<f32>>,
}
impl EntityInstance {
  pub fn new(id: u32) -> Self {
    Self {
      id,
      marker: Rc::new(RefCell::new(PosMarker::new())),
      color_id: RefCell::new(Vector3f::blank()),
    }
  }
  pub fn create_mob(&self, name: &str) -> Mob {
    // println!("creating mob");
    Mob::new(name, self.marker.clone())
  }
  pub fn set_pos(&self, x: f32, y: f32, z: f32) {
    self.marker.borrow_mut().pos.copy_from_float(x, y, z);
  }
}
