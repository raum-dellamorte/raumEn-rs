#![allow(dead_code)]

// need sub, cross, dot, angle, and length between 2 vectors

use {
  std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    fmt,
  },
}; // , Div, DivAssign, Mul, MulAssign

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

pub const XVEC: Vector3f = Vector3f {x: 1.0_f32, y: 0.0_f32, z: 0.0_f32};
pub const YVEC: Vector3f = Vector3f {x: 0.0_f32, y: 1.0_f32, z: 0.0_f32};
pub const ZVEC: Vector3f = Vector3f {x: 0.0_f32, y: 0.0_f32, z: 1.0_f32};

pub trait RVec {
  fn len_sqr(&self) -> f32;
  fn len(&self) -> f32 { ((self.len_sqr() as f32).sqrt() as f32) }
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
  fn len_sqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) }
  fn scale(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
  }
}

impl RVec for Vector3f {
  fn len_sqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) }
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

impl RVec for Vector4f {
  fn len_sqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) + (self.w * self.w)}
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

impl Vector2f {
  pub fn new(x: f32, y: f32) -> Self { Vector2f {x, y} }
  pub fn blank() -> Self { Self {x: 0.0_f32, y: 0.0_f32} }
  pub fn copy_from_v2f(&mut self, other: Vector2f) {
    self.x = other.x;
    self.y = other.y;
  }
  pub fn to_slice(self) -> [f32; 2] { [self.x, self.y] }
  pub fn scale_to(self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
  }
  pub fn negate_to(self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
  }
}

impl Vector3f {
  //#[derive_keyword_argument_macro("new", x=0.0, y=0.0, z=0.0)]
  pub fn new(x: f32, y: f32, z: f32) -> Self { Self {x, y, z} }
  pub fn new_from_v3f(other: &Vector3f) -> Self { Self { x: other.x, y: other.y, z: other.z, } }
  pub fn new_isize(x: isize, y: isize, z: isize) -> Self { Self {x: x as f32, y: y as f32, z: z as f32} }
  pub fn blank() -> Self { Vector3f::new_isize(0,0,0) }
  pub fn copy_from_isize(&mut self, x: isize, y: isize, z: isize) {
    self.x = x as f32;
    self.y = y as f32;
    self.z = z as f32;
  }
  pub fn copy_from_f32(&mut self, x: f32, y: f32, z: f32) {
    self.x = x;
    self.y = y;
    self.z = z;
  }
  pub fn copy_from_v3f(&mut self, other: &Vector3f) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
  }
  // pub fn xy_from_v3f(&mut self, other: &Vector3f) {
  //   self.x = other.x;
  //   self.y = other.y;
  // }
  pub fn xz_from_v3f(&mut self, other: &Vector3f) {
    self.x = other.x;
    self.z = other.z;
  }
  // pub fn yz_from_v3f(&mut self, other: &Vector3f) {
  //   self.y = other.y;
  //   self.z = other.z;
  // }
  pub fn rotate_y_to(&self, dest: &mut Self, theta: f32) {
    dest.x = ( self.x * theta.cos()) - (self.z * theta.sin());
    dest.y = self.y;
    dest.z = (-self.x * theta.sin()) + (self.z * theta.cos());
  }
  pub fn xz_from_dist_rot_offset(&mut self, other: &Self, dist: f32, rot: f32) {
    let ry = rot.to_radians();
    self.x = other.x + (dist * ry.sin());
    self.z = other.z + (dist * ry.cos());
  }
  pub fn is_blank(&self) -> bool {
    self.x == 0.0 &&
    self.y == 0.0 &&
    self.z == 0.0
  }
  pub fn dist_rot_offset(&self, dist: f32, rot: f32) -> Self {
    let ry = rot.to_radians();
    Self {
      x: self.x + (dist * ry.sin()),
      y: self.y,
      z: self.z + (dist * ry.cos()),
    }
  }
  pub fn to_slice(&self) -> [f32; 3] { [self.x, self.y, self.z] }
  pub fn sub_to(&self, other: &Self, dest: &mut Self) {
    (*dest).x = self.x - other.x;
    (*dest).y = self.y - other.y;
    (*dest).z = self.z - other.z;
  }
  pub fn dot(&self, other: &Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
  pub fn cross(&mut self, other: Self) {
    self.x = self.y * other.z - self.z * other.y;
    self.y = other.x * self.z - other.z * self.x;
    self.z = self.x * other.y - self.y * other.x;
  }
  pub fn cross_to(&self, other: &Self, dest: &mut Self) {
    (*dest).x = self.y * other.z - self.z * other.y;
    (*dest).y = other.x * self.z - other.z * self.x;
    (*dest).z = self.x * other.y - self.y * other.x;
  }
  pub fn cross_to_new(&self, other: &Self) -> Self {
    let mut out = Vector3f::blank();
    self.cross_to(other, &mut out);
    out
  }
  pub fn scale_to(&self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
    (*dest).z = self.z * scale;
  }
  pub fn negate_to(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
  }
  pub fn round_to(&self, dest: &mut Self, places: i32) {
    use util::RFloat;
    (*dest).x = self.x.round_to(places);
    (*dest).y = self.y.round_to(places);
    (*dest).z = self.z.round_to(places);
  }
  pub fn clear(&mut self) {
    self.x = 0.0;
    self.y = 0.0;
    self.z = 0.0;
  }
}

