
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use CVOID;

// use camera::Camera;
use entities::position::PosMarker;
use gamemgr::GameMgr;
use model::RawModel;
// use shader::lighting::Lights;
use shader::gen_font_shader;
use shader::Shader;
use text::{TextMgr, RFontType};
use texture::Texture;
use util::rvector::{Vector3f, }; // Vector2f, Vector4f, 
// use util::rvertex::{RVertex, RVertex2D};

pub struct RenderFont {
  pub shader: Shader,
}

impl RenderFont {
  pub fn new() -> Self {
    Self {
      shader: gen_font_shader(),
    }
  }
  pub fn render(&mut self, mgr: GameMgr) {
    let mut mgr = mgr;
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Text Render Pass");
    let _textmgr = mgr.textmgr.take().unwrap();
    let mut textmgr = _textmgr.lock().unwrap();
    self.shader.start();
    let _tmp: HashMap<String, HashSet<String>> = (*textmgr).active_text.clone();
    let fonts: Vec<&String> = _tmp.keys().clone().into_iter().collect();
    for font in fonts {
      let tex_id = match textmgr.fonts.get_mut(font) {
        Some(x) => {
          let texs = mgr.textures.lock().unwrap();
          match texs.get(&x.tex_atlas) {
            Some(tid) => { tid.tex_id }
            _ => { println!("No font atlas texture {}", &x.tex_atlas); continue }
          }
        }
        _ => { println!("No ftype {}", font); continue }
      };
      // println!("tex_id: {}", tex_id);
      unsafe {
        ActiveTexture(TEXTURE0);
        BindTexture(TEXTURE_2D, tex_id);
      }
      for gtexts in textmgr.active_text.get(font) {
        for gtstr in gtexts {
          for gtext in textmgr.texts.get(gtstr) {
            unsafe {
              BindVertexArray(gtext.text_mesh_vao);
              EnableVertexAttribArray(0);
              EnableVertexAttribArray(1);
              self.shader.load_vec_3f("colour", &gtext.colour);
              self.shader.load_vec_2f("translation", &gtext.position);
              DrawArrays(TRIANGLES, 0, gtext.vertex_count as i32);
              DisableVertexAttribArray(0);
              DisableVertexAttribArray(1);
              BindVertexArray(0);
            }
          }
        }
      }
    }
    self.shader.stop();
    unsafe {
      Disable(BLEND);
      Enable(DEPTH_TEST);
    }
  }
  pub fn prep_instance(&mut self, pos: Arc<Mutex<PosMarker>>) {
    let mut marker = pos.lock().unwrap();
    let trans_mat = marker.transformation();
    self.shader.load_matrix("u_Transform", trans_mat);
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}
