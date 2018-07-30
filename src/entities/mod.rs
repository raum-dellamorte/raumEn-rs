pub mod mobs;
pub mod position;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use entities::mobs::Mob;
use entities::position::PosMarker;
use model::Model;
use model::loader::Loader;

pub struct Entity {
  pub model_name: String,
  pub name: String,
  pub marker: Arc<Mutex<PosMarker>>,
}

impl Entity {
  pub fn new(model_name: &str, name: &str) -> Self {
    Entity {
      model_name: model_name.to_string(),
      name: name.to_string(),
      marker: Arc::new(Mutex::new(PosMarker::new())),
    }
  }
  pub fn create_mob(&self) -> Mob {
    println!("creating mob");
    Mob::new(&self.model_name, &self.name, self.marker.clone())
  }
  pub fn set_pos(&self, x: f32, y: f32, z: f32) {
    let marker = self.marker.clone();
    let mut marker = marker.lock().unwrap();
    marker.pos.from_f32(x, y, z);
  }
}

pub struct Entities {
  loader: Arc<Mutex<Loader>>,
  names: HashSet<String>,
  models: HashMap<String, Arc<Mutex<Model>>>,
  entities: HashMap<String, Arc<Mutex< Vec<Arc<Mutex<Entity>>> >> >,
  key: String,
}

impl Entities {
  pub fn new(loader: Arc<Mutex<Loader>>) -> Self {
    Entities {
      loader: loader,
      names: HashSet::new(),
      models: HashMap::new(),
      entities: HashMap::new(),
      key: String::new(),
    }
  }
  pub fn keys(&self) -> Vec<String> {
    self.names.clone().into_iter().collect::<Vec<String>>()
  }
  pub fn set_key(&mut self, key: &str) -> &mut Self {
    self.key = key.to_string();
    self
  }
  pub fn has_key(&self, key: &str) -> bool {
    self.names.contains(key)
  }
  pub fn model(&self) -> Arc<Mutex<Model>> {
    match self.models.get(&self.key) {
      Some(model) => { model.clone() }
      _ => panic!("No model: {}", self.key)
    }
  }
  pub fn new_model(&mut self, name: &str) -> &mut Self {
    let mut model = Model::new(name);
    let loader = self.loader.clone();
    let mut loader = loader.lock().unwrap();
    model.init_with_texture(&mut loader);
    self.names.insert(name.to_string());
    self.models.insert(name.to_string(), Arc::new(Mutex::new(model)));
    self.entities.insert(name.to_string(), Arc::new(Mutex::new(Vec::new())));
    self.set_key(name)
  }
  pub fn new_entity(&mut self, name: &str) -> &mut Self {
    if !self.models.contains_key(&self.key) {
      panic!("Need to add new model called {} before new entity.", &self.key) }
    let ent_names = self.entity_names();
    if ent_names.contains(name) {
      panic!("Entity name not unique: {}", name) }
    let entity = Arc::new(Mutex::new(Entity::new(&self.key, name)));
    if self.entities.contains_key(&self.key) {
      let ents = self.entities().clone();
      let mut ents = ents.lock().unwrap();
      ents.push(entity);
    } else {
      self.entities.insert(self.key.clone(), Arc::new(Mutex::new(vec![entity])));
    }
    self
  }
  pub fn new_entities(&mut self, names: &[&str]) -> &mut Self {
    for name in names {
      println!("attempting to add Entity {} {}", self.key, name);
      self.new_entity(name);
    }
    self
  }
  pub fn get_entity(&mut self, model_name: &str, name: &str) -> Arc<Mutex<Entity>> {
    self.set_key(model_name);
    let ents = self.entities();
    for ent_arc in ents.lock().unwrap().iter() {
      let ent = ent_arc.lock().unwrap();
      if ent.name == *name { return ent_arc.clone() }
    }
    panic!("No Entity {} found for Model {}", name, model_name)
  }
  pub fn entities(&mut self) -> Arc<Mutex<Vec<Arc<Mutex<Entity>>>>> {
    if !self.entities.contains_key(&self.key) {
      panic!("Tried to list Entities for uninitiated Model key: {}", &self.key) }
    (*self.entities.get(&self.key).unwrap()).clone()
  }
  pub fn entity_names(&mut self) -> HashSet<String> {
    let mut out = HashSet::new();
    let ents = self.entities().clone();
    for ent_arc in ents.lock().unwrap().iter() {
      let ent = ent_arc.lock().unwrap();
      out.insert(ent.name.clone());
    }
    out
  }
}
