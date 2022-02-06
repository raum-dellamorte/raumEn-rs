

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

pub struct TestParticleShader{pub shader: Shader}
impl Default for TestParticleShader {
  fn default() -> Self {
    Self { shader: gen_shader() }
  }
}
impl ShaderWrapper for TestParticleShader {
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

fn gen_shader() -> Shader {
  let mut shader = Shader::new(
    ShaderConf::new("testparticle")
      .with_attributes(vec!(("pos", 0), ) ) //  ("modelview", 1), ("blendFactor", 2), ("texOffsets", 3), 
      .with_uniforms(vec!(
        "projection", "modelview", // "rowCount", 
      ))
  );
  shader.setup();
  shader
}
