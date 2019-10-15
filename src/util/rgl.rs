
use {
  gl::{
    *,
    // types::{ 
    //   // GLuint, GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
    // },
  },
  CVOID,
  ecs::{
    c::{
      flags::MultiTex,
      components::*,
    },
    resource::{ Model, Texture, },
  },
  util::{
    Matrix4f, 
    // HashMap,
  },
};

pub fn r_gen_textures() -> u32 { unsafe {
  let mut id = 0;
  GenTextures(1, &mut id);
  id
}}
pub fn r_gen_buffers() -> u32 { unsafe {
  let mut id = 0;
  GenBuffers(1, &mut id);
  id
}}
pub fn r_gen_framebuffers() -> u32 { unsafe {
  let mut id = 0;
  GenFramebuffers(1, &mut id);
  id
}}
pub fn r_gen_renderbuffers() -> u32 { unsafe {
  let mut id = 0;
  GenRenderbuffers(1, &mut id);
  id
}}
pub fn r_gen_vertex_arrays() -> u32 { unsafe {
  let mut id = 0;
  GenVertexArrays(1, &mut id);
  id
}}

#[derive(Debug)]
pub struct VaoID(pub u32);
#[derive(Debug)]
pub struct VertexCount(pub i32);
#[derive(Debug)]
pub struct TextureID(pub u32);
#[derive(Debug)]
pub struct TextureUnit(pub i32);

pub fn r_bind_vaa_3(model: &Model) { unsafe {
  BindVertexArray(model.vao_id.0);
  // print!(" r_bind_vaa_3(model: {})", model.vao_id.0);
  EnableVertexAttribArray(0);
  EnableVertexAttribArray(1);
  EnableVertexAttribArray(2);
}}
pub fn r_bind_vaa_2(model: &Model) { unsafe {
  BindVertexArray(model.vao_id.0);
  EnableVertexAttribArray(0);
  EnableVertexAttribArray(1);
}}
pub fn r_unbind_vaa_3() { unsafe {
  DisableVertexAttribArray(2);
  DisableVertexAttribArray(1);
  DisableVertexAttribArray(0);
}}
pub fn r_unbind_vaa_2() { unsafe {
  DisableVertexAttribArray(1);
  DisableVertexAttribArray(0);
}}
pub fn r_bind_texture(texture: &Texture) { unsafe {
  let tex_id = texture.tex_id.0;
  let mut tex_unit = texture.tex_unit.0;
  if tex_unit < 0 { tex_unit = 0 };
  ActiveTexture(TEXTURE0 + tex_unit as u32);
  BindTexture(TEXTURE_2D, tex_id);
  // print!(" r_bind_texture(texture: {})", texture.tex_id.0)
}}
pub fn r_draw_triangles(model: &Model) { unsafe {
  DrawElements(TRIANGLES, model.vertex_count.0, UNSIGNED_INT, CVOID); 
}}


pub struct DrawModelsWithTextures(pub Vec<DrawModelWithTextures>);
impl Default for DrawModelsWithTextures {
  fn default() -> Self { Self(Vec::new()) }
}
impl DrawModelsWithTextures {
  pub fn index_of(&self, name: &str) -> i32 {
    for (i, x) in self.0.iter().enumerate() {
      if x.0 == name { return i as i32 }
    }
    -1
  }
  pub fn push(&mut self, name: &str) -> &mut DrawModelWithTextures {
    let n = self.0.len();
    self.0.push(DrawModelWithTextures::new(name));
    &mut self.0[n]
  }
  pub fn clear(&mut self) {
    self.0.clear();
  }
}
pub struct DrawModelWithTextures(pub String, pub Vec<DrawModelTextureWithAttribs>);
impl DrawModelWithTextures {
  pub fn new(name: &str) -> Self {
    Self(name.to_owned(), Vec::new())
  }
  pub fn index_of(&self, texture: &str, lighting: &str) -> i32 {
    for (i, x) in self.1.iter().enumerate() {
      if (x.0 == texture) && (x.1 == lighting) { return i as i32 }
    }
    -1
  }
  pub fn push(&mut self, texture: &str, lighting: &str) -> &mut DrawModelTextureWithAttribs {
    let n = self.1.len();
    self.1.push(DrawModelTextureWithAttribs::new(texture, lighting));
    &mut self.1[n]
  }
}
pub struct DrawModelTextureWithAttribs(pub String, pub String, pub Vec<ModelTextureAttribs>);
impl DrawModelTextureWithAttribs {
  pub fn new(texture: &str, lighting: &str) -> Self {
    Self(texture.to_owned(), lighting.to_owned(), Vec::new())
  }
  pub fn push(&mut self, attribs: ModelTextureAttribs) {
    self.2.push(attribs);
  }
}
pub struct ModelTextureAttribs {
  pub transform: Matrix4f<f32>,
  pub tex_index: Option<TexIndex>,
  pub row_count: Option<RowCount>,
  pub offset: Option<TexOffset>,
  pub multi_tex: Option<MultiTex>,
}