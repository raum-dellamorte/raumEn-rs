//use util::rvector::{Vector2f, Vector3f}; // , Vector4f

#[derive(Copy, Clone)]
pub struct RVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub is_set: bool,
}
impl Default for RVertex {
  fn default() -> Self {
    Self::new()
  }
}
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
impl Default for RVertex2D {
  fn default() -> Self {
    Self::new()
  }
}
impl RVertex2D {
  pub fn new() -> Self {
    RVertex2D {
      position: [0_f32; 2],
      tex_coords: [0_f32; 2],
    }
  }
}