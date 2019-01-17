
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use CVOID;

use Camera;

pub enum DepthType {
  DepthTexture,
  DepthRenderBuffer,
  NoDepth,
}

pub struct Fbo {
  pub width: i32,
  pub height: i32,
  pub frame_buffer_id: u32,
  pub color_tex_id: u32,
  pub depth_tex_id: u32,
  pub color_buf_id: u32,
  pub depth_buf_id: u32,
  pub active: bool,
}

impl Fbo {
  pub fn new(width: i32, height: i32, depth_buffer_type: DepthType) -> Self {
    let out = Fbo {
      width: width,
      height: height,
      frame_buffer_id: 0,
      color_tex_id: 0,
      depth_tex_id: 0,
      color_buf_id: 0,
      depth_buf_id: 0,
      active: true,
    };
    out.initialise_fbo(depth_buffer_type)
  }
  pub fn initialise_fbo(self, depth_buffer_type: DepthType) -> Self {
    use fbo::DepthType::{DepthRenderBuffer, DepthTexture, NoDepth};
    let mut __self = self;
    {
      let _self = &mut __self;
      _self.create_frame_buffer();
      _self.create_color_texture_attachment();
      match depth_buffer_type {
        DepthRenderBuffer => { _self.create_depth_buffer_attachment(); }
        DepthTexture => { _self.create_depth_texture_attachment(); }
        NoDepth => {}
      }
      _self.unbind();
      _self.active = true;
    }
    __self
  }
  pub fn clean_up(&mut self) { unsafe {
    DeleteFramebuffers(1_i32, &self.frame_buffer_id);
    DeleteTextures(1_i32, &self.color_tex_id);
    DeleteTextures(1_i32, &self.depth_tex_id);
    DeleteRenderbuffers(1_i32, &self.color_buf_id);
    DeleteRenderbuffers(1_i32, &self.depth_buf_id);
    self.active = false;
  }}
  pub fn bind(&self) { unsafe {
    BindFramebuffer(DRAW_FRAMEBUFFER, self.frame_buffer_id);
    Viewport(0, 0, self.width, self.height);
  }}
  pub fn unbind(&self) { unsafe {
    BindFramebuffer(FRAMEBUFFER, 0);
    Viewport(0, 0, 1024, 768);
  }}
  pub fn bind_to_read(&self) { unsafe {
    BindTexture(TEXTURE_2D, 0);
    BindFramebuffer(READ_FRAMEBUFFER, self.frame_buffer_id);
    ReadBuffer(COLOR_ATTACHMENT0);
  }}
  fn create_frame_buffer(&mut self) { unsafe {
    let mut fb = 0_u32;
    GenFramebuffers(1, &mut fb);
    if fb == 0_u32 { panic!("GenFramebuffers failed in Fbo::create_frame_buffer") }
    self.frame_buffer_id = fb;
    BindFramebuffer(FRAMEBUFFER, fb);
    DrawBuffer(COLOR_ATTACHMENT0);
  }}
  fn create_color_texture_attachment(&mut self) { unsafe {
    let mut ct = 0_u32;
    GenTextures(1, &mut ct);
    if ct == 0_u32 { panic!("GenTextures failed in Fbo::create_color_texture_attachment") }
    self.color_tex_id = ct;
    BindTexture(TEXTURE_2D, ct);
    TexImage2D(TEXTURE_2D, 0, RGBA8 as i32, self.width, self.height, 0, RGBA, UNSIGNED_BYTE, CVOID);
    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
    FramebufferTexture2D(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, ct, 0);
  }}
  fn create_depth_texture_attachment(&mut self) { unsafe {
    let mut dt = 0_u32;
    GenTextures(1, &mut dt);
    if dt == 0_u32 { panic!("GenTextures failed in Fbo::create_depth_texture_attachment") }
    self.depth_tex_id = dt;
    BindTexture(TEXTURE_2D, dt);
    TexImage2D(TEXTURE_2D, 0, DEPTH_COMPONENT24 as i32, self.width, self.height, 0, DEPTH_COMPONENT, FLOAT, CVOID);
    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
    FramebufferTexture2D(FRAMEBUFFER, DEPTH_ATTACHMENT, TEXTURE_2D, dt, 0);
  }}
  fn create_depth_buffer_attachment(&mut self) { unsafe {
    let mut db = 0_u32;
    GenRenderbuffers(1, &mut db);
    if db == 0_u32 { panic!("GenRenderbuffers failed in Fbo::create_depth_buffer_attachment") }
    self.depth_buf_id = db;
    BindRenderbuffer(RENDERBUFFER, db);
    RenderbufferStorage(RENDERBUFFER, DEPTH_COMPONENT24, self.width, self.height);
    FramebufferRenderbuffer(FRAMEBUFFER, DEPTH_ATTACHMENT, RENDERBUFFER, db);
  }}
}
