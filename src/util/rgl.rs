
use {
  std::{
    // mem,
    ptr,
  },
  gl::{
    *,
  },
  crate::{
    ecs::{
      c::{
        flags::MultiTex,
        components::*,
      },
      resource::{ Texture, ParticleVBO, },
    },
    util::{
      Matrix4f, 
      // HashMap,
    },
  },
};

pub use {
  gl::{
    ARRAY_BUFFER, 
    STREAM_DRAW, STATIC_DRAW, 
    FLOAT, 
    TRUE, FALSE, 
    types::{
      GLenum, GLuint, GLsizeiptr, GLfloat, GLboolean, GLint, GLintptr, 
    },
  },
};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct VaoID(pub u32);
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct VboID(pub u32);
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct VertexCount(pub i32);
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TextureID(pub u32);
#[derive(Copy, Clone, Default, Debug, PartialEq)]
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
pub fn r_bind_vertex_array(vao_id: VaoID) { unsafe {
  BindVertexArray(vao_id.0);
}}
pub fn r_unbind_vertex_array() {
  r_bind_vertex_array(VaoID(0));
}
pub fn r_enable_vertex_attrib_array(num: GLuint) { unsafe {
  EnableVertexAttribArray(num);
}}
pub fn r_disable_vertex_attrib_array(num: GLuint) { unsafe {
  DisableVertexAttribArray(num);
}}
pub fn r_bind_vaa_7(vao_id: VaoID) {
  r_bind_vertex_array(vao_id);
  // print!(" r_bind_vaa_7(model: {})", model.vao_id.0);
  r_enable_vertex_attrib_array(0);
  r_enable_vertex_attrib_array(1);
  r_enable_vertex_attrib_array(2);
  r_enable_vertex_attrib_array(3);
  r_enable_vertex_attrib_array(4);
  r_enable_vertex_attrib_array(5);
  r_enable_vertex_attrib_array(6);
}
pub fn r_bind_vaa_3(vao_id: VaoID) {
  r_bind_vertex_array(vao_id);
  // print!(" r_bind_vaa_3(model: {})", model.vao_id.0);
  r_enable_vertex_attrib_array(0);
  r_enable_vertex_attrib_array(1);
  r_enable_vertex_attrib_array(2);
}
pub fn r_bind_vaa_2(vao_id: VaoID) {
  r_bind_vertex_array(vao_id);
  r_enable_vertex_attrib_array(0);
  r_enable_vertex_attrib_array(1);
}
pub fn r_bind_vaa_1(vao_id: VaoID) {
  r_bind_vertex_array(vao_id);
  r_enable_vertex_attrib_array(0);
}
pub fn r_unbind_vaa_7() { unsafe {
  DisableVertexAttribArray(6);
  DisableVertexAttribArray(5);
  DisableVertexAttribArray(4);
  DisableVertexAttribArray(3);
  DisableVertexAttribArray(2);
  DisableVertexAttribArray(1);
  DisableVertexAttribArray(0);
  BindVertexArray(0);
}}
pub fn r_unbind_vaa_3() { unsafe {
  DisableVertexAttribArray(2);
  DisableVertexAttribArray(1);
  DisableVertexAttribArray(0);
  BindVertexArray(0);
}}
pub fn r_unbind_vaa_2() { unsafe {
  DisableVertexAttribArray(1);
  DisableVertexAttribArray(0);
  BindVertexArray(0);
}}
pub fn r_unbind_vaa_1() { unsafe {
  DisableVertexAttribArray(0);
  BindVertexArray(0);
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
pub fn r_bind_buffer(btype: GLenum, id: VboID) {
  unsafe { BindBuffer(btype, id.0); }
}
pub fn r_buffer_data(target: GLenum, size: GLsizeiptr, data: Option<&Vec<f32>>, usage: GLenum) { unsafe {
  match data {
    Some(data) => { BufferData(target, size, data.as_ptr() as *const _, usage); }
    None =>       { BufferData(target, size, ptr::null(), usage); }
  }
}}
pub fn r_buffer_sub_data(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: Option<&Vec<f32>> ) { unsafe {
  match data {
    Some(data) => { BufferSubData(target, offset, size, data.as_ptr() as *const _ ); }
    None =>       { BufferSubData(target, offset, size, ptr::null() ); }
  }
}}
pub fn r_vertex_attrib_pointer(index: GLuint, size: GLint, gltype: GLenum, normalized: GLboolean, stride: types::GLsizei, pointer: i32, ) { unsafe {
  VertexAttribPointer(index, size, gltype, normalized, stride, pointer as *const _ );
}}
pub fn r_vertex_attrib_divisor(attrib: u32, num: u32) { unsafe {
  VertexAttribDivisor(attrib, num);
}}
pub fn r_add_instanced_attrib(vao: VaoID, vbo: VboID, attrib: u32, data_size: i32, stride: usize, offset: usize) { unsafe {
  // This is my best guess from LWJGL to Rust's GL implementation
  r_get_errors("r_add_instanced_attrib 1");
  BindVertexArray(vao.0);
  r_get_errors("r_add_instanced_attrib 2");
  BindBuffer(ARRAY_BUFFER, vbo.0);
  r_get_errors("r_add_instanced_attrib 3");
  let offset: *const i32 = &(glfloat(offset) as i32);
  VertexAttribPointer(
    attrib, data_size, FLOAT, FALSE, 
    glfloat(stride) as i32, offset as *const _
  );
  r_get_errors("r_add_instanced_attrib 4");
  VertexAttribDivisor(attrib, 1);
  r_get_errors("r_add_instanced_attrib 5");
  BindBuffer(ARRAY_BUFFER, 0);
  r_get_errors("r_add_instanced_attrib 6");
  BindVertexArray(0);
  r_get_errors("r_add_instanced_attrib 7");
}}
pub fn r_update_instanced_attrib(pvbo: &ParticleVBO) { unsafe {
  // This is my best guess from LWJGL to Rust's GL implementation
  BindBuffer(ARRAY_BUFFER, pvbo.vbo_id.0);
  r_get_errors("r_update_instanced_attrib 2");
  // BindVertexArray(pvbo.quad.vao_id.0);
  // r_get_errors("r_update_instanced_attrib 3");
  let stride = glfloat(pvbo.instance_data_length) as i32;
  let offset: *const i32 = &(glfloat(0) as i32);
  VertexAttribPointer(
    1, 1, FLOAT, FALSE, 
    stride, offset as *const _
  );
  let offset: *const i32 = &(glfloat(1) as i32);
  VertexAttribPointer(
    2, 4, FLOAT, FALSE, 
    stride, offset as *const _
  );
  let offset: *const i32 = &(glfloat(5) as i32);
  VertexAttribPointer(
    3, 4, FLOAT, FALSE, 
    stride, offset as *const _
  );
  let offset: *const i32 = &(glfloat(9) as i32);
  VertexAttribPointer(
    4, 4, FLOAT, FALSE, 
    stride, offset as *const _
  );
  let offset: *const i32 = &(glfloat(13) as i32);
  VertexAttribPointer(
    5, 4, FLOAT, FALSE, 
    stride, offset as *const _
  );
  let offset: *const i32 = &(glfloat(17) as i32);
  VertexAttribPointer(
    6, 4, FLOAT, FALSE, 
    stride, offset as *const _
  );
  r_get_errors("r_update_instanced_attrib 4");
  VertexAttribDivisor(1, 1);
  VertexAttribDivisor(2, 1);
  VertexAttribDivisor(3, 1);
  VertexAttribDivisor(4, 1);
  VertexAttribDivisor(5, 1);
  VertexAttribDivisor(6, 1);
  r_get_errors("r_update_instanced_attrib 5");
}}
pub fn r_create_vbo(vbo_id: VboID, buffer_size: usize) { unsafe {
  r_get_errors("r_create_vbo 1");
  BindBuffer(ARRAY_BUFFER, vbo_id.0);
  r_get_errors("r_create_vbo 2");
  BufferData(ARRAY_BUFFER, glfloat(buffer_size) as GLsizeiptr,
      ptr::null(), DYNAMIC_DRAW);
  r_get_errors("r_create_vbo 3");
  BindBuffer(ARRAY_BUFFER, 0);
  r_get_errors("r_create_vbo 4");
}}
pub fn r_update_vbo(vbo: VboID, data: Vec<f32>, max: usize) { unsafe {
  // This is my best guess from LWJGL to Rust's GL implementation
  r_get_errors("r_update_vbo 1");
  BindBuffer(ARRAY_BUFFER, vbo.0); r_get_errors("r_update_vbo 2");
  let max_len = glfloat(max) as GLsizeiptr;
  let data_len = glfloat(data.len()) as GLsizeiptr;
  BufferData(ARRAY_BUFFER,
      max_len,
      std::ptr::null(),
      // data.as_ptr() as *const _,
      DYNAMIC_DRAW); r_get_errors("r_update_vbo 3");
  BufferSubData(ARRAY_BUFFER, 
      0, data_len,
      // std::ptr::null(),
      data.as_ptr() as *const _, 
      // &data[0] as *const _, 
  ); r_get_errors("r_update_vbo 4");
  // BindBuffer(ARRAY_BUFFER, 0);
  // r_get_errors("r_update_vbo 5");
}}

// Clear methods
pub fn r_clear_particle_fbo() { unsafe {
  Disable(DEPTH_TEST);
  Clear(COLOR_BUFFER_BIT);
  ClearColor(0.0, 0.0, 0.0, 0.0);
  BlendFuncSeparate(SRC_ALPHA, ONE_MINUS_SRC_ALPHA, ONE, ONE_MINUS_SRC_ALPHA);
}}

// Draw methods
pub fn r_draw_triangles(vertex_count: VertexCount) { unsafe {
  DrawElements(TRIANGLES, vertex_count.0, UNSIGNED_INT, std::ptr::null()); 
  r_get_errors("r_draw_triangles 1");
}}
pub fn r_draw_triangle_strip(vertex_count: VertexCount) { unsafe {
  DrawElements(TRIANGLE_STRIP, vertex_count.0, UNSIGNED_INT, std::ptr::null()); 
  r_get_errors("r_draw_triangle_strip 1");
}}
pub fn r_draw_arrays_triangle_strip(vertex_count: VertexCount) { unsafe {
  DrawArrays(TRIANGLE_STRIP, 0, vertex_count.0); 
  r_get_errors("r_draw_arrays_triangle_strip 1");
}}
pub fn r_draw_instanced(vertex_count: VertexCount, particle_count: u32) { unsafe {
  DrawArraysInstanced(TRIANGLE_STRIP, 0, vertex_count.0, particle_count as i32);
  // DrawArrays(TRIANGLE_STRIP, 0, vertex_count.0);
  r_get_errors("r_draw_instanced 1");
}}

pub fn glfloat(n: usize) -> usize {
  n * std::mem::size_of::<GLfloat>()
}

// Debugging
pub fn r_get_errors(msg: &str) { unsafe {
  let mut error = GetError();
  while error != NO_ERROR {
    println!("GL Error {}: {}", error, msg);
    error = GetError();
  }
}}

pub enum RBlend {
  DefaultBlend,
  AdditiveBlend,
}
impl RBlend {
  pub fn exec(&self) { unsafe {
    match self {
      RBlend::DefaultBlend => {
        // println!("Default Blend");
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      }
      RBlend::AdditiveBlend => {
        // println!("Additive Blend");
        BlendFunc(SRC_ALPHA, ONE);
      }
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