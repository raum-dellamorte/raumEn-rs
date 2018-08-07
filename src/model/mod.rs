#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod mesh;
pub mod import;
pub mod loader;

// use gl::*;
use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLboolean, GLsizeiptr, 
use model::loader::Loader;
use shader::lighting::Lighting;
// use model::mesh::Mesh;
// use util::rvertex::RVertex;
// use std::path::Path;

#[derive (Debug, PartialEq)]
pub struct Model {
  pub name: String,
  pub tex_name: String,
  pub raw: Option<RawModel>,
  pub lighting: Option<Lighting>,
  pub texture: GLuint,
  pub use_tex: bool,
}

impl Model {
  pub fn new(model_name: &str) -> Self {
    Model {
      name: model_name.to_string(),
      tex_name: "".to_string(),
      raw: None,
      lighting: None,
      texture: 0,
      use_tex: false
    }
  }
  pub fn init_with_texture(&mut self, loader: &mut Loader, tex: &str) -> &mut Self {
    self.tex_name = tex.to_string();
    self.init(loader)
    .load_texture(loader)
    .with_lighting()
  }
  pub fn init_default_texture(&mut self, loader: &mut Loader) -> &mut Self {
    self.tex_name = self.name.clone();
    self.init(loader)
    .load_texture(loader)
    .with_lighting()
  }
  pub fn init(&mut self, loader: &mut Loader) -> &mut Self {
    self.raw = Some(loader.load_to_vao(&self.name));
    self
  }
  pub fn load_texture(&mut self, loader: &mut Loader) -> &mut Self {
    self.texture = loader.load_texture(&self.tex_name);
    self
  }
  pub fn with_lighting(&mut self) -> &mut Self {
    self.lighting = Some(Lighting::new());
    self
  }
  pub fn lighting(&self) -> &Lighting {
    self.lighting.as_ref().unwrap()
  }
  pub fn lighting_mut(&mut self) -> &mut Lighting {
    self.lighting.as_mut().unwrap()
  }
  pub fn raw(&self) -> &RawModel {
    match &self.raw {
      Some(model) => { model }
      _ => panic!("raw model not loaded.")
    }
  }
}

#[derive (Debug, Eq, Hash, PartialEq)]
pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: i32,
}

impl RawModel {
  pub fn new(id: u32, count: i32) -> Self {
    RawModel { vao_id: id, vertex_count: count }
  }
}
