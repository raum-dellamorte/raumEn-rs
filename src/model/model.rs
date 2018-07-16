#[allow(non_snake_case)]
#[allow(unused_imports)]

// use glium::Display;
// use glium::texture::CompressedSrgbTexture2d;
// use glium::texture::RawImage2d;
use image;
use model::import::load_obj;
use model::mesh::Mesh;
use std::path::Path;

pub struct Model {
  pub name: String,
  pub mesh: Option<Mesh>,
  //pub texture: , // Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>>,
}

impl Model {
  pub fn new(model_name: &str) -> Self {
    Model {
      name: format!("{}", model_name),
      mesh: None,
      //texture: Err("Texture not loaded."),
    }
  }
  // pub fn load_defaults(&mut self) -> &mut Self {
  //   self.load_default_mesh().load_default_texture(display)
  // }
  // pub fn load_default_mesh(&mut self) -> &mut Self {
  //   self.mesh = match load_obj(&self.name) {
  //     Ok(mesh) => Some(mesh),
  //     Err(_) => {println!("Mesh {} failed to load.", self.name); None },
  //   };
  //   // match self.mesh.as_mut() {
  //   //   Some(mesh) => mesh.create_buffers(display),
  //   //   None => (),
  //   // };
  //   self
  // }
  // pub fn load_default_texture<R, F>(&mut self, factory: &mut F) -> &mut Self {
  //       where R: gfx::Resources, F: gfx::Factory<R> {
  //   use image;
  //   use std::path::Path;
  //   use gfx::format::Rgba8;
  //   use gfx::texture as t;
  //   let path: &str = &format!("src/res/img/{}.png", self.name);
  //   let img = match image::open(&Path::new(path)) {
  //     Ok(image) => {
  //       println!("Image loaded");
  //       image.to_rgba()
  //     },
  //     _ => panic!("Failed to load image")
  //   };
  //   let (width, height) = img.dimensions();
  //   let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
  //   let (_, view) = match factory.create_texture_immutable_u8::<Rgba8>(kind, t::Mipmap::Provided, &[&img]) {
  //     Ok(tex_view) => {
  //       println!("Texture view created");
  //       tex_view
  //     }
  //     _ => panic!("Failed to create texture view.")
  //   };
  //   self.texture =  Ok(view);
  //   self
  // }
  //   let path: &str = &format!("./res/img/{}.png", self.name);
  //   self.texture = match image::open(&Path::new(path)) {
  //     Ok(image) => {
  //       //println!("Image loaded");
  //       let image = image.to_rgba();
  //       let image_dimensions = image.dimensions();
  //       let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
  //       match CompressedSrgbTexture2d::new(display, image) {
  //         Ok(opengl_texture) => Some(opengl_texture),
  //         _ => None,
  //       }
  //     },
  //     _ => None,
  //   };
  //   self
  // }
  // pub fn mesh_verts(&self) -> Vec<Vertex> {
  //   match self.mesh {
  //     Some(mesh) => { mesh.verts }
  //     _ => { Vec::new::<Vertex>() }
  //   }
  // }
}

#[derive(Debug)]
pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: usize,
}

impl RawModel {
  pub fn new(id: u32, count: usize) -> Self {
    RawModel { vao_id: id, vertex_count: count }
  }
}
