

use {
  gl::{VERTEX_SHADER, GEOMETRY_SHADER, FRAGMENT_SHADER, },
  shader::{
    Shader, ShaderConf,
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
  let _tu: TranslationUnit = glsl!{
    in vec3 pos;
    
    uniform u_transform;
    uniform u_projection;
    uniform u_view;
    uniform u_texture;
    
    void main() {
      
    }
  };
  let mut _particle_vertex = String::new();
  glsl::transpiler::glsl::show_translation_unit(&mut _particle_vertex, &_tu);
  let mut shader = Shader::new(
    ShaderConf::new("particle")
      .with_attribute("pos")
      .with_uniforms(vec!(
        "u_Transform", "u_Projection", "u_View", "u_Texture",
        "row_count",
        "offset",
        "light_pos",
        "light_color",
        // "attenuation"
      ))
  );
  shader.setup();
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
  shader
}
