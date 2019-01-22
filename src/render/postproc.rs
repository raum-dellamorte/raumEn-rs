
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use CVOID;

// use Camera;
use entities::PosMarker;
use GameMgr;
use model::RawModel;
// use Lights;
use shader::gen_postproc_shader;
use Shader;
use text::{TextMgr, RFontType};
use Texture;
use util::{Vector3f, Vector2f}; // Vector2f, Vector4f, RVertex, RVertex2D

pub struct RenderPostProc {
  pub shader: Shader,
}

impl RenderPostProc {
  pub fn new() -> Self {
    Self {
      shader: gen_postproc_shader(),
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
      // todo: switch from HUD to fullscreen texture to process
      let mut _hud = _mgr.hud.borrow_mut();
      self.shader.start();
      
      let _offset = Vector2f::blank();
      unsafe {
        BindVertexArray(_hud.quad_id);
        EnableVertexAttribArray(0);
        for gui in &mut _hud.elements {
          // This took me forever to solve.  I could only get TEXTURE0 to work. TEXTURE1 was being ignored.
          // I eventually found out that if you have more that one texture you have to declare which
          // uniform sampler is which with Uniform1i(), that's i at the end for integer, not f for float.
          // So I added add_sampler_uniforms() to Shader for the gen_x_shader() functions to specify
          // which uniforms are texture samplers and what TEXTURE unit to attach them to, and then added
          // load_sampler_uniforms() to do all the Uniform1i() stuff automatically for any uniform with a
          // texture value greater than -1, and put that function inside load_defaults(), which was already
          // being called in all the gen_x_shader() functions.  The end result is that I can declare the 
          // regular uniforms as normal and declare the sampler uniforms with their TEXTURE number and it
          // all just works.  When I switch to using (probably) JSON files (or XML or whatever) to load all
          // the game details, half the work is done because Shader already turns lists of strings into
          // all the glsl variable connections I need.
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