
use camera::Camera;
use glium::texture;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::*;
use glium::{Display};

pub struct Fbo {
  tex: Texture2d,
}

pub struct FboWithDepth {
  pub tex: Texture2d,
  pub depth: DepthTexture2d,
}

impl FboWithDepth {
  pub fn new(display: &Display, width: u32, height: u32) -> Self {
    let fb_tex = Texture2d::empty_with_format(display, texture::UncompressedFloatFormat::U8U8U8, MipmapsOption::NoMipmap, width, height).unwrap();
    let fb_depth_tex = DepthTexture2d::empty_with_format(display, texture::DepthFormat::I32, MipmapsOption::NoMipmap, width, height).unwrap();
    FboWithDepth {
      tex: fb_tex,
      depth: fb_depth_tex,
    }
  }
  
  pub fn new_default(cam: &Camera) -> Self {
    let (w, h) = cam.get_dimensions();
    FboWithDepth::new(&cam.display, w, h)
  }
  
  pub fn fb(&self, cam: &Camera) -> Option<SimpleFrameBuffer> {
    Some(SimpleFrameBuffer::with_depth_buffer(&cam.display, &self.tex, &self.depth).unwrap())
  }
  
  
}
