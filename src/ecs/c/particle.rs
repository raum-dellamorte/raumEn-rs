

use {
  std::{
    f64::consts::{ PI, },
  },
  specs::{*, WorldExt, },
  rand::Rng,
  ecs::c::{*, material::*, },
  util::{
    RVec, Vector2f, Vector3f, 
    // HashSet,
    TAU, ZVEC64, 
    specs::*,
  },
};

const TOLERANCE64: f64 = 0.00001;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Particle;
impl Particle {
  pub fn gen_particles(world: &mut World, system: &ParticleSystem) {
    let mut rng = rand::thread_rng();
    let mut rotator = Rotator::<f64>::default();
    let delta = {
      let handler = world.read_resource::<crate::Handler>();
      f64::from(handler.timer.delta)
    };
    let particles_to_create = system.pps * delta;
    let count = particles_to_create.floor() as i32;
    let partial_particle: f64 = particles_to_create.fract();
    for _ in 0..count { emit_particle(world, system, &mut rng, &mut rotator) };
    if rng.gen::<f64>() < partial_particle { 
      emit_particle(world, system, &mut rng, &mut rotator);
    }
  }
}
fn emit_particle(
  world: &mut World, system: &ParticleSystem, 
  rng: &mut rand::prelude::ThreadRng, rotator: &mut Rotator<f64>
) {
  let mut velocity = if system.is_directional {
    gen_random_unit_vector3f_within_cone(rng, rotator, system.base_dir, system.angle)
  } else {
    gen_random_unit_vector3f(rng)
  };
  velocity.normalize();
  velocity.scale(gen_value(rng, system.half_speed, system.speed_error));
  let scale = gen_value(rng, system.half_scale, system.scale_error);
  let life = gen_value(rng, system.half_life, system.life_error);
  let rot = gen_rotation(rng, system.rand_rot);
  // emit_particle(
  //   world, texture, system.base_pos, velocity, 
  //   , 
  //   scale, system.gravity_mult, life
  // );
  let p_ent = get_particle(world);
  // p.apply {
  //   pTexture = ntexture
  //   pos.set(npos)
  //   velocity.set(nvelocity)
  //   rot = nrot
  //   scale = nscale
  //   gravEffect = ngravEffect
  //   life = nlife
  //   elapsedTime = 0.0
  // }
  // add(p)
  // return p
  
  // mod_comp::<>(world, p_ent, "Particle ", &|o| {
  //   // o.0 = ;
  // });
  
  mod_comp::<TextureComponent>(world, p_ent, "Particle TextureComponent", &|o| {
    o.0 = system.texture.to_owned();
  });
  mod_comp::<TexIndexComponent>(world, p_ent, "Particle TexIndexComponent", &|o| {
    o.0 = 0;
  });
  mod_comp::<Position>(world, p_ent, "Particle Position", &|o| {
    o.0 = system.base_pos.into();
  });
  mod_comp::<Velocity>(world, p_ent, "Particle Velocity", &|o| {
    o.0 = velocity.into();
  });
  mod_comp::<Rotation>(world, p_ent, "Particle Rotation", &|o| {
    o.0.z = rot as f32;
  });
  mod_comp::<ScaleFloat>(world, p_ent, "Particle ScaleFloat", &|o| {
    o.0 = scale as f32;
  });
  mod_comp::<ParticleLife>(world, p_ent, "Particle ParticleLife", &|o| {
    o.set_life(life);
  });
  
  ins_flag::<ParticleAlive>(world, p_ent, "Particle ParticleAlive");
  
}

fn get_particle(world: &mut World) -> Entity {
  {
    let e = world.entities();
    let particles = world.read_storage::<Particle>();
    let particles_alive = world.read_storage::<ParticleAlive>();
    if let Some((ent, _, _)) = (&e, &particles, !&particles_alive).join()
      .collect::<Vec<_>>().pop()
    { return ent; }
  }
  create_particle(world)
}
fn create_particle(world: &mut World) -> Entity {
  world.create_entity()
    .with(Particle::default())
    .with(Position::default())
    .with(Velocity::default())
    .with(TexAtlas::default())
    .with(ParticleLife::default())
    .with(GravPercent::default())
    .with(TextureComponent("cosmic".to_owned()))
    .with(Position::default())
    .build()
}
// These 2 funcs are ported over from the Kotlin version
// I **think** the math is the same...
pub fn gen_random_unit_vector3f(rng: &mut rand::prelude::ThreadRng, ) -> Vector3f<f64> {
  let theta: f64 = rng.gen::<f64>() * TAU;
  let z: f64 = (rng.gen::<f64>() * TAU) - 1.0;
  let root_one_minus_z_squared: f64 = (1.0 - (z * z)).sqrt();
  let x: f64 = root_one_minus_z_squared * theta.cos();
  let y: f64 = root_one_minus_z_squared * theta.sin();
  Vector3f {x, y, z}
}
pub fn gen_random_unit_vector3f_within_cone(
    rng: &mut rand::prelude::ThreadRng, rotator: &mut Rotator<f64>, 
    cone_dir: Vector3f<f64>, angle: f64
) -> Vector3f<f64> {
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

fn gen_value(rng: &mut rand::prelude::ThreadRng, average: f64, error_margin: f64) -> f64 {
  let offset = (rng.gen::<f64>() - 0.5) * 2.0 * error_margin;
  average + offset
}

fn gen_rotation(rng: &mut rand::prelude::ThreadRng, bother: bool) -> f64 {
  if bother {
    rng.gen::<f64>() * 360.0
  } else {
    0.0
  }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexAtlas {
  pub num_of_rows: u32,
  pub additive: bool,
}

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
#[storage(NullStorage)]
pub struct ParticleAlive;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct CamDistance(pub f64);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexOffsets{
  pub a: Vector2f<f64>,
  pub b: Vector2f<f64>,
}

#[derive(Component, Default, Debug)]
#[storage(DenseVecStorage)]
pub struct ParticleSystem {
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
impl ParticleSystem {
  pub fn set_direction(&mut self, direction: Vector3f<f64>, deviation: f64) {
    self.base_dir = direction;
    self.dir_error = deviation * PI;
    self.is_directional = true;
  }
  pub fn randomize_rotation(&mut self) { self.rand_rot = !self.rand_rot }
  // pub fn
}
