
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
  pub handler: Arc<Mutex<Handler>>,
  pub loader: Arc<Mutex<Loader>>,
  pub lights: Arc<Mutex<Lights>>,
  pub camera: Arc<Mutex<Camera>>,
}

impl GameMgr {
  pub fn new() -> Self {
    let mut lights = Lights::new();
    lights.add_light();
    lights.lights[0].pos.from_isize(0,500,-10);
    let handler = Arc::new(Mutex::new(Handler::new()));
    GameMgr {
      handler: handler.clone(),
      loader: Arc::new(Mutex::new(Loader::new())),
      lights: Arc::new(Mutex::new(lights)),
      camera: Arc::new(Mutex::new(Camera::new(handler.clone()))),
    }
  }
  pub fn clean_up(&mut self) {
    let mut loader = self.loader.lock().unwrap();
    loader.clean_up();
  }
}
