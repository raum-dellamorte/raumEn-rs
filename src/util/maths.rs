
use util::rmatrix::Matrix4f;
use util::rvector::{Vector2f, Vector3f, XVEC, YVEC, ZVEC};

use num::{Float, NumCast};

pub trait RFloat {
  fn modulo<F>(&self, m: F) -> F where F: Float;
  fn round_to<F>(&self, places: i32) -> F where F: Float;
}
impl RFloat for f32 {
  fn modulo<F>(&self, m: F) -> F where F: Float {
    modulo(NumCast::from(*self).unwrap(), m)
  }
  fn round_to<F>(&self, places: i32) -> F where F: Float {
    round_to(NumCast::from(*self).unwrap(), places)
  }
}

pub fn modulo<F: Float>(x: F, m: F) -> F {
  let zero = NumCast::from(0).unwrap();
  if x == zero || m == zero { return zero }
  let out = x - ((x / m).floor() * m);
  if out == m { zero } else { out }
}

pub fn round_to<F: Float>(f: F, places: i32) -> F {
  let zero = NumCast::from(0).unwrap();
  if f == zero { return zero }
  let ten: F = NumCast::from(10).unwrap();
  let p: F = ten.powi(places);
  let out = (f * p).round() / p;
  out
}

pub fn barry_centric(p1: &Vector3f, p2: &Vector3f, p3: &Vector3f, pos: &Vector2f) -> f32 {
  let det: f32 = ((p2.z - p3.z) * (p1.x - p3.x)) + ((p3.x - p2.x) * (p1.z - p3.z));
  let l1: f32 = (((p2.z - p3.z) * (pos.x - p3.x)) + ((p3.x - p2.x) * (pos.y - p3.z))) / det;
  let l2: f32 = (((p3.z - p1.z) * (pos.x - p3.x)) + ((p1.x - p3.x) * (pos.y - p3.z))) / det;
  let l3: f32 = (1.0 - l1) - l2;
  ((l1 * p1.y) + (l2 * p2.y)) + (l3 * p3.y)
}

pub fn create_transformation_matrix_vf3(matrix: &mut Matrix4f, translation: &Vector3f, rx: f32, ry: f32, rz: f32, scale: f32) {
  matrix.set_identity();
  matrix.translate_v3f(translation);
  matrix.rotate(rx.to_radians(), &XVEC);
  matrix.rotate(ry.to_radians(), &YVEC);
  matrix.rotate(rz.to_radians(), &ZVEC);
  matrix.scale(&Vector3f::new(scale, scale, scale));
}

pub fn create_transformation_matrix_v2f_ordered(matrix: &mut Matrix4f, translation: &Vector2f, scale: &Vector2f, draw_order: usize) {
  matrix.set_identity();
  let depth = -((draw_order as f32) * 0.00001);
  matrix.translate_v3f(&Vector3f::new(translation.x, translation.y, depth));
  matrix.scale(&Vector3f::new(scale.x, scale.y, 1.0_f32));
}

pub fn create_transformation_matrix_v2f(matrix: &mut Matrix4f, translation: &Vector2f, scale: &Vector2f) {
  create_transformation_matrix_v2f_ordered(matrix, translation, scale, 0);
}

pub fn create_pos_matrix(matrix: &mut Matrix4f, pos: &Vector3f, pitch: f32, yaw: f32) {
  matrix.set_identity();
  matrix.rotate(pitch.to_radians(), &XVEC);
  matrix.rotate(yaw.to_radians(), &YVEC);
  let mut neg_pos = Vector3f::blank();
  pos.negate_to(&mut neg_pos);
  matrix.translate_v3f(&neg_pos);
}
