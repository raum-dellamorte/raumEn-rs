

use {
  std::f64::consts::{ PI, },
  specs::{
    Component, VecStorage, World, NullStorage, 
  },
  ecs::c::{ Rotator, },
  util::{
    RVec, Vector2f, Vector3f, 
    // HashSet,
    TAU, ZVEC64, 
  },
};

const TOLERANCE64: f64 = 0.00001;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Particle;
impl Particle {
  pub fn prep_new(_world: &mut World, _particle_count: u32) {
    
  }
  pub fn launch(_world: &mut World, _system: ParticleSystem) {
    
  }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ParticleTexture {
  pub id: u32,
  pub num_of_rows: u32,
  pub additive: bool,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GravPercent(pub f64);

#[derive(Component, Debug)]
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
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct ParticleAlive;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CamDistance(pub f64);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TexOffsets{
  pub a: Vector2f<f64>,
  pub b: Vector2f<f64>,
}

#[derive(Default, Debug)]
pub struct ParticleSystem {
  pub half_life: f64,
  pub half_speed: f64,
  pub half_scale: f64,
  pub base_pos: Vector3f<f64>,
  pub base_dir: Vector3f<f64>,
  pub dir_error: f64,
  pub rand_rot: bool,
  
}
impl ParticleSystem {
  pub fn set_direction(&mut self, direction: Vector3f<f64>, deviation: f64) {
    self.base_dir = direction;
    self.dir_error = deviation * PI;
  }
  pub fn randomize_rotation(&mut self) { self.rand_rot = !self.rand_rot }
  // pub fn
}

// These 2 funcs are ported over from the Kotlin version
// I **think** the math is the same...
pub fn gen_random_unit_vector3f() -> Vector3f<f64> {
  use rand::Rng;
  let mut rng = rand::thread_rng();
  let theta: f64 = rng.gen::<f64>() * TAU;
  let z: f64 = (rng.gen::<f64>() * TAU) - 1.0;
  let root_one_minus_z_squared: f64 = (1.0 - (z * z)).sqrt();
  let x: f64 = root_one_minus_z_squared * theta.cos();
  let y: f64 = root_one_minus_z_squared * theta.sin();
  Vector3f {x, y, z}
}
pub fn gen_random_unit_vector3f_within_cone(rotator: &mut Rotator<f64>, cone_dir: Vector3f<f64>, angle: f64) -> Vector3f<f64> {
  use rand::Rng;
  let mut rng = rand::thread_rng();
  let cos_angle = angle.cos();
  let theta: f64 = rng.gen::<f64>() * TAU;
  let z: f64 = cos_angle + (rng.gen::<f64>() * (1.0 - cos_angle));
  let root_one_minus_z_squared: f64 = (1.0 - (z * z)).sqrt();
  let x = root_one_minus_z_squared * theta.cos();
  let y = root_one_minus_z_squared * theta.sin();
  let mut dir_tmp = Vector3f::new(x, y, z);
  let mut rotate_axis: Vector3f<f64> = Vector3f::default();
  if cone_dir.x != 0.0 || cone_dir.y != 0.0 || 
    ((cone_dir.z - 1.0).abs() > TOLERANCE64 && (cone_dir.z + 1.0).abs() > TOLERANCE64)
  {
    cone_dir.cross_to(ZVEC64, &mut rotate_axis);
    rotate_axis.normalize();
    let rotate_angle = cone_dir.dot(ZVEC64).acos();
    // The Kotlin version used a rotation matrix, which I now think is clunky
    // I **think** this will do the same thing with less work
    // I love my quaternion rotator <3
    rotator
      .set_axis(rotate_axis)
      .set_point(Vector3f {x, y, z})
      .set_angle(-rotate_angle)
      .rotate()
      .get_point(&mut dir_tmp);
  } else if (cone_dir.z + 1.0).abs() < TOLERANCE64 {
    dir_tmp.z *= -1.0;
  }
  dir_tmp
}
