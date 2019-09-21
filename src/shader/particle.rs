

use {
  gl::{VERTEX_SHADER, GEOMETRY_SHADER, FRAGMENT_SHADER, },
  shader::{
    Shader,
    glslmaker::GlslMaker,
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
  let mut _test = GlslMaker::default();
  _test.with_stage(VERTEX_SHADER, |_parent,stage| {
    stage.with_in("pos")
    .with_out("texCoord")
    .with_uniforms(&[
      "u_Transform", "u_Projection", "u_View", "u_Texture", 
      "row_count", 
      "offset", 
      "light_pos", 
      "light_color", 
      // "attenuation"
    ])
    .with_func("main", &[("", "void")], "void", |__parent,func| {
      func
      .with_assign("offset", "vec4", "vec4(-1.0, 1.0, 0.0, 0.0)")
      ;
    })
    ;
  })
  .with_stage(GEOMETRY_SHADER, |_parent,_stage| {
    
  })
  .with_stage(FRAGMENT_SHADER, |_parent,_stage| {
    
  })
  ;
  println!("{:?}", _test);
  println!("Created particle shader.");
  out
}
