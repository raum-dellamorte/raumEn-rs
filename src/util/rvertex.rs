use util::rvector::{Vector2f, Vector3f, Vector4f};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

#[derive(Copy, Clone)]
pub struct TextureMap {
    pub txtrmp: (f32, f32)
}

implement_vertex!(Vertex, position);
implement_vertex!(Normal, normal);
implement_vertex!(TextureMap, txtrmp);

impl Vertex {
  fn new(v3f: Vector3f) -> Self {
    Vertex { position: (v3f.x, v3f.y, v3f.z) }
  }
}

impl Normal {
  fn new(v3f: Vector3f) -> Self {
    Normal { normal: (v3f.x, v3f.y, v3f.z) }
  }
}

impl TextureMap {
  fn new(v2f: Vector2f) -> Self {
    TextureMap { txtrmp: (v2f.x, v2f.y) }
  }
}
