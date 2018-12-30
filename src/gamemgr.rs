
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use Camera;
use Display;
use Entity;
use Handler;
use Material;
use Loader;
use model::{RawModel};
use shader::lighting::{Lighting, Lights};
use terrain::{World, WorldBuilder};
use text::{TextMgr, }; // RFontType, 
use texture::Texture;
use util::{Matrix4f, Vector3f};

//#[derive(Clone)]
pub struct GameMgr {
  pub handler: Option<Box<Handler>>,
  pub loader: Arc<Mutex<Loader>>,
  pub lights: Arc<Mutex<Lights>>,
  pub camera: Option<Box<Camera>>,
  pub display: Arc<Mutex<Display>>,
  pub world: Option<Box<World>>,
  pub world_builder: WorldBuilder,
  pub textmgr: Option<Arc<Mutex<TextMgr>>>,
  pub entities: Arc<Mutex<HashMap<String, Entity>>>,
  pub models: Arc<Mutex<HashMap<String, Arc<RawModel>>>>,
  pub materials: Arc<Mutex<HashMap<String, Arc<Mutex<Material>>>>>,
  pub textures: Arc<Mutex<HashMap<String, Arc<Texture>>>>,
  pub lightings: Arc<Mutex<HashMap<String, Arc<Mutex<Lighting>>>>>,
  // pub fonts: Option<Arc<Mutex<HashMap<String, RFontType>>>>,
  pub view_mat: Matrix4f,
  pub player_loc: Vector3f,
}

