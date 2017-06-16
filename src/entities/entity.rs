
use util::rvector::{Vector3f, XVEC, YVEC, ZVEC};
use util::rmatrix::Matrix4f;

pub struct PosMarker {
  pub pos: Vector3f,
  pub rx: f32,
  pub ry: f32,
  pub rz: f32,
  pub scale: f32,
  pub transMat: Matrix4f,
}

pub struct Entity {
  pub marker: PosMarker,
  pub h: f32,
  pub w: f32,
  pub distance: f32,
}

impl PosMarker {
  pub fn new() -> Self {
    PosMarker {
      pos: Vector3f::blank(),
      rx: 0_f32,
      ry: 0_f32,
      rz: 0_f32,
      scale: 1_f32,
      transMat: Matrix4f::new(),
    }
  }
  
  pub fn transformation(&mut self) -> [[f32; 4]; 4] {
    self.calc_transformation();
    self.transMat.as_slice()
  }
  
  fn calc_transformation(&mut self) {
    self.transMat.setIdentity();
    self.transMat.translate_v3f(&self.pos);
    self.transMat.rotate(self.rx.to_radians(), &XVEC);
    self.transMat.rotate(self.ry.to_radians(), &YVEC);
    self.transMat.rotate(self.rz.to_radians(), &ZVEC);
    self.transMat.scale(&Vector3f::new(self.scale, self.scale, self.scale));
  }
  
  pub fn inc_xrot(&mut self, dx: f32) {
    self.rx += dx;
    if self.rx > 360_f32 { self.rx -= 360_f32; }
  }
  
  pub fn inc_yrot(&mut self, dy: f32) {
    self.ry += dy;
    if self.ry > 360_f32 { self.ry -= 360_f32; }
  }
  
  pub fn inc_zrot(&mut self, dz: f32) {
    self.rz += dz;
    if self.rz > 360_f32 { self.rz -= 360_f32; }
  }
}
