

pub struct Lighting {
  pub light_pos: [[f32; 3]; 4],
  pub light_color: [[f32; 3]; 4],
  pub attenuation: [[f32; 3]; 4],
  pub shine_damper: f32,
  pub reflectivity: f32,
  pub use_fake_lighting: bool,
}

impl Lighting {
  pub fn new() -> Self {
    Lighting {
      light_pos: [[0_f32; 3]; 4],
      light_color: [[0_f32; 3]; 4],
      attenuation: [[0_f32; 3]; 4],
      shine_damper: 0_f32,
      reflectivity: 0_f32,
      use_fake_lighting: false,
    }
  }
  pub fn light_pos(&mut self, light_pos: [[f32; 3]; 4]) -> &mut Self {
    self.light_pos = light_pos;
    self
  }
  pub fn light_color(&mut self, light_color: [[f32; 3]; 4]) -> &mut Self {
    self.light_color = light_color;
    self
  }
  pub fn attenuation(&mut self, attenuation: [[f32; 3]; 4]) -> &mut Self {
    self.attenuation = attenuation;
    self
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