impl GameMgr {
  pub fn new() -> Self {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let mut lights = Lights::new();
    lights.add_light();
    lights.lights[0].pos.from_isize(0,500,-10);
    // let handler = Arc::new(Mutex::new(Handler::new()));
    let handler = Some(Box::new(Handler::new()));
    let camera = Some(Box::new(Camera::new()));
    let display = Arc::new(Mutex::new(Display::new()));
    // let ents = Entities::new(loader.clone());
    let textmgr = TextMgr::new();
    let mut world = Box::new(World::new());
    let mut builder = WorldBuilder::new();
    builder.set_landscape_weight_and_mult(0.5, 3);
    builder.gen_world(&mut world, 0.0, 0.0);
    GameMgr {
      handler: handler,
      loader: loader,
      lights: Arc::new(Mutex::new(lights)),
      camera: camera,
      display: display,
      world: Some(world),
      world_builder: builder,
      textmgr: Some(Arc::new(Mutex::new(textmgr))),
      entities: Arc::new(Mutex::new(HashMap::new())),
      models: Arc::new(Mutex::new(HashMap::new())),
      materials: Arc::new(Mutex::new(HashMap::new())),
      textures: Arc::new(Mutex::new(HashMap::new())),
      lightings: Arc::new(Mutex::new(HashMap::new())),
      // fonts: Some(Arc::new(Mutex::new(HashMap::new()))),
      view_mat: Matrix4f::new(),
      player_loc: Vector3f::blank(),
    }
  }
  pub fn update_size(self, dimensions: (u32, u32)) -> Box<Self> {
    let mut _self = Box::new(self);
    {
      let mut d = (&mut _self).display.lock().unwrap();
      d.update_size(dimensions);
    }
    let _textmgr = (&mut _self).textmgr.take().unwrap();
    {
      let mut textmgr = _textmgr.lock().unwrap();
      _self = textmgr.update_size(_self);
    }
    (&mut _self).textmgr = Some(_textmgr);
    _self
  }
  pub fn aspect_ratio(&self) -> f32 {
    let d = self.display.lock().unwrap();
    d.aspect_ratio
  }
  pub fn dimensions(&self) -> (u32, u32) {
    let d = self.display.lock().unwrap();
    d.dimensions()
  }
  pub fn take_handler(&mut self) -> Box<Handler> {
    let out = self.handler.take();
    Box::new(*out.unwrap())
  }
  pub fn return_handler(&mut self, handler: Box<Handler>) {
    self.handler = Some(handler)
  }
  // pub fn handler_do<F>(&mut self, f: F)
  //   where F: Fn(&mut Handler) -> ()
  // {
  //   let mut h = self.take_handler();
  //   f(&mut h);
  //   self.return_handler(h);
  // }
  pub fn loader_do<F>(&mut self, f: F)
    where F: Fn(&mut Loader) -> ()
  {
    let mut h = self.loader.lock().unwrap();
    f(&mut h);
  }
  pub fn lights_do<F>(&mut self, f: F)
    where F: Fn(&mut Lights) -> ()
  {
    // println!("Lights in");
    let mut h = self.lights.lock().unwrap();
    f(&mut h);
    // println!("Lights out");
  }
  pub fn take_camera(&mut self) -> Box<Camera> {
    let out = self.camera.take();
    Box::new(*out.unwrap())
  }
  pub fn return_camera(&mut self, camera: Box<Camera>) {
    self.camera = Some(camera)
  }
  // pub fn camera_do<F>(&mut self, f: F)
  //   where F: Fn(&mut Camera, &mut Handler) -> ()
  // {
  //   let mut c = self.take_camera();
  //   let mut h = self.take_handler();
  //   f(&mut c, &mut h);
  //   self.return_handler(h);
  // }
  pub fn take_world(&mut self) -> Box<World> {
    let out = self.world.take();
    Box::new(*out.unwrap())
  }
  pub fn return_world(&mut self, world: Box<World>) {
    self.world = Some(world)
  }
  pub fn gen_chunks(&mut self) {
    let mut world = self.take_world();
    self.world_builder.gen_world(&mut world, self.player_loc.x, self.player_loc.z);
    self.return_world(world);
  }
  pub fn entities_do<F>(&mut self, f: F)
    where F: Fn(&mut HashMap<String, Entity>) -> ()
  {
    let mut h = self.entities.lock().unwrap();
    f(&mut h);
  }
  pub fn create_view_matrix(&mut self) {
    let mut cam = self.take_camera();
    cam.create_view_matrix(&mut self.view_mat);
    self.return_camera(cam);
  }
  pub fn new_entity(&mut self, name: &str, model: &str, material: &str) {
    let _arc = self.entities.clone();
    let mut ents = _arc.lock().unwrap();
    if ents.contains_key(name) { panic!("Entity name not unique: {}", name) } // they should prolly have IDs instead
    let entity = Entity::new(name, model, material);
    ents.insert(name.to_string(), entity);
    // println!("new Entity name<{}> model<{}> material<{}>", name, model, material);
  }
  pub fn new_entities(&mut self, names: &[(&str, &str, &str)]) {
    for name in names {
      let (name, model, material) = name;
      self.new_entity(name, model, material);
    }
  }
  pub fn new_model(&mut self, name: &str) {
    let model = {
      let _arc = self.loader.clone();
      let mut loader = _arc.lock().unwrap();
      loader.load_to_vao(name)
    };
    let models_arc = self.models.clone();
    let mut models = models_arc.lock().unwrap();
    models.insert(name.to_string(), Arc::new(model));
  }
  pub fn new_material(&mut self, name: &str, texture: &str, lighting: &str) {
    self.new_texture(texture);
    self.new_lighting(lighting);
    let _arc = self.materials.clone();
    let mut hm = _arc.lock().unwrap();
    hm.insert(name.to_string(), Arc::new(Mutex::new(Material::new(name, texture, lighting))));
  }
  pub fn new_texture(&mut self, name: &str) {
    let texture = {
      let _arc = self.loader.clone();
      let mut loader = _arc.lock().unwrap();
      loader.load_texture(name)
    };
    let _arc = self.textures.clone();
    let mut hm = _arc.lock().unwrap();
    // println!("texture: image<{}> tex_id<{}>", name, texture.tex_id);
    hm.insert(name.to_string(), Arc::new(texture));
  }
  pub fn new_lighting(&mut self, name: &str) {
    let _arc = self.lightings.clone();
    let mut hm = _arc.lock().unwrap();
    hm.insert(name.to_string(), Arc::new(Mutex::new(Lighting::new())));
  }
  pub fn mod_entity<F>(&mut self, name: &str, f: F) 
    where F: Fn(&mut Entity) -> ()
  {
    let _arc = self.entities.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let mut ent = hm.get_mut(name).unwrap();
      f(&mut ent);
    } else { panic!("No Entity to modify: {}", name) }
  }
  pub fn mod_material<F>(&mut self, name: &str, f: F) 
    where F: Fn(&mut Material) -> ()
  {
    let _arc = self.materials.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let mut ent = hm.get_mut(name).unwrap().lock().unwrap();
      f(&mut ent);
    } else { panic!("No Entity to modify: {}", name) }
  }
  pub fn model(&self, name: &str) -> Arc<RawModel> {
    let _arc = self.models.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Model: {}", name) }
  }
  pub fn material(&self, name: &str) -> Arc<Mutex<Material>> {
    let _arc = self.materials.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Material: {}", name) }
  }
  pub fn texture(&self, name: &str) -> Arc<Texture> {
    let _arc = self.textures.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Texture: {}", name) }
  }
  pub fn lighting(&self, name: &str) -> Arc<Mutex<Lighting>> {
    let _arc = self.lightings.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Lighting: {}", name) }
  }
  pub fn clean_up(&mut self) {
    let mut loader = self.loader.lock().unwrap();
    loader.clean_up();
  }
}
