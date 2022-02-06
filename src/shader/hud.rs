

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

#[derive(Clone,Copy)]
enum TexCount {
  ONE = 1, TWO = 2,
}
pub struct HudShader {
  pub one_tex: Shader,
  pub two_tex: Shader,
  active_shader: TexCount,
}
impl Default for HudShader {
  fn default() -> Self {
    Self {
      one_tex: gen_hud_shader(TexCount::ONE),
      two_tex: gen_hud_shader(TexCount::TWO),
      active_shader: TexCount::ONE,
    }
  }
}
impl HudShader {
  pub fn shader(&self) -> &Shader {
    match self.active_shader {
      TexCount::ONE => { &self.one_tex }
      TexCount::TWO => { &self.two_tex }
    }
  }
  pub fn active_shader_is(&self) -> usize {
    self.active_shader as usize
  }
  pub fn next_active_shader(&mut self) {
    let tc = self.active_shader;
    self.active_shader = match tc {
      TexCount::ONE => { TexCount::TWO }
      TexCount::TWO => { TexCount::ONE }
    }
  }
}
impl ShaderWrapper for HudShader {
  fn projection_name(&self) -> String { "projection".to_owned() }
  fn start(&self) { self.shader().start() }
  fn stop(&self) { self.shader().stop() }
  fn load_bool(&self, name: &str, value: bool) { 
    self.shader().load_bool(name, value); 
  }
  fn load_int(&self, name: &str, value: GLint) { 
    self.shader().load_int(name, value); 
  }
  fn load_float(&self, name: &str, value: GLfloat) { 
    self.shader().load_float(name, value); 
  }
  fn load_vec_2f(&self, name: &str, value: Vector2f<f32>) { 
    self.shader().load_vec_2f(name, value); 
  }
  fn load_vec_3f(&self, name: &str, value: Vector3f<f32>) { 
    self.shader().load_vec_3f(name, value); 
  }
  fn load_quaternion(&self, name: &str, value: Quaternion<f32>) { 
    self.shader().load_quaternion(name, value); 
  }
  fn load_matrix(&self, name: &str, value: &Matrix4f<f32>) { 
    self.shader().load_matrix(name, value); 
  }
}

fn gen_hud_shader(tex_count: TexCount) -> Shader {
  let mut shader = match tex_count {
    TexCount::ONE => {
      Shader::new(ShaderConf::new("hud_one")
        .with_attributes_auto(vec!("a_Pos"))
        .with_uniforms(vec!(
          // Vertex
          "u_Transform",
          "row_count",
          "offset",
          "flip_y",
          // Fragment
          "tex",
        ))
      )
    }
    TexCount::TWO => {
      Shader::new(ShaderConf::new("hud_two")
        .with_attributes_auto(vec!("a_Pos"))
        .with_uniforms(vec!(
          // Vertex
          "u_Transform",
          "row_count",
          "offset",
          "flip_y",
        ))
        .with_sampler_uniforms(vec!(
          // Fragment
          ("guiTexture", 0),
          ("depthMap", 1),
        ))
      )
    }
  };
  shader.setup();
  println!("Created HUD shader.");
  shader
}