impl Vector4f {
  fn new() -> Self { Vector4f {x: 0.0_f32, y: 0.0_f32, z: 0.0_f32, w: 0.0_f32} }
  fn to_slice(&self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }
  fn scale_to(&self, dest: &mut Self, scale: f32) {
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
    (*dest).z = self.z * scale;
    (*dest).w = self.w * scale;
  }
  fn negate_to(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
    (*dest).w = -self.w;
  }
}

impl fmt::Display for Vector2f {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector2f(x:{}, y:{})", self.x, self.y)
  }
}

impl fmt::Display for Vector3f {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector3f(x:{}, y:{}, z:{})", self.x, self.y, self.z)
  }
}

impl fmt::Display for Vector4f {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector4f(x:{}, y:{}, z:{}, w:{})", self.x, self.y, self.z, self.w)
  }
}

impl PartialEq for Vector2f {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
impl Eq for Vector2f {}

impl PartialEq for Vector3f {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}
impl Eq for Vector3f {}

impl PartialEq for Vector4f {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
  }
}
impl Eq for Vector4f {}

impl Add for &Vector2f {
  type Output = Vector2f;
  
  fn add(self, other: &Vector2f) -> Vector2f {
    Vector2f {x: self.x + other.x, y: self.y + other.y}
  }
}

impl Add for &Vector3f {
  type Output = Vector3f;
  
  fn add(self, other: &Vector3f) -> Vector3f {
    Vector3f {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

impl Add for &Vector4f {
  type Output = Vector4f;
  
  fn add(self, other: &Vector4f) -> Vector4f {
    Vector4f {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w}
  }
}

impl AddAssign for Vector2f {
  fn add_assign(&mut self, other: Vector2f) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl AddAssign for Vector3f {
  fn add_assign(&mut self, other: Vector3f) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
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

impl Sub for &Vector2f {
  type Output = Vector2f;
  
  fn sub(self, other: &Vector2f) -> Vector2f {
    Vector2f {x: self.x - other.x, y: self.y - other.y}
  }
}

impl Sub for &Vector3f {
  type Output = Vector3f;
  
  fn sub(self, other: &Vector3f) -> Vector3f {
    Vector3f {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

impl Sub for &Vector4f {
  type Output = Vector4f;
  
  fn sub(self, other: &Vector4f) -> Vector4f {
    Vector4f {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w}
  }
}

impl SubAssign for Vector2f {
  fn sub_assign(&mut self, other: Vector2f) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl SubAssign for Vector3f {
  fn sub_assign(&mut self, other: Vector3f) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
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
