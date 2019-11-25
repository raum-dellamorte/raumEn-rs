

use {
  std::{
    f32::consts::{ PI, },
  },
  specs::{*, WorldExt, },
  crate::{
    Loader,
    Model,
    util::{
      Vector3f,
      rgl::*,
    },
  }
};

#[derive(Default, Debug)]
pub struct ParticleRules {
  pub texture: String,
  pub tex_rows: u32,
  pub pps: f32,
  pub base_pos: Vector3f<f32>,
  pub base_dir: Vector3f<f32>,
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
  pub fn randomize_rotation(self) -> Self {
    let mut slf = self;
    slf.rand_rot = !slf.rand_rot;
    slf
  }
  pub fn set_direction(self, direction: Vector3f<f32>, deviation: f32) -> Self {
    let mut slf = self;
    slf.base_dir = direction;
    slf.dir_error = deviation * PI;
    slf.is_directional = true;
    slf
  }
  pub fn set_life_params(self, life: f32, plus_or_minus: f32) -> Self {
    let mut slf = self;
    slf.half_life = life;
    slf.life_error = plus_or_minus;
    slf
  }
  pub fn set_parts_per_sec(self, pps: f32) -> Self {
    let mut slf = self;
    slf.pps = pps;
    slf
  }
  pub fn set_position(self, pos: Vector3f<f32>) -> Self {
    let mut slf = self;
    slf.base_pos = pos;
    slf
  }
  pub fn set_scale_params(self, scale: f32, plus_or_minus: f32) -> Self {
    let mut slf = self;
    slf.half_scale = scale;
    slf.scale_error = plus_or_minus;
    slf
  }
  pub fn set_speed_params(self, speed: f32, plus_or_minus: f32) -> Self {
    let mut slf = self;
    slf.half_speed = speed;
    slf.speed_error = plus_or_minus;
    slf
  }
  pub fn set_tex_row_count(self, row_count: u32) -> Self {
    let mut slf = self;
    slf.tex_rows = row_count;
    slf
  }
  pub fn set_texture(self, name: &str) -> Self {
    let mut slf = self;
    slf.texture = name.to_owned();
    slf
  }
}

#[derive(Default, Debug)]
pub struct ParticleSystems {
  // Gonna hold the particle rules 
}

pub struct ParticleVBO {
  pub vbo_id: VboID,
  pub quad: Model,
  pub max_instances: usize,
  pub instance_data_length: usize,
}
impl Default for ParticleVBO {
  fn default() -> Self {
    Self {
      vbo_id: VboID(0),
      quad: Model::default(),
      max_instances: 1_0000,
      instance_data_length: 21, 
    }
  }
}
impl ParticleVBO {
  pub fn init(self, world: &mut World) {
    let mut slf = self;
    {
      let mut loader = world.write_resource::<Loader>();
      slf.quad = loader.quad_0_5;
      slf.vbo_id = loader.create_empty_vbo(slf.max_instances * slf.instance_data_length);
      r_add_instanced_attrib(slf.quad.vao_id, slf.vbo_id, 1, 1, slf.instance_data_length, 0);
      r_add_instanced_attrib(slf.quad.vao_id, slf.vbo_id, 2, 4, slf.instance_data_length, 1);
      r_add_instanced_attrib(slf.quad.vao_id, slf.vbo_id, 3, 4, slf.instance_data_length, 5);
      r_add_instanced_attrib(slf.quad.vao_id, slf.vbo_id, 4, 4, slf.instance_data_length, 9);
      r_add_instanced_attrib(slf.quad.vao_id, slf.vbo_id, 5, 4, slf.instance_data_length, 13);
      r_add_instanced_attrib(slf.quad.vao_id, slf.vbo_id, 6, 4, slf.instance_data_length, 17);
    }
    world.insert(slf);
  }
}
