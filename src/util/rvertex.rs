//use util::rvector::{Vector2f, Vector3f}; // , Vector4f

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
    pub tex_coords: (f32, f32),
    pub is_set: bool,
}

implement_vertex!(Vertex, position, normal, tex_coords);

impl Vertex {
  pub fn new() -> Self {
    Vertex {
      position: (0_f32, 0_f32, 0_f32),
      normal: (0_f32, 0_f32, 0_f32),
      tex_coords: (0_f32, 0_f32),
      is_set: false,
    }
  }
}
