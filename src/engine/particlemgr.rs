
use {
  std::{
    f32::consts::{
      //PI, 
      TAU, 
    },
  },
  rand::Rng,
  crate::{
    constants::*,
    ecs::resource::{
      Model, 
      // Texture,
    },
    shader::{
      TestParticleShader, ShaderWrapper, 
    },
    util::{
      // Vector2f,
      Vector3f,
      // Quaternion,
      Rotator,
      Matrix4f,
      RVec,
      rgl::{
        *,
      },
    },
  },
};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Particle(usize);
#[allow(dead_code)]
impl Particle {
  pub fn id(&self) -> usize { self.0 } // for readability
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SystemID(usize);
#[allow(dead_code)]
impl SystemID {
  pub fn id(&self) -> usize { self.0 } // for readability
}

#[allow(dead_code)]
pub struct ParticleMgr{
  position: Vector3f<f32>,
  active: Vec<Particle>,
  inactive: Vec<Particle>,
  attribs: ParticleAttribs,
  textures: ParticleTextures,
  systems: ParticleSystems,
  quad: Model,
  shader: TestParticleShader,
  loader: ParticleLoader,
}
impl Default for ParticleMgr {
  fn default() -> Self {
    let position = Vector3f::default();
    let active = Vec::new();
    let inactive = Vec::new();
    let attribs = ParticleAttribs::default();
    let textures = ParticleTextures::default();
    let systems = ParticleSystems::default();
    let quad = Model { vao_id: VaoID(0), vertex_count: VertexCount(0) };
    let shader = TestParticleShader::default();
    let loader = ParticleLoader::default();
    Self { position, active, inactive, attribs, textures, systems, quad, shader, loader, }
  }
}
#[allow(dead_code)]
impl ParticleMgr {
  pub fn init(self) -> Self {
    let mut s = self;
    let quad_vec = vec!(-0.5,0.5, -0.5,-0.5, 0.5,0.5, 0.5,-0.5);
    let pmat = DISPLAY.lock().unwrap().proj_mat.clone();
    s.quad = s.loader.load_to_vao(&quad_vec);
    s.shader.start();
    s.shader.update_projection(&pmat);
    s.shader.stop();
    s
  }
  pub fn position_mut(&mut self) -> &mut Vector3f<f32> {
    &mut self.position
  }
  pub fn systems_mut(&mut self) -> &mut ParticleSystems {
    &mut self.systems
  }
  pub fn render(&self) {
    let view = DISPLAY.lock().unwrap().camera.view_mat.clone();
    self.shader.start();
    // Prepare
    r_bind_vertex_array(self.quad.vao_id);
    r_enable_vertex_attrib_array(0);
    GlSettings::default()
        .disable_depth_mask()
        .enable_blend()
        .set();
    RBlend::DefaultBlend.exec();
    // Render
    for p in &self.active {
      self.update_model_view_matrix(self.attribs.position(*p), self.attribs.rotation(*p), self.attribs.scale(*p), &view);
      // print!("I should be drawing - ");
      r_draw_arrays_triangle_strip(self.quad.vertex_count);
    }
    // Cleanup
    GlSettings::default()
        .enable_depth_mask()
        .disable_blend()
        .set();
    r_disable_vertex_attrib_array(0);
    r_unbind_vertex_array();
    self.shader.stop();
  }
  fn emit_particle(&mut self, system_id: SystemID) {
    let p = self.acquire_particle();
    self.attribs.apply_rule_set(p, &self.systems.rule(system_id));
    
  }
  pub fn emit_particles(&mut self, system_id: SystemID, delta: f32) {
    if self.active.len() > 1000 { return; }
    let particles_to_create = self.systems.rule(system_id).pps * delta;
    let count = particles_to_create.floor() as i32;
    let partial_particle: f32 = particles_to_create.fract();
    for _ in 0..count { self.emit_particle(system_id) };
    if self.attribs.rng.gen::<f32>() < partial_particle { 
      self.emit_particle(system_id);
    }
  }
  fn update_particle(&mut self, particle: Particle, delta: f32) {
    { *self.attribs.life_used_mut(particle) += delta; }
    if self.attribs.life_used(particle) > self.attribs.life_total(particle) {
      self.retire_particle(particle);
      return;
    }
    self.attribs.update_particle(particle, delta);
  }
  pub fn update_particles(&mut self, delta: f32) {
    // let delta = DISPLAY.lock().unwrap().timer.delta;
    for p in self.active.clone() {
      self.update_particle(p, delta);
    }
  }
  fn acquire_particle(&mut self) -> Particle {
    let p = self.inactive.pop();
    if p.is_none() { return self.create_particle(); }
    let p = p.unwrap();
    self.active.push(p);
    p
  }
  fn create_particle(&mut self) -> Particle { // TODO: When creating or resetting particle we need to take the texture id.
    let pid = self.active.len() + self.inactive.len();
    // print!("{} ", pid);
    let p = Particle(pid);
    self.attribs.register(p);
    self.active.push(p);
    p
  }
  fn retire_particle(&mut self, particle: Particle) {
    if let Some(pid) = self.find_active(particle) {
      self.active.swap_remove(pid); // Should be fast but loses order and that may be important later
    }
    self.inactive.push(particle);
    // println!("Retiring particle {}. active: {} inactive: {}", particle.id(), self.active.len(), self.inactive.len() );
  }
  fn find_active(&self, particle: Particle) -> Option<usize> {
    for (id, p) in self.active.iter().enumerate() {
      if *p == particle { return Some(id) }
    }
    None
  }
  fn update_model_view_matrix(&self, position: Vector3f<f32>, rotation: f32, scale: f32, view: &Matrix4f<f32>) {
    let mut model_mat: Matrix4f<f32> = Matrix4f::default();
    model_mat.translate_v3f(position);
    model_mat.transpose3x3(view);
    model_mat.rotate(rotation.to_radians(), Vector3f { x: 0_f32, y: 0_f32, z: 1_f32 });
    model_mat.scale(Vector3f { x: scale, y: scale, z: scale });
    let model_view = *view * model_mat;
    self.shader.load_matrix("modelview", &model_view);
  }
  pub fn clean_up(&mut self) {
    self.shader.shader.clean_up();
  }
}

pub struct ParticleSystems {
  rules: Vec<ParticleSystem>,
}
impl Default for ParticleSystems {
  fn default() -> Self {
    Self {
      rules: Vec::new(),
    }
  }
}
impl ParticleSystems {
  pub fn new_rule_set<F>(&mut self, f: F) -> SystemID
  where F: Fn(ParticleSystem) -> ParticleSystem {
    let id = self.rules.len();
    let h = ParticleSystem::default();
    let rule_set = f(h);
    self.rules.push(rule_set);
    SystemID(id)
  }
  pub fn recycle<F>(&mut self, rule: SystemID, f: F) // May never use this...
  where F: Fn(ParticleSystem) -> ParticleSystem {
    let h = ParticleSystem::default();
    let rule_set = f(h);
    self.rules[rule.id()] = rule_set;
  }
  pub fn rule(&self, rule: SystemID) -> &ParticleSystem {
    &self.rules[rule.id()]
  }
  pub fn rule_mut(&mut self, rule: SystemID) -> &mut ParticleSystem {
    &mut self.rules[rule.id()]
  }
}

#[derive(Default, Debug)]
pub struct ParticleSystem {
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
impl ParticleSystem {
  pub fn randomize_rotation(self) -> Self {
    let mut slf = self;
    slf.rand_rot = !slf.rand_rot;
    slf
  }
  pub fn set_direction(self, direction: Vector3f<f32>, deviation: f32) -> Self {
    let mut slf = self;
    slf.base_dir = direction;
    slf.dir_error = deviation; // in degrees
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
// #[allow(dead_code)]
struct ParticleAttribs {
  pub rotator: Rotator<f32>,
  pub rng: rand::prelude::ThreadRng,
  position: Vec<Vector3f<f32>>,
  velocity: Vec<Vector3f<f32>>,
  rotation: Vec<f32>,
  scale: Vec<f32>,
  gravity: Vec<f32>,
  life_total: Vec<f32>,
  life_used: Vec<f32>,
  distance: Vec<f32>,
  texture: Vec<Option<u8>>,
  offsets: Vec<(f32,f32)>,
  blend: Vec<f32>,
}
impl Default for ParticleAttribs {
  fn default() -> Self {
    Self{
      rotator: Rotator::default(),
      rng: rand::prelude::ThreadRng::default(),
      position: Vec::new(),
      velocity: Vec::new(),
      rotation: Vec::new(),
      scale: Vec::new(),
      gravity: Vec::new(),
      life_total: Vec::new(),
      life_used: Vec::new(),
      distance: Vec::new(),
      texture: Vec::new(),
      offsets: Vec::new(),
      blend: Vec::new(),
    }
  }
}
#[allow(dead_code)]
impl ParticleAttribs {
  pub fn register(&mut self, particle: Particle) {
    // println!(".{:?} ", self.sanity_check());
    if particle.id() != self.position.len() {
      // The particle we're registering should always be equal to the current length of all vecs
      panic!("Registering new particle in ParticleAttribs: Bad Particle ID {} expected {}", particle.0, self.position.len() );
    }
    self.position.push(Vector3f::default());
    self.velocity.push(Vector3f::default());
    self.rotation.push(0.0);
    self.scale.push(1.0);
    self.gravity.push(1.0);
    self.life_total.push(2.0);
    self.life_used.push(0.0);
    self.distance.push(0.0);
    self.texture.push(None);
    self.offsets.push((0.0, 0.0));
    self.blend.push(0.0);
  }
  pub fn sanity_check(&self) -> Result<usize, &str> {
    let l = self.position.len();
    if self.velocity.len() != l { return Err("ParticleAttribs: sanity_check: velocity.len() is bad.") }
    if self.rotation.len() != l { return Err("ParticleAttribs: sanity_check: rotation.len() is bad.") }
    if self.scale.len() != l { return Err("ParticleAttribs: sanity_check: scale.len() is bad.") }
    if self.gravity.len() != l { return Err("ParticleAttribs: sanity_check: gravity.len() is bad.") }
    if self.life_total.len() != l { return Err("ParticleAttribs: sanity_check: life_total.len() is bad.") }
    if self.life_used.len() != l { return Err("ParticleAttribs: sanity_check: life_used.len() is bad.") }
    if self.distance.len() != l { return Err("ParticleAttribs: sanity_check: distance.len() is bad.") }
    if self.texture.len() != l { return Err("ParticleAttribs: sanity_check: texture.len() is bad.") }
    if self.offsets.len() != l { return Err("ParticleAttribs: sanity_check: offsets.len() is bad.") }
    if self.blend.len() != l { return Err("ParticleAttribs: sanity_check: blend.len() is bad.") }
    Ok(l)
  }
  pub fn apply_rule_set(&mut self, particle: Particle, system: &ParticleSystem) {
    self.position_mut(particle).copy_from_v3f(system.base_pos);
    let mut direction = if system.is_directional {
      gen_random_unit_vector3f_within_cone(&mut self.rng, &mut self.rotator, ZVEC, system.dir_error )
    } else {
      gen_random_unit_vector3f(&mut self.rng)
    };
    direction.normalize();
    let speed = gen_value(&mut self.rng, system.half_speed, system.speed_error);
    self.velocity_mut(particle).copy_from_v3f(direction * speed);
    *self.scale_mut(particle) = gen_value(&mut self.rng, system.half_scale, system.scale_error);
    *self.life_total_mut(particle) = gen_value(&mut self.rng, system.half_life, system.life_error);
    *self.life_used_mut(particle) = 0.0;
    *self.rotation_mut(particle) = gen_rotation(&mut self.rng, system.rand_rot);
    
  }
  pub fn update_particle(&mut self, particle: Particle, delta: f32) {
    let vel: Vector3f<f32> = {
      let grav = self.gravity(particle);
      let vel = self.velocity_mut(particle);
      if -vel.y < TERMVEL { vel.y -= GRAVITY * grav * delta; } else { vel.y = -TERMVEL; }
      *vel
    };
    *self.position_mut(particle) += vel * delta;
    
  }
  pub fn position(&self, particle: Particle) -> Vector3f<f32> {
    self.position[particle.id()]
  }
  pub fn position_mut(&mut self, particle: Particle) -> &mut Vector3f<f32> {
    &mut self.position[particle.id()]
  }
  pub fn velocity(&self, particle: Particle) -> Vector3f<f32> {
    self.velocity[particle.id()]
  }
  pub fn velocity_mut(&mut self, particle: Particle) -> &mut Vector3f<f32> {
    &mut self.velocity[particle.id()]
  }
  pub fn rotation(&self, particle: Particle) -> f32 {
    self.rotation[particle.id()]
  }
  pub fn rotation_mut(&mut self, particle: Particle) -> &mut f32 {
    &mut self.rotation[particle.id()]
  }
  pub fn scale(&self, particle: Particle) -> f32 {
    self.scale[particle.id()]
  }
  pub fn scale_mut(&mut self, particle: Particle) -> &mut f32 {
    &mut self.scale[particle.id()]
  }
  pub fn gravity(&self, particle: Particle) -> f32 {
    self.gravity[particle.id()]
  }
  pub fn gravity_mut(&mut self, particle: Particle) -> &mut f32 {
    &mut self.gravity[particle.id()]
  }
  pub fn life_total(&self, particle: Particle) -> f32 {
    self.life_total[particle.id()]
  }
  pub fn life_total_mut(&mut self, particle: Particle) -> &mut f32 {
    &mut self.life_total[particle.id()]
  }
  pub fn life_used(&self, particle: Particle) -> f32 {
    self.life_used[particle.id()]
  }
  pub fn life_used_mut(&mut self, particle: Particle) -> &mut f32 {
    &mut self.life_used[particle.id()]
  }
  pub fn distance(&self, particle: Particle) -> f32 {
    self.distance[particle.id()]
  }
  pub fn distance_mut(&mut self, particle: Particle) -> &mut f32 {
    &mut self.distance[particle.id()]
  }
  pub fn texture_id(&self, particle: Particle) -> Option<usize> {
    if let Some(id) = self.texture[particle.id()] {
      return Some(id as usize);
    }
    None
  }
  pub fn offsets(&self, particle: Particle) -> (f32, f32) {
    self.offsets[particle.id()]
  }
  pub fn blend(&self, particle: Particle) -> f32 {
    self.blend[particle.id()]
  }
}

#[allow(dead_code)]
struct ParticleTextures {
  texture: Vec<String>,
  row_count: Vec<u8>,
}
impl Default for ParticleTextures {
  fn default() -> Self {
    ParticleTextures{
      texture: Vec::new(),
      row_count: Vec::new(),
    }
  }
}
#[allow(dead_code)]
impl ParticleTextures {
  pub fn sanity_check(&self) -> Result<(), &str> {
    let l = self.texture.len();
    if self.row_count.len() != l { return Err("ParticleTextures: sanity_check: row_count.len() is bad.") }
    Ok(())
  }
  pub fn texture(&self, particle: Particle) -> &String {
    &self.texture[particle.id()]
  }
  pub fn row_count(&self, particle: Particle) -> usize {
    self.row_count[particle.id()] as usize
  }
}

struct ParticleLoader {
  vaos: Vec<VaoID>,
  vbos: Vec<VboID>,
}
impl Default for ParticleLoader {
  fn default() -> Self {
    Self {
      vaos: Vec::new(),
      vbos: Vec::new(),
    }
  }
}
#[allow(dead_code)]
impl ParticleLoader {
  fn create_vao(&mut self) -> VaoID {
    let vao_id = VaoID(r_gen_vertex_arrays());
    assert!(vao_id.0 != 0);
    self.vaos.push(vao_id);
    r_bind_vertex_array(vao_id);
    vao_id
  }
  pub fn load_to_vao(&mut self, verts: &Vec<f32>) -> Model {
    let vao_id = self.create_vao();
    self.bind_attrib(0, 2, verts);
    r_unbind_vertex_array();
    Model::new(vao_id, VertexCount((verts.len() / 2) as i32) )
  }
  pub fn load_to_vao_textured(&mut self, verts: &Vec<f32>, tex_coords: &Vec<f32>) -> VaoID {
    let vao_id = self.create_vao();
    self.bind_attrib(0, 2, verts);
    self.bind_attrib(1, 2, tex_coords);
    r_unbind_vertex_array();
    vao_id
  }
  pub fn create_empty_vbo(&mut self, count: usize) -> VboID {
    let vbo = VboID(r_gen_buffers());
    self.vbos.push(vbo);
    r_bind_buffer(ARRAY_BUFFER, vbo);
    r_buffer_data(ARRAY_BUFFER, (count * 4) as isize, None, STREAM_DRAW);
    r_bind_buffer(ARRAY_BUFFER, VboID(0) );
    return vbo
  }
  fn add_instanced_attribute(vao: VaoID, vbo: VboID, attrib: u32, size: GLint, stride: i32, offset: i32 ) {
    r_bind_buffer(ARRAY_BUFFER, vbo);
    r_bind_vertex_array(vao);
    r_vertex_attrib_pointer(attrib, size, FLOAT, FALSE, stride * 4, offset * 4 );
    r_vertex_attrib_divisor(attrib, 1); // lwjgl original: ARBInstancedArrays.glVertexAttribDivisorARB(attrib, 1);
    r_bind_buffer(ARRAY_BUFFER, VboID(0) );
    r_bind_vertex_array( VaoID(0) );
  }
  fn update_vbo(vbo: VboID, data: &Vec<f32>) {
    r_bind_buffer(ARRAY_BUFFER, vbo);
    r_buffer_data(ARRAY_BUFFER, (data.len() * 4) as isize, None, STREAM_DRAW);
    r_buffer_sub_data(ARRAY_BUFFER, 0, 0, Some(data) );
    r_bind_buffer(ARRAY_BUFFER, VboID(0) );
  }
  fn bind_attrib(&mut self, attrib: u32, step: GLint, data: &Vec<f32>) {
    let vbo_id = VboID(r_gen_buffers());
    assert!(vbo_id.0 != 0);
    self.vbos.push(vbo_id);
    r_bind_buffer(ARRAY_BUFFER, vbo_id);
    r_buffer_data(ARRAY_BUFFER, (data.len() * 4) as isize, Some(data), STATIC_DRAW);
    r_vertex_attrib_pointer(attrib, step, FLOAT, FALSE, 0, 0);
    r_bind_buffer(ARRAY_BUFFER, VboID(0));
  }
}

fn gen_random_unit_vector3f(rng: &mut rand::prelude::ThreadRng, ) -> Vector3f<f32> {
  let theta: f32 = rng.gen::<f32>() * TAU;
  let z: f32 = (rng.gen::<f32>() * TAU) - 1.0;
  let root_one_minus_z_squared: f32 = (1.0 - (z * z)).sqrt();
  let x: f32 = root_one_minus_z_squared * theta.cos();
  let y: f32 = root_one_minus_z_squared * theta.sin();
  Vector3f {x, y, z}
}

fn gen_random_unit_vector3f_within_cone(
    rng: &mut rand::prelude::ThreadRng, rotator: &mut Rotator<f32>, 
    cone_dir: Vector3f<f32>, angle: f32
) -> Vector3f<f32> {
  let cos_angle = angle.cos();
  let theta: f32 = rng.gen::<f32>() * TAU;
  let z: f32 = cos_angle + (rng.gen::<f32>() * (1.0 - cos_angle));
  let root_one_minus_z_squared: f32 = (1.0 - (z * z)).sqrt();
  let x = root_one_minus_z_squared * theta.cos();
  let y = root_one_minus_z_squared * theta.sin();
  let mut dir_tmp = Vector3f::new(x, y, z);
  let mut rotate_axis: Vector3f<f32> = Vector3f::default();
  if cone_dir.x != 0.0 || cone_dir.y != 0.0 || 
    ((cone_dir.z - 1.0).abs() > TOLERANCE64 && (cone_dir.z + 1.0).abs() > TOLERANCE64)
  {
    cone_dir.cross_to(ZVEC, &mut rotate_axis);
    rotate_axis.normalize();
    let rotate_angle = cone_dir.dot(ZVEC).acos();
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

#[allow(dead_code)]
fn gen_random_unit_vector3f_within_cone_bak(
    rng: &mut rand::prelude::ThreadRng, rotator: &mut Rotator<f32>, 
    cone_dir: Vector3f<f32>, angle: f32) -> Vector3f<f32> // angle in degrees
{ 
  let xrad = ((2.0 * angle * rng.gen::<f32>()) - angle ).to_radians();
  let yrad = ((2.0 * angle * rng.gen::<f32>()) - angle ).to_radians();
  let zrad = ((2.0 * angle * rng.gen::<f32>()) - angle ).to_radians();
  let mut dir_tmp: Vector3f<f32> = Vector3f::default();
  rotator
    .set_point(cone_dir)
    .set_axis(XVEC)
    .set_angle(xrad)
    .rotate()
    .set_axis(YVEC)
    .set_angle(yrad)
    .rotate()
    .set_axis(ZVEC)
    .set_angle(zrad)
    .rotate()
    .get_point(&mut dir_tmp);
  // println!("angle {} xrad {} yrad {} zrad {} result {}", angle, xrad, yrad, zrad, dir_tmp);
  dir_tmp
}

fn gen_value(rng: &mut rand::prelude::ThreadRng, average: f32, error_margin: f32) -> f32 {
  let offset = (rng.gen::<f32>() - 0.5) * 2.0 * error_margin;
  average + offset
}

fn gen_rotation(rng: &mut rand::prelude::ThreadRng, bother: bool) -> f32 {
  if bother {
    rng.gen::<f32>() * 360.0
  } else {
    0.0
  }
}
