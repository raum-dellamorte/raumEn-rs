#![allow(dead_code)]

use {
  // GameMgr,
  // Loader,
  util::{
    Matrix4f, Vector2f, Vector3f,
  },
};

// -1.0,1.0,-1.0,-1.0,1.0,1.0,1.0,-1.0

pub struct HUD {
  pub quad_id: u32,
  pub elements: Vec<GuiObj>,
}
impl HUD {
  pub fn new(quad_id: u32) -> Self {
    HUD {
      quad_id: quad_id,
      elements: Vec::new(),
    }
  }
  
}

pub struct GuiObj {
  pub pos: Vector2f,
  pub pos_last: Vector2f,
  pub scale: Vector2f,
  pub transmat: Matrix4f,
  pub row_count: f32,
  pub offset: Vector2f,
  pub flip_y: bool,
  pub tex_id: u32,
  pub depth_tex_id: u32,
}
impl GuiObj {
  pub fn new() -> Self {
    GuiObj {
      pos: Vector2f::new(0.25,0.75),
      pos_last: Vector2f::new(0.0,0.0),
      scale: Vector2f {x: 0.25, y: 0.25},
      transmat: Matrix4f::new(),
      row_count: 1_f32,
      offset: Vector2f::blank(),
      flip_y: false,
      tex_id: 0,
      depth_tex_id: 0,
    }
  }
  pub fn transformation(&mut self) -> &Matrix4f {
    self.calc_transformation();
    &self.transmat
  }
  fn calc_transformation(&mut self) {
    // Don't recalc if we haven't moved.
    if self.pos == self.pos_last { return }
    self.pos_last.from_v2f(&self.pos);
    self.transmat.set_identity();
    let x = (&self.pos.x *  2.0_f32) - 1.0_f32;
    let y = (&self.pos.y * -2.0_f32) + 1.0_f32;
    self.transmat.translate_v3f(&Vector3f {x: x, y: y, z: 0.0_f32});
    self.transmat.scale(&Vector3f::new(self.scale.x, self.scale.y, 1.0));
  }
}