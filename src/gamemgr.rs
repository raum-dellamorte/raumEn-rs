
use std::sync::{Arc, Mutex};

pub use camera::Camera;
pub use entities::Entity;
pub use entities::Entities;
pub use entities::mobs::Mob;
pub use input::Handler;
pub use model::loader::Loader;
pub use model::Model;
pub use render::{RenderMgr, };
pub use shader::lighting::Lights;
pub use shader::Shader;
pub use terrain::World;
pub use util::rmatrix::Matrix4f;

#[derive(Clone)]
pub struct GameMgr {
  pub handler: Arc<Mutex<Handler>>,
  pub loader: Arc<Mutex<Loader>>,
  pub lights: Arc<Mutex<Lights>>,
  pub camera: Arc<Mutex<Camera>>,
  pub entities: Arc<Mutex<Entities>>,
  pub world: Arc<Mutex<World>>,
  pub view_mat: Matrix4f,
}

impl GameMgr {
  pub fn new() -> Self {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let mut lights = Lights::new();
    lights.add_light();
    lights.lights[0].pos.from_isize(0,500,-10);
    let handler = Arc::new(Mutex::new(Handler::new()));
    let ents = Entities::new(loader.clone());
    let world = World::new(loader.clone());
    GameMgr {
      handler: handler.clone(),
      loader: loader,
      lights: Arc::new(Mutex::new(lights)),
      camera: Arc::new(Mutex::new(Camera::new(handler.clone()))),
      entities: Arc::new(Mutex::new(ents)),
      world: Arc::new(Mutex::new(world)),
      view_mat: Matrix4f::new(),
    }
  }
  pub fn handler_do<F>(&mut self, f: F)
    where F: Fn(&mut Handler) -> ()
  {
    let mut h = self.handler.lock().unwrap();
    f(&mut h);
  }
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
  pub fn camera_do<F>(&mut self, f: F)
    where F: Fn(&mut Camera) -> ()
  {
    let mut h = self.camera.lock().unwrap();
    f(&mut h);
  }
  pub fn entities_do<F>(&mut self, f: F)
    where F: Fn(&mut Entities) -> ()
  {
    let mut h = self.entities.lock().unwrap();
    f(&mut h);
  }
  pub fn world_do<F>(&mut self, f: F)
    where F: Fn(&mut World) -> ()
  {
    let mut h = self.world.lock().unwrap();
    f(&mut h);
  }
  pub fn create_view_matrix(&mut self) {
    let mut cam = self.camera.lock().unwrap();
    cam.create_view_matrix(&mut self.view_mat);
  }
  pub fn clean_up(&mut self) {
    let mut loader = self.loader.lock().unwrap();
    loader.clean_up();
  }
}
