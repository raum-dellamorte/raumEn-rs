
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use CVOID;

use entities::PosMarker;
use {GameMgr, Shader, Texture}; // Lights, Camera, 
use model::RawModel;
use shader::gen_hud_shader;
use text::{TextMgr, RFontType};
use util::{Vector3f, Vector2f, HashMap, HashSet, Arc, Mutex}; // Vector2f, Vector4f, RVertex, RVertex2D

pub struct RenderHUD {
  pub shader: Shader,
}

impl RenderHUD {
  pub fn new() -> Self {
    Self {
      shader: gen_hud_shader(),
    }
  }
  pub fn render(&mut self, _mgr: Box<GameMgr>) -> Box<GameMgr> {
    let mut _mgr = _mgr;
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Gui Render Pass");
    {
      let mut _hud = _mgr.hud.borrow_mut();
      self.shader.start();
      
      let _offset = Vector2f::blank();
      unsafe {
        BindVertexArray(_hud.quad_id);
        EnableVertexAttribArray(0);
        for gui in &mut _hud.elements {
          // Texture!
          ActiveTexture(TEXTURE0);
          BindTexture(TEXTURE_2D, gui.tex_id);
          ActiveTexture(TEXTURE1);
          BindTexture(TEXTURE_2D, gui.depth_tex_id);
          // Shader Vars!
          self.shader.load_matrix("u_Transform", gui.transformation());
          self.shader.load_float("row_count", gui.row_count);
          self.shader.load_vec_2f("offset", &gui.offset);
          self.shader.load_bool("flip_y", gui.flip_y);
          // Draw!
          DrawArrays(TRIANGLE_STRIP, 0, 4_i32);
        }
        DisableVertexAttribArray(0);
        BindVertexArray(0);
      }
      self.shader.stop();
      unsafe {
        Disable(BLEND);
        Enable(DEPTH_TEST);
      }
    }
    _mgr
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}
