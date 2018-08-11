
use util::rvector::{Vector3f, XVEC, YVEC, ZVEC};
use util::rmatrix::Matrix4f;

pub struct PosMarker {
  pub pos: Vector3f,
  pub rx: f32,
  pub ry: f32,
  pub rz: f32,
  pub scale: f32,
  pub distance: f32,
  pub trans_mat: Matrix4f,
}

impl PosMarker {
  pub fn new() -> Self {
    PosMarker {
      pos: Vector3f::blank(),
      rx: 0_f32,
      ry: 0_f32,
      rz: 0_f32,
      scale: 1_f32,
      distance: 0_f32,
      trans_mat: Matrix4f::new(),
    }
  }
  
  pub fn forward(&mut self, dist: f32) {
    self.pos.x += dist * self.ry.to_radians().sin();
    self.pos.z += dist * self.ry.to_radians().cos();
  }
  
  pub fn transformation(&mut self) -> &Matrix4f {
    self.calc_transformation();
    &self.trans_mat
  }
  
  fn calc_transformation(&mut self) {
    self.trans_mat.set_identity();
    self.trans_mat.translate_v3f(&self.pos);
    self.trans_mat.rotate(self.rx.to_radians(), &XVEC);
    self.trans_mat.rotate(self.ry.to_radians(), &YVEC);
    self.trans_mat.rotate(self.rz.to_radians(), &ZVEC);
    self.trans_mat.scale(&Vector3f::new(self.scale, self.scale, self.scale));
  }
  
  pub fn inc_rot(&mut self, dx: f32, dy: f32, dz: f32) {
    self.rx += dx;
    if self.rx > 360_f32 { self.rx -= 360_f32; }
    self.ry += dy;
    if self.ry > 360_f32 { self.ry -= 360_f32; }
    self.rz += dz;
    if self.rz > 360_f32 { self.rz -= 360_f32; }
  }
}
