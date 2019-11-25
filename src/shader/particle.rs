

use {
  gl::{
    // VERTEX_SHADER, GEOMETRY_SHADER, FRAGMENT_SHADER, 
    types::*, 
  },
  // glsl::syntax::TranslationUnit,
  // glsl_quasiquote::glsl,
    
  crate::{
    shader::{
      Shader, ShaderConf, ShaderWrapper, 
      // glslmaker::GlslMaker,
    },
    util::{
      Vector2f, Vector3f, Quaternion, Matrix4f,
    },
  },
};

pub struct ParticleShader{pub shader: Shader}
impl Default for ParticleShader {
  fn default() -> Self {
    Self { shader: gen_particle_shader() }
  }
}
impl ShaderWrapper for ParticleShader {
  fn projection_name(&self) -> String { "projection".to_owned() }
  fn start(&self) { self.shader.start() }
  fn stop(&self) { self.shader.stop() }
  fn load_bool(&self, name: &str, value: bool) { 
    self.shader.load_bool(name, value); 
  }
  fn load_int(&self, name: &str, value: GLint) { 
    self.shader.load_int(name, value); 
  }
  fn load_float(&self, name: &str, value: GLfloat) { 
    self.shader.load_float(name, value); 
  }
  fn load_vec_2f(&self, name: &str, value: Vector2f<f32>) { 
    self.shader.load_vec_2f(name, value); 
  }
  fn load_vec_3f(&self, name: &str, value: Vector3f<f32>) { 
    self.shader.load_vec_3f(name, value); 
  }
  fn load_quaternion(&self, name: &str, value: Quaternion<f32>) { 
    self.shader.load_quaternion(name, value); 
  }
  fn load_matrix(&self, name: &str, value: &Matrix4f<f32>) { 
    self.shader.load_matrix(name, value); 
  }
}

pub fn gen_particle_shader() -> Shader {
  // let _tu: TranslationUnit = glsl!{
  //   in vec3 pos;
    
  //   uniform u_transform;
  //   uniform u_projection;
  //   uniform u_view;
  //   uniform u_texture;
    
  //   void main() {
      
  //   }
  // };
  // let mut _particle_vertex = String::new();
  // glsl::transpiler::glsl::show_translation_unit(&mut _particle_vertex, &_tu);
  let mut shader = Shader::new(
    ShaderConf::new("particle")
      .with_attributes(vec!(("pos", 0), ("blendFactor", 1), ("texOffsets", 2), ("view", 3)))
      .with_uniforms(vec!(
        "projection", "rowCount", 
      ))
  );
  shader.setup();
  // let mut _test = GlslMaker::default();
  // _test.with_stage(VERTEX_SHADER, |_parent,stage| {
  //   stage.with_in("pos")
  //   .with_out("texCoord")
  //   .with_uniforms(&[
  //     "u_Transform", "u_Projection", "u_View", "u_Texture", 
  //     "row_count", 
  //     "offset", 
  //     "light_pos", 
  //     "light_color", 
  //     // "attenuation"
  //   ])
  //   .with_func("main", &[("", "void")], "void", |__parent,func| {
  //     func
  //     .with_assign("offset", "vec4", "vec4(-1.0, 1.0, 0.0, 0.0)")
  //     ;
  //   })
  //   ;
  // })
  // .with_stage(GEOMETRY_SHADER, |_parent,_stage| {
    
  // })
  // .with_stage(FRAGMENT_SHADER, |_parent,_stage| {
    
  // })
  // ;
  // println!("{:?}", _test);
  // println!("Created particle shader.");
  shader
}
