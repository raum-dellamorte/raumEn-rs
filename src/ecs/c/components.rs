

use {
  specs::{
    Component, VecStorage, 
  },
  crate::{
    util::{
      Vector2f, 
    },
  },
};


#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct CamDistance(pub f64);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GravPercent(pub f32);
impl Default for GravPercent {
  fn default() -> Self {
    Self(1.0)
  }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct ParticleLife {
  pub total: f64,
  pub elapsed: f64,
}
impl ParticleLife {
  pub fn inc_time(&mut self, delta: f64) {
    self.elapsed += delta;
  }
  pub fn is_alive(&self) -> bool {
    self.elapsed < self.total
  }
  pub fn set_life(&mut self, life_length: f64) {
    self.elapsed = 0.0;
    self.total = life_length;
  }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexAtlas {
  pub num_of_rows: u32,
  pub additive: bool,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexOffsets{
  pub a: Vector2f<f64>,
  pub b: Vector2f<f64>,
}

