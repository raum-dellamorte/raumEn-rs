use util::rvertex::{Vertex, Normal};

pub struct Mesh {
  verts: Vector<Vertex>,
  norms: Vector<Normal>,
  indcs: Vector<u16>,
}
