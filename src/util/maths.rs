
use util::rmatrix::Matrix4f;
use util::rvector::{Vector2f, Vector3f, XVEC, YVEC, ZVEC};

pub fn barryCentric(p1: &Vector3f, p2: &Vector3f, p3: &Vector3f, pos: &Vector2f) -> f32 {
  let det: f32 = ((p2.z - p3.z) * (p1.x - p3.x)) + ((p3.x - p2.x) * (p1.z - p3.z));
  let l1: f32 = (((p2.z - p3.z) * (pos.x - p3.x)) + ((p3.x - p2.x) * (pos.y - p3.z))) / det;
  let l2: f32 = (((p3.z - p1.z) * (pos.x - p3.x)) + ((p1.x - p3.x) * (pos.y - p3.z))) / det;
  let l3: f32 = (1.0 - l1) - l2;
  ((l1 * p1.y) + (l2 * p2.y)) + (l3 * p3.y)
}

pub fn createTransformationMatrix_vf3(matrix: &mut Matrix4f, translation: &Vector3f, rx: f32, ry: f32, rz: f32, scale: f32) {
  matrix.setIdentity();
  matrix.translate_v3f(translation);
  matrix.rotate(rx.to_radians(), &XVEC);
  matrix.rotate(ry.to_radians(), &YVEC);
  matrix.rotate(rz.to_radians(), &ZVEC);
  matrix.scale(&Vector3f::new(scale, scale, scale));
}

pub fn createTransformationMatrix_v2f_ordered(matrix: &mut Matrix4f, translation: &Vector2f, scale: &Vector2f, drawOrder: usize) {
  matrix.setIdentity();
  let depth = -((drawOrder as f32) * 0.00001);
  matrix.translate_v3f(&Vector3f::new(translation.x, translation.y, depth));
  matrix.scale(&Vector3f::new(scale.x, scale.y, 1.0_f32));
}

pub fn createTransformationMatrix_v2f(matrix: &mut Matrix4f, translation: &Vector2f, scale: &Vector2f) {
  createTransformationMatrix_v2f_ordered(matrix, translation, scale, 0);
}

pub fn createTransformationMatrix_gui(matrix: &mut Matrix4f, translation: &Vector2f, scale: &Vector2f) {
  matrix.setIdentity();
  matrix.translate_v3f(&Vector3f::new((translation.x * 2.0_f32) - 1.0_f32, (translation.y * -2.0_f32) + 1.0_f32, 0.0_f32));
  matrix.scale(&Vector3f::new(scale.x, scale.y, 1.0_f32));
}

pub fn createPosMatrix(matrix: &mut Matrix4f, pos: &Vector3f, pitch: f32, yaw: f32) {
  matrix.setIdentity();
  matrix.rotate(pitch.to_radians(), &XVEC);
  matrix.rotate(yaw.to_radians(), &YVEC);
  let mut negPos = Vector3f::blank();
  pos.negateTo(&mut negPos);
  matrix.translate_v3f(&negPos);
}
