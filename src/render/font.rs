
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
// use CVOID;

use {
  specs::World,
  // Camera, 
  // GameMgr, 
  // Lights, 
  Shader, 
  // Texture, 
  // entities::PosMarker,
  // model::Model,
  Textures,
  TextMgr,
  shader::gen_font_shader,
  // text::{
  //   // TextMgr, 
  //   // RFontType,
  // },
  util::{
    HashMap, 
    HashSet, 
    // Arc, 
    // Mutex,
    // Vector3f, 
    // Vector2f, 
    // Vector4f, 
    // RVertex, 
    // RVertex2D
  }, 
};

pub struct RenderFont {
  pub shader: Shader,
}
impl Default for RenderFont {
  fn default() -> Self {
    Self {
      shader: gen_font_shader(),
    }
  }
}
impl RenderFont {
  pub fn new() -> Self {
    Self::default()
  }
  // pub fn render(&mut self, mgr: Box<GameMgr>) -> Box<GameMgr> {
  pub fn render(&mut self, world: &World) {
    // let mut mgr = mgr;
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Text Render Pass");
    {
      let mut textmgr = world.write_resource::<TextMgr>();
      self.shader.start();
      let _tmp: HashMap<String, HashSet<String>> = (*textmgr).active_text.clone();
      let fonts: Vec<&String> = _tmp.keys().clone().collect();
      use util::rgl::r_bind_texture;
      for font in fonts {
        match textmgr.fonts.get_mut(font) {
          Some(x) => {
            let texs = world.write_resource::<Textures>();
            match texs.0.get(&x.tex_atlas) {
              Some(tid) => { 
                // println!("tex_id: {}", tex_id);
                r_bind_texture(&tid); 
              }
              _ => { println!("No font atlas texture {}", &x.tex_atlas); continue }
            }
          }
          _ => { println!("No ftype {}", font); continue }
        };
        if let Some(gtexts) = textmgr.active_text.get(font) {
          for gtstr in gtexts {
            if let Some(gtext) = textmgr.texts.get(gtstr) {
              unsafe {
                BindVertexArray(gtext.text_mesh_vao);
                EnableVertexAttribArray(0);
                EnableVertexAttribArray(1);
                self.shader.load_vec_3f("colour", gtext.colour);
                self.shader.load_vec_2f("translation", gtext.position);
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
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}
