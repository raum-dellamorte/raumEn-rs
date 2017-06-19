use util::rvertex::Vertex;
//use util::rvector::Vector2f;

use glium::{VertexBuffer, IndexBuffer};
use glium::backend::Facade;
use glium::index::PrimitiveType::TrianglesList;

pub struct Mesh {
  pub verts: Vec<Vertex>,
  pub indcs: Vec<u16>,
  pub far_point: u16,
  pub buffers: Option<MeshBuffers>,
}

pub struct MeshBuffers {
  pub verts: VertexBuffer<Vertex>,
  pub indcs: IndexBuffer<u16>,
}

impl Mesh {
  pub fn create_buffers(&mut self, display: &Facade) {
    let v = VertexBuffer::new(display, &(self.verts)).unwrap();
    let i = IndexBuffer::new(display, TrianglesList, &(self.indcs)).unwrap();
    self.buffers = Some(MeshBuffers {verts: v, indcs: i});
  }
}
