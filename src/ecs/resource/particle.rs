

use {
  std::{
    f32::consts::{ PI, },
  },
  crate::{
    util::{
      Vector3f,
    },
  }
};

#[derive(Default, Debug)]
pub struct ParticleRules {
  pub base_pos: Vector3f<f32>,
  pub base_dir: Vector3f<f32>,
  pub texture: String,
  pub tex_rows: u32,
  pub pps: f32,
  pub dir_error: f32,
  pub half_life: f32,
  pub life_error: f32,
  pub half_speed: f32,
  pub speed_error: f32,
  pub half_scale: f32,
  pub scale_error: f32,
  pub angle: f32,
  pub gravity_mult: f32,
  pub rand_rot: bool,
  pub is_directional: bool,
}
impl ParticleRules {
  pub fn set_direction(&mut self, direction: Vector3f<f32>, deviation: f32) {
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
