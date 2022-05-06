
use {
  gl::{
    *,
    // types::{
    //   // GLuint, GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
    // }, 
  },
  specs::{World, WorldExt, },
  HUD,
  shader::{
    ShaderWrapper, 
    HudShader,
  },
  util::{
    // Vector3f, 
    Vector2f, 
    // HashMap, HashSet, Arc, Mutex,
    // Vector2f, Vector4f, RVertex, RVertex2D,
  }, 
};

#[derive(Default)]
pub struct RenderHUD {
  pub shader: HudShader,
}
impl RenderHUD {
  pub fn render(&mut self, world: &World) {
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Gui Render Pass");
    {
      self.render_one_tex(world);
      self.render_two_tex(world);
    }
    unsafe {
      Disable(BLEND);
      Enable(DEPTH_TEST);
    }
  }
  fn render_one_tex(&mut self, world: &World) {
    let mut _hud = world.write_resource::<HUD>();
    while self.shader.active_shader_is() != 1 {
      self.shader.next_active_shader();
    }
    self.shader.start();
    
    let _offset = Vector2f::<f32>::blank();
    unsafe {
      BindVertexArray(_hud.quad.vao_id.0);
      EnableVertexAttribArray(0);
      for gui in &mut _hud.elements {
        if gui.tex_count != 1 { continue; }
        // Texture!
        ActiveTexture(TEXTURE0);
        BindTexture(TEXTURE_2D, gui.tex_id);
        // Shader Vars!
        self.shader.load_matrix("u_Transform", gui.transformation());
        self.shader.load_float("row_count", gui.row_count);
        self.shader.load_vec_2f("offset", gui.offset);
        self.shader.load_bool("flip_y", gui.flip_y);
        // Draw!
        DrawArrays(TRIANGLE_STRIP, 0, 4_i32);
      }
      DisableVertexAttribArray(0);
      BindVertexArray(0);
    }
    self.shader.stop();
  }
  fn render_two_tex(&mut self, world: &World) {
    let mut _hud = world.write_resource::<HUD>();
    while self.shader.active_shader_is() != 2 {
      self.shader.next_active_shader();
    }
    self.shader.start();
    
    let _offset = Vector2f::<f32>::blank();
    unsafe {
      BindVertexArray(_hud.quad.vao_id.0);
      EnableVertexAttribArray(0);
      for gui in &mut _hud.elements {
        if gui.tex_count != 2 { continue; }
        // Texture!
        ActiveTexture(TEXTURE0);
        BindTexture(TEXTURE_2D, gui.tex_id);
        ActiveTexture(TEXTURE1);
        BindTexture(TEXTURE_2D, gui.depth_tex_id);
        // Shader Vars!
        self.shader.load_matrix("u_Transform", gui.transformation());
        self.shader.load_float("row_count", gui.row_count);
        self.shader.load_vec_2f("offset", gui.offset);
        self.shader.load_bool("flip_y", gui.flip_y);
        // Draw!
        DrawArrays(TRIANGLE_STRIP, 0, 4_i32);
      }
      DisableVertexAttribArray(0);
      BindVertexArray(0);
    }
    self.shader.stop();
  }
  pub fn clean_up(&mut self) {
    self.shader.one_tex.clean_up();
    self.shader.two_tex.clean_up();
  }
}
