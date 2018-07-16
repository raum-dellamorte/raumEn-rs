//use util::rvector::{Vector2f, Vector3f}; // , Vector4f

#[derive(Copy, Clone)]
pub struct RVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub is_set: bool,
}

//impl_vertex!(Vertex, position, normal, tex_coords);

impl RVertex {
  pub fn new() -> Self {
    RVertex {
      position: [0_f32; 3],
      normal: [0_f32; 3],
      tex_coords: [0_f32; 2],
      is_set: false,
    }
  }
}

#[derive(Copy, Clone)]
pub struct RVertex2D {
  pub position: [f32; 2],
  pub tex_coords: [f32; 2],
}

//impl_vertex!(Vertex2D, position, tex_coords);

impl RVertex2D {
  pub fn new() -> Self {
    RVertex2D {
      position: [0_f32; 2],
      tex_coords: [0_f32; 2],
    }
  }
}