
use {
  gl::{
    *,
    types::*,
  },
  ecs::{
    c::{
      flags::MultiTex,
      components::*,
    },
    resource::{ Texture, },
  },
  util::{
    Matrix4f, 
    // HashMap,
  },
};

#[derive(Copy, Clone, Default, Debug)]
pub struct VaoID(pub u32);
#[derive(Copy, Clone, Default, Debug)]
pub struct VboID(pub u32);
#[derive(Copy, Clone, Default, Debug)]
pub struct VertexCount(pub i32);
#[derive(Copy, Clone, Default, Debug)]
pub struct TextureID(pub u32);
#[derive(Copy, Clone, Default, Debug)]
pub struct TextureUnit(pub i32);

// Gen methods
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

// Vertex attrib bindings
pub fn r_bind_vaa_7(vao_id: VaoID) { unsafe {
  BindVertexArray(vao_id.0);
  // print!(" r_bind_vaa_3(model: {})", model.vao_id.0);
  EnableVertexAttribArray(0);
  EnableVertexAttribArray(1);
  EnableVertexAttribArray(2);
  EnableVertexAttribArray(3);
  EnableVertexAttribArray(4);
  EnableVertexAttribArray(5);
  EnableVertexAttribArray(6);
}}
pub fn r_bind_vaa_3(vao_id: VaoID) { unsafe {
  BindVertexArray(vao_id.0);
  // print!(" r_bind_vaa_3(model: {})", model.vao_id.0);
  EnableVertexAttribArray(0);
  EnableVertexAttribArray(1);
  EnableVertexAttribArray(2);
}}
pub fn r_bind_vaa_2(vao_id: VaoID) { unsafe {
  BindVertexArray(vao_id.0);
  EnableVertexAttribArray(0);
  EnableVertexAttribArray(1);
}}
pub fn r_unbind_vaa_7() { unsafe {
  DisableVertexAttribArray(6);
  DisableVertexAttribArray(5);
  DisableVertexAttribArray(4);
  DisableVertexAttribArray(3);
  DisableVertexAttribArray(2);
  DisableVertexAttribArray(1);
  DisableVertexAttribArray(0);
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

// Texture tools
pub fn r_bind_texture(texture: &Texture) { unsafe {
  let tex_id = texture.tex_id.0;
  let mut tex_unit = texture.tex_unit.0;
  if tex_unit < 0 { tex_unit = 0 };
  ActiveTexture(TEXTURE0 + tex_unit as u32);
  BindTexture(TEXTURE_2D, tex_id);
  // print!(" r_bind_texture(texture: {})", texture.tex_id.0)
}}

// VBO tools
pub fn r_add_instanced_attrib(vao: VaoID, vbo: VboID, attrib: u32, data_size: i32, stride: usize, offset: i32) { unsafe {
  // This is my best guess from LWJGL to Rust's GL implementation
  BindBuffer(ARRAY_BUFFER, vbo.0);
  BindVertexArray(vao.0);
  let offset = offset * std::mem::size_of::<GLfloat>() as i32;
  let offset: *const i32 = &offset;
  let offset = offset as *const std::ffi::c_void;
  VertexAttribPointer(
    attrib, data_size, FLOAT, FALSE, 
    (stride * std::mem::size_of::<GLfloat>()) as i32, offset
  );
  VertexAttribDivisor(attrib, 1);
  BindBuffer(ARRAY_BUFFER, 0);
  BindVertexArray(0);
}}
pub fn r_update_vbo(vbo: VboID, data: &[GLfloat]) { unsafe {
  // This is my best guess from LWJGL to Rust's GL implementation
  BindBuffer(ARRAY_BUFFER, vbo.0);
  let data_len = (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr;
  BufferData(ARRAY_BUFFER,
      data_len,
      std::ptr::null(),
      STREAM_DRAW);
  BufferSubData(ARRAY_BUFFER, 
      0, data_len,
      &data[0] as *const f32 as *const std::ffi::c_void);
  BindBuffer(ARRAY_BUFFER, 0);
}}

pub fn r_draw_triangles(vertex_count: VertexCount) { unsafe {
  DrawElements(TRIANGLES, vertex_count.0, UNSIGNED_INT, std::ptr::null()); 
}}
pub fn r_draw_triangle_strip(vertex_count: VertexCount) { unsafe {
  DrawElements(TRIANGLE_STRIP, vertex_count.0, UNSIGNED_INT, std::ptr::null()); 
}}
pub fn r_draw_instanced(vertex_count: VertexCount, particle_count: u32) { unsafe {
  DrawArraysInstanced(TRIANGLE_STRIP, 0, vertex_count.0, particle_count as i32);
}}

pub enum RBlend {
  DefaultBlend,
  AdditiveBlend,
}
impl RBlend {
  pub fn r_blend_func(&self) { unsafe {
    match self {
      RBlend::DefaultBlend => { BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA); }
      RBlend::AdditiveBlend => { BlendFunc(SRC_ALPHA, ONE) }
    }
  }}
}

pub enum GlEnDisIg {
  Enable,
  Disable,
  Ignore,
}
impl Default for GlEnDisIg {
  fn default() -> Self { Self::Ignore }
}

#[derive(Default)]
pub struct GlSettings {
  pub blend: GlEnDisIg,
  pub cull_face: GlEnDisIg,
  pub depth_test: GlEnDisIg,
  pub depth_mask: GlEnDisIg,
  // pub : GlEnDisIg,
}
impl GlSettings {
  pub fn set(&self) { unsafe {
    match self.blend {
      GlEnDisIg::Enable => { Enable(BLEND); }
      GlEnDisIg::Disable => { Disable(BLEND); }
      GlEnDisIg::Ignore => {}
    }
    match self.cull_face {
      GlEnDisIg::Enable => { Enable(CULL_FACE); }
      GlEnDisIg::Disable => { Disable(CULL_FACE); }
      GlEnDisIg::Ignore => {}
    }
    match self.depth_test {
      GlEnDisIg::Enable => { Enable(DEPTH_TEST); }
      GlEnDisIg::Disable => { Disable(DEPTH_TEST); }
      GlEnDisIg::Ignore => {}
    }
    match self.depth_mask {
      GlEnDisIg::Enable => { DepthMask(TRUE); }
      GlEnDisIg::Disable => { DepthMask(FALSE); }
      GlEnDisIg::Ignore => {}
    }
  }}
  pub fn enable_blend(self) -> Self {
    let mut _self = self;
    _self.blend = GlEnDisIg::Enable;
    _self
  }
  pub fn disable_blend(self) -> Self {
    let mut _self = self;
    _self.blend = GlEnDisIg::Disable;
    _self
  }
  pub fn enable_cull_face(self) -> Self {
    let mut _self = self;
    _self.cull_face = GlEnDisIg::Enable;
    _self
  }
  pub fn disable_cull_face(self) -> Self {
    let mut _self = self;
    _self.cull_face = GlEnDisIg::Disable;
    _self
  }
  pub fn enable_depth_test(self) -> Self {
    let mut _self = self;
    _self.depth_test = GlEnDisIg::Enable;
    _self
  }
  pub fn disable_depth_test(self) -> Self {
    let mut _self = self;
    _self.depth_test = GlEnDisIg::Disable;
    _self
  }
  pub fn enable_depth_mask(self) -> Self {
    let mut _self = self;
    _self.depth_mask = GlEnDisIg::Enable;
    _self
  }
  pub fn disable_depth_mask(self) -> Self {
    let mut _self = self;
    _self.depth_mask = GlEnDisIg::Disable;
    _self
  }
}

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