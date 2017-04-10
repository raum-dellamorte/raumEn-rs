use util::rvertex::{Vertex, Normal, TextureMap};
//use util::rvector::Vector2f;

use glium::{VertexBuffer, IndexBuffer};
use glium::backend::Facade;
use glium::index::PrimitiveType::TrianglesList;

pub struct Mesh {
  pub verts: Vec<Vertex>,
  pub txtrs: Vec<TextureMap>,
  pub norms: Vec<Normal>,
  pub indcs: Vec<u16>,
  pub far_point: u16,
}

pub struct MeshBuffers {
  pub verts: VertexBuffer<Vertex>,
  pub norms: VertexBuffer<Normal>,
  pub indcs: IndexBuffer<u16>,
}

impl Mesh {
  pub fn create_buffers(&self, display: &Facade) -> MeshBuffers {
    let v = VertexBuffer::new(display, &(self.verts)).unwrap();
    let n = VertexBuffer::new(display, &(self.norms)).unwrap();
    let i = IndexBuffer::new(display, TrianglesList, &(self.indcs)).unwrap();
    MeshBuffers {verts: v, norms: n, indcs: i}
  }
}
