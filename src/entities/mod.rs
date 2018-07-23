pub mod mobs;
pub mod position;

use entities::position::PosMarker;
use model::Model;
use model::loader::Loader;

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
  pub fn name(&self) -> &str {
    &self.model.name
  }
  pub fn init(&mut self, loader: &mut Loader) -> &mut Self {
    self.model.init_with_texture(loader);
    self
  }
}
