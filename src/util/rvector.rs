#![allow(dead_code)]

// need sub, cross, dot, angle, and length between 2 vectors

use {
  std::{
    ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul},
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

#[derive(Clone,Copy,Debug)]
pub struct Quaternion {
  pub w: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

pub const XVEC: Vector3f = Vector3f {x: 1.0_f32, y: 0.0_f32, z: 0.0_f32};
pub const YVEC: Vector3f = Vector3f {x: 0.0_f32, y: 1.0_f32, z: 0.0_f32};
pub const ZVEC: Vector3f = Vector3f {x: 0.0_f32, y: 0.0_f32, z: 1.0_f32};

pub trait RVec {
  fn sum(&self) -> f32;
  fn len_sqr(&self) -> f32;
  fn len(&self) -> f32 { ((self.len_sqr() as f32).sqrt() as f32) }
  fn scale(&mut self, scale: f32);
  fn divscale(&mut self, scale: f32);
  fn negate(&mut self);
  fn normalize(&mut self) {
    let mag = self.len();
    if mag != 0.0_f32 {
      self.divscale(mag);
    } else {
      panic!("Called normalize on zero length vector");
    }
  }
}

impl RVec for Vector2f {
  fn sum(&self) -> f32 { self.x + self.y }
  fn len_sqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) }
  fn scale(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
  }
  fn divscale(&mut self, scale: f32) {
    self.x /= scale;
    self.y /= scale;
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
  }
}

impl RVec for Vector3f {
  fn sum(&self) -> f32 { self.x + self.y + self.z }
  fn len_sqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) }
  fn scale(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
    self.z *= scale;
  }
  fn divscale(&mut self, scale: f32) {
    self.x /= scale;
    self.y /= scale;
    self.z /= scale;
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
  }
}

