
use entities::{Entity, EntityInstance};
use util::{Rc,RefCell,HashMap};

pub struct EntityMgr {
  pub entities: RefCell<HashMap<String,Rc<RefCell<Entity>>>>,
  pub instances: RefCell<HashMap<String,Vec<Rc<RefCell<EntityInstance>>>>>,
}
impl EntityMgr {
  pub fn new() -> Self {
    Self {
      entities: RefCell::new(HashMap::new()),
      instances: RefCell::new(HashMap::new()),
    }
  }
  pub fn new_entity(&self, name: &str, model: &str, material: &str) {
    let mut entities = self.entities.borrow_mut();
    let mut instances = self.instances.borrow_mut();
    if entities.contains_key(name) { panic!("Entity name not unique: {}", name) } // they should prolly have IDs instead
    let ent = Rc::new(RefCell::new(Entity::new(name, model, material)));
    entities.insert(name.to_string(), ent);
    instances.insert(name.to_string(), Vec::new());
    println!("new Entity name<{}> model<{}> material<{}>", name, model, material);
  }
  pub fn new_instance_at(&self, entity_name: &str, x: f32, y: f32, z: f32) {
    let entities = self.entities.borrow();
    if entities.get(entity_name).is_some() {
      let mut all_instances = self.instances.borrow_mut();
      if let Some(instances) = all_instances.get_mut(entity_name) {
        let id = instances.len() as u32;
        let ent = EntityInstance::new(id);
        ent.set_pos(x, y, z);
        instances.push(Rc::new(RefCell::new(ent)));
      } else { println!("EntityMgr: No Instance Vec for Entity \"{}\".", entity_name); }
    } else { println!("EntityMgr: No Entity \"{}\" to make new instance of.", entity_name); }
  }
  pub fn new_instance_do<F>(&mut self, name: &str, f: F) where F: Fn(&mut EntityInstance) {
    let mut all_instances = self.instances.borrow_mut();
    let instances = all_instances.get_mut(name).unwrap();
    let id = instances.len() as u32;
    let out = Rc::new(RefCell::new(EntityInstance::new(id)));
    instances.push(out.clone());
    f(&mut out.borrow_mut());
  }
  pub fn first(&self, name: &str) -> Rc<RefCell<EntityInstance>> {
    let mut all_instances = self.instances.borrow_mut();
    let instances = all_instances.get_mut(name).unwrap();
    if instances.len() == 0 { panic!("No instances of Entity<{}>", name) }
    instances[0].clone()
  }
}
