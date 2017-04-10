#![allow(non_snake_case)]
#![allow(dead_code)]

// need sub, cross, dot, angle, and length between 2 vectors

use std::ops::{Add, AddAssign, Sub, SubAssign}; // , Div, DivAssign, Mul, MulAssign

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

impl Vector2f {
  fn new() -> Self { Vector2f {x: 0.0_f32, y: 0.0_f32} }
  fn toSlice(&self) -> [f32; 2] { [self.x, self.y] }
  fn scaleTo(&self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
  }
  fn negateTo(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
  }
}

impl RVec for Vector2f {
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

impl Add for Vector2f {
  type Output = Vector2f;
  
  fn add(self, other: Vector2f) -> Vector2f {
    Vector2f {x: self.x + other.x, y: self.y + other.y}
  }
}

impl AddAssign for Vector2f {
  fn add_assign(&mut self, other: Vector2f) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl Sub for Vector2f {
  type Output = Vector2f;
  
  fn sub(self, other: Vector2f) -> Vector2f {
    Vector2f {x: self.x - other.x, y: self.y - other.y}
  }
}

impl SubAssign for Vector2f {
  fn sub_assign(&mut self, other: Vector2f) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl Vector3f {
  fn new() -> Self { Vector3f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32} }
  fn toSlice(&self) -> [f32; 3] { [self.x, self.y, self.z] }
  fn cross(&mut self, other: Self) {
    self.x = self.y * other.z - self.z * other.y;
    self.y = other.x * self.z - other.z * self.x;
    self.z = self.x * other.y - self.y * other.x;
  }
  fn crossTo(&self, other: &Self, dest: &mut Self) {
    (*dest).x = self.y * other.z - self.z * other.y;
    (*dest).y = other.x * self.z - other.z * self.x;
    (*dest).z = self.x * other.y - self.y * other.x;
  }
  fn crossToNew(&self, other: &Self) -> Self {
    let mut out = Vector3f::new();
    self.crossTo(other, &mut out);
    out
  }
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

impl RVec for Vector3f {
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

impl Add for Vector3f {
  type Output = Vector3f;
  
  fn add(self, other: Vector3f) -> Vector3f {
    Vector3f {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

impl AddAssign for Vector3f {
  fn add_assign(&mut self, other: Vector3f) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl Sub for Vector3f {
  type Output = Vector3f;
  
  fn sub(self, other: Vector3f) -> Vector3f {
    Vector3f {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

impl SubAssign for Vector3f {
  fn sub_assign(&mut self, other: Vector3f) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl Vector4f {
  fn new() -> Self { Vector4f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32, w: 0.0_f32} }
  fn toSlice(&self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }
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

impl RVec for Vector4f {
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

impl Add for Vector4f {
  type Output = Vector4f;
  
  fn add(self, other: Vector4f) -> Vector4f {
    Vector4f {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w}
  }
}

impl AddAssign for Vector4f {
  fn add_assign(&mut self, other: Vector4f) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
    self.w += other.w;
  }
}

impl Sub for Vector4f {
  type Output = Vector4f;
  
  fn sub(self, other: Vector4f) -> Vector4f {
    Vector4f {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w}
  }
}

impl SubAssign for Vector4f {
  fn sub_assign(&mut self, other: Vector4f) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
    self.w -= other.w;
  }
}