impl RVec for Quaternion {
  fn sum(&self) -> f32 { self.x + self.y + self.z + self.w }
  fn len_sqr(&self) -> f32 { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) + (self.w * self.w)}
  fn scale(&mut self, scale: f32) {
    self.w *= scale;
    self.x *= scale;
    self.y *= scale;
    self.z *= scale;
  }
  fn divscale(&mut self, n: f32) {
    self.w /= n;
    self.x /= n;
    self.y /= n;
    self.z /= n;
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
  pub fn new_from_v3f(other: Vector3f) -> Self { Self { x: other.x, y: other.y, z: other.z, } }
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
  pub fn copy_from_v3f(&mut self, other: Vector3f) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
  }
  // pub fn xy_from_v3f(&mut self, other: Vector3f) {
  //   self.x = other.x;
  //   self.y = other.y;
  // }
  pub fn xz_from_v3f(&mut self, other: Vector3f) {
    self.x = other.x;
    self.z = other.z;
  }
  // pub fn yz_from_v3f(&mut self, other: Vector3f) {
  //   self.y = other.y;
  //   self.z = other.z;
  // }
  pub fn xz_from_dist_rot_offset(&mut self, other: &Self, dist: f32, rot: f32) {
    let ry = rot.to_radians();
    self.x = other.x + (dist * ry.sin());
    self.z = other.z + (dist * ry.cos());
  }
  pub fn to_tuple(&self) -> (f32, f32, f32) {
    (self.x, self.y, self.z)
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
  pub fn cross_self(&mut self, other: Self) {
    self.x = self.y * other.z - self.z * other.y;
    self.y = other.x * self.z - other.z * self.x;
    self.z = self.x * other.y - self.y * other.x;
  }
  pub fn cross_to(&self, other: Self, dest: &mut Self) {
    (*dest).x = self.y * other.z - self.z * other.y;
    (*dest).y = other.x * self.z - other.z * self.x;
    (*dest).z = self.x * other.y - self.y * other.x;
  }
  pub fn cross(&self, other: Self) -> Self {
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

impl Default for Quaternion {
  fn default() -> Self { Self::new(1.,0.,0.,0.) }
}
impl Quaternion {
  pub fn new(w: f32, x: f32, y: f32, z: f32) -> Quaternion {
    Self { w, x, y, z, }
  }
  pub fn normalize(&mut self) {
    let n = self.len();
    if n == 0. { return }
    self.divscale(n);
  }
  fn scale_to(&self, dest: &mut Self, scale: f32) {
    (*dest).w = self.w * scale;
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
    (*dest).z = self.z * scale;
  }
  fn divscale_to(&self, dest: &mut Self, scale: f32) {
    (*dest).w = self.w * scale;
    (*dest).x = self.x * scale;
    (*dest).y = self.y * scale;
    (*dest).z = self.z * scale;
  }
  pub fn conjugate(&self) -> Quaternion {
    Self {
      w: self.w,
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
  fn conjugate_to(&self, dest: &mut Self) {
    (*dest).w = self.w;
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
  }
  fn negate_to(&self, dest: &mut Self) {
    (*dest).w = -self.w;
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
  }
  fn to_slice(&self) -> [f32; 4] { [self.w, self.x, self.y, self.z] }
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

impl fmt::Display for Quaternion {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "({} + {}i + {}j + {}k)", self.w, self.x, self.y, self.z)
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

impl PartialEq for Quaternion {
  fn eq(&self, other: &Self) -> bool {
    self.w == other.w && self.x == other.x && self.y == other.y && self.z == other.z
  }
}
impl Eq for Quaternion {}

impl Add for &Vector2f {
  type Output = Vector2f;
  
  fn add(self, other: &Vector2f) -> Vector2f {
    Vector2f {x: self.x + other.x, y: self.y + other.y}
  }
}

impl Add for Vector3f {
  type Output = Vector3f;
  
  fn add(self, other: Vector3f) -> Vector3f {
    Vector3f {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

impl Add for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn add(self, other: Quaternion) -> Self::Output {
    Quaternion {
      w: self.w + other.w,
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}
impl Add<f32> for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn add(self, other: f32) -> Self::Output {
    Quaternion {
      w: self.w + other,
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}
impl Add<Quaternion> for f32 {
  type Output = Quaternion;
  
  #[inline]
  fn add(self, other: Quaternion) -> Self::Output {
    Quaternion {
      w: other.w + self,
      x: other.x,
      y: other.y,
      z: other.z,
    }
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

impl AddAssign for Quaternion {
  fn add_assign(&mut self, other: Quaternion) {
    self.w += other.w;
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl Sub for &Vector2f {
  type Output = Vector2f;
  
  fn sub(self, other: &Vector2f) -> Vector2f {
    Vector2f {x: self.x - other.x, y: self.y - other.y}
  }
}

impl Sub for Vector3f {
  type Output = Vector3f;
  
  fn sub(self, other: Vector3f) -> Vector3f {
    Vector3f {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

impl Sub for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn sub(self, other: Quaternion) -> Self::Output {
    Quaternion {
      w: self.w - other.w,
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
impl Sub<f32> for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn sub(self, other: f32) -> Self::Output {
    Quaternion {
      w: self.w - other,
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}
impl Sub<Quaternion> for f32 {
  type Output = Quaternion;
  
  #[inline]
  fn sub(self, other: Quaternion) -> Self::Output {
    Quaternion {
      w: other.w - self,
      x: other.x,
      y: other.y,
      z: other.z,
    }
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

impl SubAssign for Quaternion {
  fn sub_assign(&mut self, other: Quaternion) {
    self.w -= other.w;
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl Mul for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn mul(self, rhs: Quaternion) -> Self::Output {
    Quaternion {
      w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
      x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
      y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
      z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
    }
  }
}
impl Mul<f32> for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn mul(self, other: f32) -> Self::Output {
    Quaternion {
      w: self.w * other,
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}
impl Mul<Quaternion> for f32 {
  type Output = Quaternion;
  
  #[inline]
  fn mul(self, other: Quaternion) -> Self::Output {
    Quaternion {
      w: other.w * self,
      x: other.x * self,
      y: other.y * self,
      z: other.z * self,
    }
  }
}
impl Neg for Quaternion {
  type Output = Quaternion;
  
  #[inline]
  fn neg(self) -> Self::Output {
    Quaternion {
      w: -self.w,
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
