
use entities::position::PosMarker;
use glium::Display;
use model::model::Model;

pub struct Entity {
  pub marker: PosMarker,
  pub model: Model,
  pub distance: f32,
}

impl Entity {
  pub fn new(name: &str) -> Self {
    Entity {
      marker: PosMarker::new(),
      model: Model::new(name),
      distance: 0_f32,
    }
  }
  
  pub fn load_model_defaults(&mut self, display: &Display) -> &mut Self {
    self.model.load_default_mesh(display).load_default_texture(display);
    self
  }
}
