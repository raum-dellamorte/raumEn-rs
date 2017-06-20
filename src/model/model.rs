
use glium::Display;
use glium::texture::CompressedSrgbTexture2d;
use glium::texture::RawImage2d;
use image;
use model::import::load_obj;
use model::mesh::Mesh;
use std::path::Path;

pub struct Model {
  pub name: String,
  pub mesh: Option<Mesh>,
  pub texture: Option<CompressedSrgbTexture2d>,
}

impl Model {
  pub fn new(model_name: &str) -> Self {
    Model {
      name: format!("{}", model_name),
      mesh: None,
      texture: None,
    }
  }
  
  pub fn load_defaults(&mut self, display: &Display) -> &mut Self {
    self.load_default_mesh(display).load_default_texture(display)
  }
  
  pub fn load_default_mesh(&mut self, display: &Display) -> &mut Self {
    self.mesh = match load_obj(&self.name) {
      Ok(mesh) => Some(mesh),
      Err(_) => {println!("Mesh {} failed to load.", self.name); None },
    };
    match self.mesh.as_mut() {
      Some(mesh) => mesh.create_buffers(display),
      None => (),
    };
    self
  }
  
  pub fn load_default_texture(&mut self, display: &Display) -> &mut Self {
    let path: &str = &format!("./res/img/{}.png", self.name);
    self.texture = match image::open(&Path::new(path)) {
      Ok(image) => {
        //println!("Image loaded");
        let image = image.to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        match CompressedSrgbTexture2d::new(display, image) {
          Ok(opengl_texture) => Some(opengl_texture),
          _ => None,
        }
      },
      _ => None,
    };
    self
  }
}
