

use {
  std::{
    f64::consts::{ PI, },
  },
  crate::{
    util::{
      Vector3f,
    },
  }
};

#[derive(Default, Debug)]
pub struct ParticleRules {
  pub base_pos: Vector3f<f64>,
  pub base_dir: Vector3f<f64>,
  pub texture: String,
  pub tex_rows: u32,
  pub pps: f64,
  pub dir_error: f64,
  pub half_life: f64,
  pub life_error: f64,
  pub half_speed: f64,
  pub speed_error: f64,
  pub half_scale: f64,
  pub scale_error: f64,
  pub angle: f64,
  pub gravity_mult: f64,
  pub rand_rot: bool,
  pub is_directional: bool,
}
impl ParticleRules {
  pub fn set_direction(&mut self, direction: Vector3f<f64>, deviation: f64) {
    self.base_dir = direction;
    self.dir_error = deviation * PI;
    self.is_directional = true;
  }
  pub fn randomize_rotation(&mut self) { self.rand_rot = !self.rand_rot }
  // pub fn
}

#[derive(Default, Debug)]
pub struct ParticleSystems {
  // Gonna hold the particle rules 
}
