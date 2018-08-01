
use shader::Shader;
use util::rvector::Vector3f;

#[derive (Debug, PartialEq)]
pub struct Lighting {
  pub shine_damper: f32,
  pub reflectivity: f32,
  pub use_fake_lighting: bool,
}

impl Lighting {
  pub fn new() -> Self {
    Lighting {
      shine_damper: 2_f32,
      reflectivity: 1_f32,
      use_fake_lighting: false,
    }
  }
  pub fn load_to_shader(&self, shader: &Shader) {
    // Assumes shader is active
    shader.load_float("shine_damper", self.shine_damper);
    shader.load_float("reflectivity", self.reflectivity);
    // shader.load_bool("use_fake_lighting", self.use_fake_lighting);
  }
  pub fn shine_damper(&mut self, shine_damper: f32) -> &mut Self {
    self.shine_damper = shine_damper;
    self
  }
  pub fn reflectivity(&mut self, reflectivity: f32) -> &mut Self {
    self.reflectivity = reflectivity;
    self
  }
  pub fn use_fake_lighting(&mut self) -> &mut Self {
    self.use_fake_lighting = !self.use_fake_lighting;
    self
  }
}

pub struct Light {
  pub pos: Vector3f,
  pub color: Vector3f,
  pub atten: Vector3f,
}

impl Light {
  pub fn new() -> Self {
    Light {
      pos: Vector3f::blank(),
      color: Vector3f::new(0.5, 0.5, 0.5),
      atten: Vector3f::blank(),
    }
  }
  pub fn load_to_shader_single(&self, shader: &Shader) {
    // Assumes shader is active
    shader.load_vec_3f("light_pos", &self.pos);
    shader.load_vec_3f("light_color", &self.color);
    // shader.load_vec_3f("attenuation", &self.atten);
  }
  pub fn load_to_shader_array(&self, shader: &Shader, id: usize) {
    // Assumes shader is active
    shader.load_vec_3f(&format!("light_pos[{}]", id), &self.pos);
    shader.load_vec_3f(&format!("light_color[{}]", id), &self.color);
    shader.load_vec_3f(&format!("attenuation[{}]", id), &self.atten);
  }
  pub fn set_pos(&mut self, light_pos: Vector3f) -> &mut Self {
    self.pos = light_pos;
    self
  }
  pub fn set_color(&mut self, light_color: Vector3f) -> &mut Self {
    self.color = light_color;
    self
  }
  pub fn set_attenuation(&mut self, attenuation: Vector3f) -> &mut Self {
    self.atten = attenuation;
    self
  }
}

pub struct Lights {
  pub lights: Vec<Light>,
}

impl Lights {
  pub fn new() -> Self {
    Lights { lights: Vec::new() }
  }
  pub fn add_light(&mut self) {
    self.lights.push(Light::new());
  }
  pub fn load_to_shader(&self, shader: &Shader) {
    // Assumes shader is active
    self.lights[0].load_to_shader_single(shader);
    // let count = 0;
    // for light in &self.lights {
    //   if !(count < 4) {break}
    //   light.load_to_shader_array(shader, count);
    // }
  }
}
