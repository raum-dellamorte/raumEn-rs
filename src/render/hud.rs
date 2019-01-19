
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
use shader::gen_hud_shader;
use Shader;
use text::{TextMgr, RFontType};
use Texture;
use util::{Vector3f, Vector2f}; // Vector2f, Vector4f, RVertex, RVertex2D

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
          BindTexture(TEXTURE_2D, gui.depth_tex_id);
          ActiveTexture(TEXTURE1);
          BindTexture(TEXTURE_2D, gui.tex_id);
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
    // mgr.textmgr = Some(_textmgr);
    _mgr
  }
  // fn use_material(mgr: &mut GameMgr, shader: &Shader, material: &str) {
  //   let (lighting, texture) = {
  //     let _mat = mgr.material(material);
  //     let material = _mat.borrow_mut();
  //     shader.load_float("row_count", material.row_count as f32); // float numOfRows
  //     shader.load_vec_2f("offset", &material.offset); // vec2 offset;
  //     (&material.lighting.clone(), &material.texture.clone())
  //   };
  //   {
  //     let lighting = mgr.lighting(lighting);
  //     lighting.borrow_mut().load_to_shader(shader);
  //   }
  //   {
  //     let texture = mgr.texture(texture);
  //     Self::bind_texture(&texture);
  //   }
  // }
  // fn bind_texture(texture: &Texture) { unsafe {
  //   ActiveTexture(TEXTURE0);
  //   BindTexture(TEXTURE_2D, texture.tex_id);
  // }}
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}
