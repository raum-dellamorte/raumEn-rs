
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use CVOID;

use {GameMgr, Shader, Texture}; // Lights, Camera, 
use entities::PosMarker;
use model::Model;
use shader::gen_font_shader;
use text::{TextMgr, RFontType};
use util::{Vector3f, HashMap, HashSet, Arc, Mutex, }; // Vector2f, Vector4f, RVertex, RVertex2D

pub struct RenderFont {
  pub shader: Shader,
}

impl RenderFont {
  pub fn new() -> Self {
    Self {
      shader: gen_font_shader(),
    }
  }
  pub fn render(&mut self, mgr: Box<GameMgr>) -> Box<GameMgr> {
    let mut mgr = mgr;
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Text Render Pass");
    let _textmgr = mgr.textmgr.take().unwrap();
    {
      let mut textmgr = _textmgr.borrow_mut();
      self.shader.start();
      let _tmp: HashMap<String, HashSet<String>> = (*textmgr).active_text.clone();
      let fonts: Vec<&String> = _tmp.keys().clone().into_iter().collect();
      use util::rgl::r_bind_texture;
      for font in fonts {
        match textmgr.fonts.get_mut(font) {
          Some(x) => {
            let texs = mgr.textures.borrow_mut();
            match texs.get(&x.tex_atlas) {
              Some(tid) => { 
                // println!("tex_id: {}", tex_id);
                r_bind_texture(&tid); 
              }
              _ => { println!("No font atlas texture {}", &x.tex_atlas); continue }
            }
          }
          _ => { println!("No ftype {}", font); continue }
        };
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
    mgr.textmgr = Some(_textmgr);
    mgr
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}
