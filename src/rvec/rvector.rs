#![allow(non_snake_case)]
#![allow(dead_code)]

trait rvec {
  fn new() -> Self;
  fn lenSqr(&self) -> f32;
  fn len(&self) -> f32 { ((self.lenSqr() as f64).sqrt() as f32) }
}

#[derive(Debug, Copy, Clone)]
pub struct vector2f {
  pub x: f32,
  pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct vector3f {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct vector4f {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl rvec for vector2f {
  fn new() -> vector2f { vector2f {x: 0.0_f32, y: 0.0_f32} }
  fn lenSqr(&self) -> f32 { (x * x) + (y * y) }
}

impl rvec for vector3f {
  fn new() -> vector3f { vector2f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32} }
  fn lenSqr(&self) -> f32 { (x * x) + (y * y) +  (z * z) }
}

impl rvec for vector4f {
  fn new() -> vector4f { vector2f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32, w: 0.0_f32} }
  fn lenSqr(&self) -> f32 { (x * x) + (y * y) +  (z * z) + (w * w)}
}
