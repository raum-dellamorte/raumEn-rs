#![allow(dead_code)]

use {
  crate::{
    Model,
    util::{
      Matrix4f, Vector2f, Vector3f,
    },
  },
};

// -1.0,1.0,-1.0,-1.0,1.0,1.0,1.0,-1.0

pub struct HUD {
  pub quad: Model,
  pub elements: Vec<GuiObj>,
}
impl HUD {
  pub fn new(quad: Model) -> Self {
    HUD {
      quad,
      elements: Vec::new(),
    }
  }
  
}

pub struct GuiObj {
  pub pos: Vector2f<f32>,
  pub pos_last: Vector2f<f32>,
  pub scale: Vector2f<f32>,
  pub transmat: Matrix4f<f32>,
  pub row_count: f32,
  pub offset: Vector2f<f32>,
  pub flip_y: bool,
  pub tex_count: usize,
  pub tex_id: u32,
  pub depth_tex_id: u32,
}
impl Default for GuiObj {
  fn default() -> Self {
    Self {
      pos: (0.75,0.75).into(),
      pos_last: (0.0,0.0).into(),
      scale: (0.25, 0.25).into(),
      transmat: Matrix4f::new(),
      row_count: 4_f32,
      offset: (0.0,0.0).into(),
      flip_y: false,
      tex_count: 1,
      tex_id: 0,
      depth_tex_id: 0,
    }
  }
}
impl GuiObj {
  pub fn new_one() -> Self {
    Self::default()
  }
  pub fn new_two() -> Self {
    let mut out = Self::default();
    out.pos.x = 0.25;
    out.tex_count = 2;
    out.row_count = 1.;
    out
  }
  pub fn transformation(&mut self) -> &Matrix4f<f32> {
    self.calc_transformation();
    &self.transmat
  }
  fn calc_transformation(&mut self) {
    // Don't recalc if we haven't moved.
    if self.pos == self.pos_last { return }
    self.pos_last.copy_from_v2f(self.pos);
    self.transmat.set_identity();
    let x = (self.pos.x *  2.0_f32) - 1.0_f32;
    let y = (self.pos.y * -2.0_f32) + 1.0_f32;
    self.transmat.translate_v3f(Vector3f {x, y, z: 0.0_f32});
    self.transmat.scale(Vector3f::new(self.scale.x, self.scale.y, 1.0));
  }
}