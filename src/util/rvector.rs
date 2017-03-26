#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Vector2f {
  pub x: f32,
  pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3f {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector4f {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

pub trait RVec {
  fn new() -> Self;
  fn lenSqr(&self) -> f32;
  fn len(&self) -> f32 { ((self.lenSqr() as f64).sqrt() as f32) }
  fn scale(&mut self, scale: f32);
  fn negate(&mut self);
  fn normalize(&mut self) {
    let l = self.len();
    if l != 0.0_f32 {
      self.scale(1.0_f32 / l)
    } else {
      panic!("Zero length vector")
    }
  }
}

impl RVec for Vector2f {
  fn new() -> Self { Vector2f {x: 0.0_f32, y: 0.0_f32} }
  fn lenSqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) }
  fn scale(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
  }
}

impl Vector2f {
  fn scaleTo(&self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
  }
  fn negateTo(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
  }
}

impl RVec for Vector3f {
  fn new() -> Self { Vector3f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32} }
  fn lenSqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) }
  fn scale(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
    self.z *= scale;
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
  }
}

impl Vector3f {
  fn scaleTo(&self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
    (*dest).z = self.z * scale;
  }
  fn negateTo(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
  }
}

impl RVec for Vector4f {
  fn new() -> Self { Vector4f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32, w: 0.0_f32} }
  fn lenSqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) + (self.w * self.w)}
  fn scale(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
    self.z *= scale;
    self.w *= scale;
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
    self.w = -self.w;
  }
}

impl Vector4f {
  fn scaleTo(&self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
    (*dest).z = self.z * scale;
    (*dest).w = self.w * scale;
  }
  fn negateTo(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
    (*dest).w = -self.w;
  }
}
