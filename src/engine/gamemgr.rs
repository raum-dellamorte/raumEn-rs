
use {
  EntityMgr, 
  Handler, 
  HUD, 
  Loader, 
  ecs::c::{
    material::*,
    Model, Texture,
    lighting::{Lights, Lighting},
  },
  text::{
    TextMgr, 
    // RFontType, 
  },
  util::{
    Matrix4f, 
    Vector3f, 
    Rc, RefCell, 
    HashMap, 
  },
};

pub struct GameMgr {
  pub handler: Option<Box<Handler>>,
  pub loader: Rc<RefCell<Loader>>,
  pub lights: Rc<RefCell<Lights>>,
  // pub world: Option<Box<World>>,
  // pub world_builder: WorldBuilder,
  pub textmgr: Option<Rc<RefCell<TextMgr>>>,
  pub hud: Rc<RefCell<HUD>>,
  pub entity_mgr: Rc<EntityMgr>,
  pub models: Rc<RefCell<HashMap<String, Rc<Model>>>>,
  pub materials: Rc<RefCell<HashMap<String, Rc<RefCell<Material>>>>>,
  pub textures: Rc<RefCell<HashMap<String, Rc<Texture>>>>,
  pub lightings: Rc<RefCell<HashMap<String, Rc<RefCell<Lighting>>>>>,
  // pub fonts: Option<Rc<RefCell<HashMap<String, RFontType>>>>,
  pub view_mat: Matrix4f,
  pub player_loc: Vector3f,
  pub quad_id: u32,
}
impl GameMgr {
  pub fn new() -> Self {
    let loader = Rc::new(RefCell::new(Loader::new()));
    let mut lights = Lights::new();
    lights.add_light();
    lights.lights[0].pos.from_isize(0,500,-10);
    // let handler = Arc::new(Mutex::new(Handler::new()));
    let handler = Some(Box::new(Handler::new()));
    // let ents = Entities::new(loader.clone());
    let textmgr = TextMgr::new();
    // let mut world = Box::new(World::new());
    // let mut builder = WorldBuilder::new();
    // builder.set_landscape_weight_and_mult(0.5, 3);
    // builder.gen_world(&mut world, 0.0, 0.0);
    let quad_vec = vec![-1.0,1.0, -1.0,-1.0, 1.0,1.0, 1.0,-1.0];
    let quad = loader.borrow_mut().load_to_vao_gui(&quad_vec);
    let hud = HUD::new(quad);
    GameMgr {
      handler: handler,
      loader: loader,
      lights: Rc::new(RefCell::new(lights)),
      // world: Some(world),
      // world_builder: builder,
      textmgr: Some(Rc::new(RefCell::new(textmgr))),
      hud: Rc::new(RefCell::new(hud)),
      entity_mgr: Rc::new(EntityMgr::new()),
      models: Rc::new(RefCell::new(HashMap::new())),
      materials: Rc::new(RefCell::new(HashMap::new())),
      textures: Rc::new(RefCell::new(HashMap::new())),
      lightings: Rc::new(RefCell::new(HashMap::new())),
      // fonts: Some(Rc::new(RefCell::new(HashMap::new()))),
      view_mat: Matrix4f::new(),
      player_loc: Vector3f::blank(),
      quad_id: quad,
    }
  }
  pub fn update_size(self, dimensions: (u32, u32)) -> Box<Self> {
    let mut _self = Box::new(self);
    _self.display.borrow_mut().update_size(dimensions);
    let _textmgr = _self.textmgr.take().unwrap();
    let mut _self = _textmgr.borrow_mut().update_size(_self);
    _self.textmgr = Some(_textmgr);
    _self
  }
  pub fn aspect_ratio(&self) -> f32 {
    self.display.borrow().aspect_ratio
  }
  pub fn dimensions(&self) -> (u32, u32) {
    let d = self.display.borrow();
    d.dimensions()
  }
  pub fn fps_and_delta(&mut self) -> (f32, f32) {
    let handler = self.take_handler();
    let fps = handler.timer.fps;
    let delta = handler.timer.delta;
    self.return_handler(handler);
    (fps, delta)
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
      where F: Fn(&mut Loader) -> () {
    let mut h = self.loader.borrow_mut();
    f(&mut h);
  }
  pub fn lights_do<F>(&mut self, f: F)
      where F: Fn(&mut Lights) -> () {
    // println!("Lights in");
    let mut h = self.lights.borrow_mut();
    f(&mut h);
    // println!("Lights out");
  }
  // pub fn take_world(&mut self) -> Box<World> {
  //   let out = self.world.take();
  //   Box::new(*out.unwrap())
  // }
  // pub fn return_world(&mut self, world: Box<World>) {
  //   self.world = Some(world)
  // }
  // pub fn gen_chunks(&mut self) {
  //   let mut world = self.take_world();
  //   self.world_builder.gen_world(&mut world, self.player_loc.x, self.player_loc.z);
  //   self.return_world(world);
  // }
  // pub fn entities_do<F>(&mut self, f: F)
  //     where F: Fn(&mut HashMap<String, Entity>) -> () {
  //   let mut h = self.entities.borrow_mut();
  //   f(&mut h);
  // }
  pub fn create_view_matrix(&mut self) {
    let mut cam = self.take_camera();
    cam.create_view_matrix(&mut self.view_mat);
    self.return_camera(cam);
  }
  pub fn new_entity(&mut self, name: &str, model: &str, material: &str) {
    self.entity_mgr.new_entity(name, model, material);
  }
  pub fn new_entities(&mut self, names: &[(&str, &str, &str)]) {
    for name in names {
      let (name, model, material) = name;
      self.new_entity(name, model, material);
    }
  }
  pub fn new_model(&mut self, name: &str) {
    let model = {
      let mut loader = self.loader.borrow_mut();
      loader.load_to_vao(name)
    };
    let mut models = self.models.borrow_mut();
    models.insert(name.to_string(), Rc::new(model));
  }
  pub fn new_material(&mut self, name: &str, texture: &str, lighting: &str) {
    self.new_texture(texture);
    self.new_lighting(lighting);
    self.materials.borrow_mut().insert(name.to_string(), Rc::new(RefCell::new(Material::new(name, texture, lighting))));
  }
  pub fn new_texture(&mut self, name: &str) {
    let texture =  self.loader.borrow_mut().load_texture(name);
    // println!("texture: image<{}> tex_id<{}>", name, texture.tex_id);
    self.textures.borrow_mut().insert(name.to_string(), Rc::new(texture));
  }
  pub fn new_lighting(&mut self, name: &str) {
    self.lightings.borrow_mut().insert(name.to_string(), Rc::new(RefCell::new(Lighting::new())));
  }
  // pub fn mod_entity<F>(&mut self, name: &str, f: F) 
  //     where F: Fn(&mut Entity) -> () {
  //   let mut hm = self.entities.borrow_mut();
  //   if hm.contains_key(name) {
  //     let mut ent = hm.get_mut(name).unwrap();
  //     f(&mut ent);
  //   } else { panic!("No Entity to modify: {}", name) }
  // }
  pub fn mod_material<F>(&mut self, name: &str, f: F) 
      where F: Fn(&mut Material) -> () {
    let mut hm = self.materials.borrow_mut();
    if hm.contains_key(name) {
      let mut ent = hm.get_mut(name).unwrap().borrow_mut();
      f(&mut ent);
    } else { panic!("No Entity to modify: {}", name) }
  }
  pub fn model(&self, name: &str) -> Rc<Model> {
    let mut hm = self.models.borrow_mut();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Model: {}", name) }
  }
  pub fn material(&self, name: &str) -> Rc<RefCell<Material>> {
    let mut hm = self.materials.borrow_mut();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Material: {}", name) }
  }
  pub fn texture(&self, name: &str) -> Rc<Texture> {
    let mut hm = self.textures.borrow_mut();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Texture: {}", name) }
  }
  pub fn lighting(&self, name: &str) -> Rc<RefCell<Lighting>> {
    let mut hm = self.lightings.borrow_mut();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Lighting: {}", name) }
  }
  pub fn clean_up(&mut self) {
    let mut loader = self.loader.borrow_mut();
    loader.clean_up();
  }
}
