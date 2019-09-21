

use {
  shader::{
    Shader,
  },
};

pub struct ParticleShader{pub shader: Shader}
impl Default for ParticleShader {
  fn default() -> Self {
    Self { shader: gen_particle_shader() }
  }
}

pub fn gen_particle_shader() -> Shader {
  let mut out = Shader::new("particle");
  out.add_attribute("pos")
  .add_uniforms(vec!(
    "u_Transform", "u_Projection", "u_View", "u_Texture", 
    "row_count", 
    "offset", 
    "light_pos", 
    "light_color", 
    // "attenuation"
  ))
  .setup();
  println!("Created particle shader.");
  out
}
