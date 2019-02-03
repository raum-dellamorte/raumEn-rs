
use util::{Rc, RefCell};

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use CVOID;

use Display;
use util::rgl::*;

pub enum ColorType {
  ColorTexture,
  ColorMultisampleRenderBuffer,
  ColorMultisampleRenderBuffers2,
  NoColor,
}
pub enum DepthType {
  DepthTexture,
  DepthRenderBuffer,
  NoDepth,
}

use engine::fbo::ColorType::{ColorTexture, ColorMultisampleRenderBuffer, ColorMultisampleRenderBuffers2, NoColor};
use engine::fbo::DepthType::{DepthTexture, DepthRenderBuffer, NoDepth};

pub struct Fbo {
  pub display: Rc<RefCell<Display>>,
  pub width: i32,
  pub height: i32,
  pub color_type: ColorType,
  pub depth_type: DepthType,
  pub frame_buffer_id: u32,
  pub color_tex_id: u32,
  pub depth_tex_id: u32,
  pub color_buf_id: u32,
  pub color_buf_id_2: u32,
  pub depth_buf_id: u32,
  pub active: bool,
}

impl Fbo {
  pub fn new(display: Rc<RefCell<Display>>, width: i32, height: i32, color_type: ColorType, depth_type: DepthType) -> Self {
    let (w, h) = display.borrow().dimensions();
    let mut out = Fbo {
      display: display,
      width: if width == 0 { w as i32 } else { width },
      height: if height == 0 { h as i32 } else { height },
      color_type: color_type,
      depth_type: depth_type,
      frame_buffer_id: 0,
      color_tex_id: 0,
      depth_tex_id: 0,
      color_buf_id: 0,
      color_buf_id_2: 0,
      depth_buf_id: 0,
      active: true,
    };
    {
      let _self = &mut out;
      _self.create_frame_buffer();
      match _self.color_type {
        ColorTexture => { _self.create_color_texture_attachment(); }
        ColorMultisampleRenderBuffer => { _self.create_multisample_color_buffer_attachment(1); }
        ColorMultisampleRenderBuffers2 => {
          _self.create_multisample_color_buffer_attachment(1);
          _self.create_multisample_color_buffer_attachment(2);
        }
        NoColor => {}
      }
      match _self.depth_type {
        DepthTexture => { _self.create_depth_texture_attachment(); }
        DepthRenderBuffer => { _self.create_depth_buffer_attachment(); }
        NoDepth => {}
      }
      _self.unbind();
      _self.active = true;
    }
    out
  }
  pub fn clean_up(&mut self) { unsafe {
    DeleteFramebuffers(1_i32, &self.frame_buffer_id);
    DeleteTextures(1_i32, &self.color_tex_id);
    DeleteTextures(1_i32, &self.depth_tex_id);
    DeleteRenderbuffers(1_i32, &self.color_buf_id);
    DeleteRenderbuffers(1_i32, &self.color_buf_id_2);
    DeleteRenderbuffers(1_i32, &self.depth_buf_id);
    self.active = false;
  }}
  pub fn bind(&self) { unsafe {
    BindFramebuffer(DRAW_FRAMEBUFFER, self.frame_buffer_id);
    Viewport(0, 0, self.width, self.height);
  }}
  pub fn unbind(&self) { unsafe {
    let (w, h) = self.display.borrow().dimensions();
    BindFramebuffer(FRAMEBUFFER, 0);
    Viewport(0, 0, w as i32, h as i32);
  }}
  pub fn bind_to_read(&self) { unsafe {
    BindTexture(TEXTURE_2D, 0);
    BindFramebuffer(READ_FRAMEBUFFER, self.frame_buffer_id);
    ReadBuffer(COLOR_ATTACHMENT0);
  }}
  pub fn blit_to_fbo(&self, color_attachment: u32, other: &Self) { unsafe {
    BindFramebuffer(DRAW_FRAMEBUFFER, other.frame_buffer_id);
    BindFramebuffer(READ_FRAMEBUFFER, self.frame_buffer_id);
    ReadBuffer(COLOR_ATTACHMENT0 + color_attachment);
    BlitFramebuffer(0, 0, self.width, self.height, 0, 0, other.width, other.height, 
        COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT, NEAREST);
    self.unbind();
  }}
  pub fn blit_to_screen(&self) { unsafe {
    let (w, h) = self.display.borrow().dimensions();
    BindFramebuffer(DRAW_FRAMEBUFFER, 0);
    BindFramebuffer(READ_FRAMEBUFFER, self.frame_buffer_id);
    BlitFramebuffer(0, 0, self.width, self.height, 0, 0, w as i32, h as i32, COLOR_BUFFER_BIT, NEAREST);
    self.unbind();
  }}
  fn create_frame_buffer(&mut self) { unsafe {
    let id = r_gen_framebuffers();
    if id == 0_u32 { panic!("GenFramebuffers failed in Fbo::create_frame_buffer") }
    self.frame_buffer_id = id;
    BindFramebuffer(FRAMEBUFFER, id);
    let mut buffers = vec![COLOR_ATTACHMENT0];
    match self.color_type {
      ColorMultisampleRenderBuffers2 => { buffers.push(COLOR_ATTACHMENT1); }
      _ => {}
    }
    use std::mem;
    DrawBuffers(buffers.len() as i32, mem::transmute(&buffers[0]));
  }}
  fn create_color_texture_attachment(&mut self) { unsafe {
    let id = r_gen_textures();
    if id == 0_u32 { panic!("GenTextures failed in Fbo::create_color_texture_attachment") }
    self.color_tex_id = id;
    BindTexture(TEXTURE_2D, id);
    TexImage2D(TEXTURE_2D, 0, RGBA8 as i32, self.width, self.height, 0, RGBA, UNSIGNED_BYTE, CVOID);
    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
    FramebufferTexture2D(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, id, 0);
  }}
  fn create_depth_texture_attachment(&mut self) { unsafe {
    let id = r_gen_textures();
    if id == 0_u32 { panic!("GenTextures failed in Fbo::create_depth_texture_attachment") }
    self.depth_tex_id = id;
    BindTexture(TEXTURE_2D, id);
    TexImage2D(TEXTURE_2D, 0, DEPTH_COMPONENT24 as i32, self.width, self.height, 0, DEPTH_COMPONENT, FLOAT, CVOID);
    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
    TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
    FramebufferTexture2D(FRAMEBUFFER, DEPTH_ATTACHMENT, TEXTURE_2D, id, 0);
  }}
  fn create_multisample_color_buffer_attachment(&mut self, num: isize) { unsafe {
    let id = r_gen_renderbuffers();
    if id == 0_u32 { panic!("GenRenderbuffers failed in Fbo::create_depth_buffer_attachment") }
    let color_attachment = match num {
      1 => { self.color_buf_id = id; COLOR_ATTACHMENT0 }
      2 => { self.color_buf_id_2 = id; COLOR_ATTACHMENT1 }
      _ => { COLOR_ATTACHMENT0 }
    };
    BindRenderbuffer(RENDERBUFFER, id);
    RenderbufferStorageMultisample(RENDERBUFFER, 4, RGBA8, self.width, self.height);
    FramebufferRenderbuffer(FRAMEBUFFER, color_attachment, RENDERBUFFER, id);
  }}
  fn create_depth_buffer_attachment(&mut self) { unsafe {
    let id = r_gen_renderbuffers();
    if id == 0_u32 { panic!("GenRenderbuffers failed in Fbo::create_depth_buffer_attachment") }
    self.depth_buf_id = id;
    BindRenderbuffer(RENDERBUFFER, id);
    match self.color_type {
      ColorMultisampleRenderBuffer | ColorMultisampleRenderBuffers2 => { RenderbufferStorageMultisample(RENDERBUFFER, 4, DEPTH_COMPONENT24, self.width, self.height); }
      _ => { RenderbufferStorage(RENDERBUFFER, DEPTH_COMPONENT24, self.width, self.height); }
    }
    FramebufferRenderbuffer(FRAMEBUFFER, DEPTH_ATTACHMENT, RENDERBUFFER, id);
  }}
}
