#[allow(non_snake_case)]
#[allow(unused_imports)]

use gl::*;
use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLboolean, GLsizeiptr, 
use model::import::load_obj;
use model::loader::Loader;
use model::mesh::Mesh;
use util::rvertex::RVertex;
//use std::path::Path;

pub struct Model {
  pub name: String,
  pub mesh: Option<Mesh>,
  pub raw: Option<RawModel>,
  pub texture: GLuint,
  pub use_tex: bool,
}

impl Model {
  pub fn new(model_name: &str) -> Self {
    Model {
      name: format!("{}", model_name),
      mesh: None,
      raw: None,
      texture: 0,
      use_tex: false
    }
  }
  pub fn load_defaults(&mut self, loader: &mut Loader) -> &mut Self {
    self.load_default_mesh(loader)
    .load_default_texture(loader)
  }
  pub fn load_default_mesh(&mut self, loader: &mut Loader) -> &mut Self {
    let mesh = match load_obj(&self.name) {
      Ok(mesh) => { mesh }
      _ => panic!("Mesh {} failed to load.", self.name)
    };
    self.raw = Some(loader.load_to_vao(&mesh));
    self.mesh = Some(mesh);
    self
  }
  pub fn load_default_texture(&mut self, loader: &mut Loader) -> &mut Self {
    self.texture = loader.load_texture(&self.name);
    self
  }
  pub fn raw(&self) -> &RawModel {
    match &self.raw {
      Some(model) => { model }
      _ => panic!("raw model not loaded.")
    }
  }
  pub fn mesh_verts(&self) -> &[RVertex] {
    match self.mesh {
      Some(ref mesh) => { &mesh.verts }
      _ => { &[] }
    }
  }
}

#[derive(Debug)]
pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: i32,
}

impl RawModel {
  pub fn new(id: u32, count: i32) -> Self {
    RawModel { vao_id: id, vertex_count: count }
  }
}
