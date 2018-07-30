
use std::sync::{Arc, Mutex};

pub use camera::Camera;
pub use entities::Entity;
pub use entities::mobs::Mob;
pub use input::Handler;
pub use model::loader::Loader;
pub use model::Model;
pub use shader::lighting::Lights;
pub use shader::Shader;
pub use render::{RenderMgr, };

#[derive(Clone)]
pub struct GameMgr {
  pub camera: Arc<Mutex<Camera>>,
  pub handler: Arc<Mutex<Handler>>,
  pub loader: Arc<Mutex<Loader>>,
  pub lights: Arc<Mutex<Lights>>,
}

impl GameMgr {
  pub fn new() -> Self {
    let mut lights = Lights::new();
    lights.add_light();
    lights.lights[0].pos.from_isize(-50,500,50);
    GameMgr {
      camera: Arc::new(Mutex::new(Camera::new())),
      handler: Arc::new(Mutex::new(Handler::new())),
      loader: Arc::new(Mutex::new(Loader::new())),
      lights: Arc::new(Mutex::new(lights)),
    }
  }
  pub fn clean_up(&mut self) {
    let mut loader = self.loader.lock().unwrap();
    loader.clean_up();
  }
}
