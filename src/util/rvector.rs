#![allow(dead_code)]

// need sub, cross, dot, angle, and length between 2 vectors

use {
  std::{
    ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign},
    fmt,
  },
  util::{
    RFloat, NumCast, Zero, 
  },
}; // , Div, DivAssign, Mul, MulAssign

pub const XVEC: Vector3f<f32> = Vector3f {x: 1.0_f32, y: 0.0_f32, z: 0.0_f32};
pub const YVEC: Vector3f<f32> = Vector3f {x: 0.0_f32, y: 1.0_f32, z: 0.0_f32};
pub const ZVEC: Vector3f<f32> = Vector3f {x: 0.0_f32, y: 0.0_f32, z: 1.0_f32};
pub const XVEC64: Vector3f<f64> = Vector3f {x: 1.0_f64, y: 0.0_f64, z: 0.0_f64};
pub const YVEC64: Vector3f<f64> = Vector3f {x: 0.0_f64, y: 1.0_f64, z: 0.0_f64};
pub const ZVEC64: Vector3f<f64> = Vector3f {x: 0.0_f64, y: 0.0_f64, z: 1.0_f64};


#[derive(Debug, Copy, Clone)]
// #[repr(C, packed)]
pub struct Vector2f<F: RFloat> {
  pub x: F,
  pub y: F,
}

#[derive(Debug, Copy, Clone)]
// #[repr(C, packed)]
pub struct Vector3f<F: RFloat> {
  pub x: F,
  pub y: F,
  pub z: F,
}

#[derive(Clone,Copy,Debug)]
// #[repr(C, packed)]
pub struct Quaternion<F: RFloat> {
  pub w: F,
  pub x: F,
  pub y: F,
  pub z: F,
}

impl<F: RFloat> From<(F, F)> for Vector2f<F> {
  fn from(other: (F, F)) -> Self {
      Self { x: other.0, y: other.1}
  }
}

impl<F: RFloat> From<(F, F, F)> for Vector3f<F> {
  fn from(other: (F, F, F)) -> Self {
      Self { x: other.0, y: other.1, z: other.2}
  }
}

impl<F: RFloat> From<(F, F, F, F)> for Quaternion<F> {
  fn from(other: (F, F, F, F)) -> Self {
      Self { w: other.0, x: other.1, y: other.2, z: other.3 }
  }
}

impl From<Vector2f<f32>> for Vector2f<f64> {
  fn from(v: Vector2f<f32>) -> Vector2f<f64> {
    Self {
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
    }
  }
}

impl From<Vector2f<f64>> for Vector2f<f32> {
  fn from(v: Vector2f<f64>) -> Vector2f<f32> {
    Self {
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
    }
  }
}

impl From<Vector3f<f64>> for Vector3f<f32> {
  fn from(v: Vector3f<f64>) -> Vector3f<f32> {
    Self {
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
      z: NumCast::from(v.z).unwrap(),
    }
  }
}

impl From<Vector3f<f32>> for Vector3f<f64> {
  fn from(v: Vector3f<f32>) -> Vector3f<f64> {
    Self {
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
      z: NumCast::from(v.z).unwrap(),
    }
  }
}

impl From<Quaternion<f64>> for Quaternion<f32> {
  fn from(v: Quaternion<f64>) -> Quaternion<f32> {
    Self {
      w: NumCast::from(v.w).unwrap(),
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
      z: NumCast::from(v.z).unwrap(),
    }
  }
}

impl From<Quaternion<f32>> for Quaternion<f64> {
  fn from(v: Quaternion<f32>) -> Quaternion<f64> {
    Self {
      w: NumCast::from(v.w).unwrap(),
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
      z: NumCast::from(v.z).unwrap(),
    }
  }
}

pub trait RVec<F: RFloat> {
  fn sum(&self) -> F;
  fn len_sqr(&self) -> F;
  fn len(&self) -> F { 
    let ls: F = self.len_sqr();
    ls.sqrt()
  }
  fn scale(&mut self, scale: F);
  fn divscale(&mut self, scale: F);
  fn negate(&mut self);
  fn clear(&mut self);
  fn normalize(&mut self) {
    let mag = self.len();
    let zero = NumCast::from(0).unwrap();
    if mag != zero {
      self.divscale(mag);
    } else {
      panic!("Called normalize on zero length vector");
    }
  }
}

impl<F: RFloat> RVec<F> for Vector2f<F> {
  fn sum(&self) -> F { self.x + self.y }
  fn len_sqr(&self) -> F { (self.x * self.x) + (self.y * self.y) }
  fn scale(&mut self, scale: F) { *self *= scale; }
  fn divscale(&mut self, scale: F) { *self /= scale; }
  fn clear(&mut self) {
    self.x = Zero::zero();
    self.y = Zero::zero();
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
  }
}

impl<F: RFloat> RVec<F> for Vector3f<F> {
  fn sum(&self) -> F { self.x + self.y + self.z }
  fn len_sqr(&self) -> F { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) }
  fn scale(&mut self, scale: F) { *self *= scale; }
  fn divscale(&mut self, scale: F) { *self /= scale; }
  fn clear(&mut self) {
    self.x = Zero::zero();
    self.y = Zero::zero();
    self.z = Zero::zero();
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
  }
}

impl<F: RFloat> RVec<F> for Quaternion<F> {
  fn sum(&self) -> F { self.x + self.y + self.z + self.w }
  fn len_sqr(&self) -> F { (self.x * self.x) + (self.y * self.y) +  (self.z * self.z) + (self.w * self.w)}
  fn scale(&mut self, scale: F) { *self *= scale; }
  fn divscale(&mut self, scale: F) { *self /= scale; }
  fn clear(&mut self) {
    self.w = NumCast::from(1).unwrap();
    self.x = Zero::zero();
    self.y = Zero::zero();
    self.z = Zero::zero();
  }
  fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
    self.w = -self.w;
  }
}

impl<F: RFloat> Default for Vector2f<F> {
  fn default() -> Self {
    Self { x: Zero::zero(), y: Zero::zero() }
  }
}

impl<F: RFloat> Default for Vector3f<F> {
  fn default() -> Self {
    Self { x: Zero::zero(), y: Zero::zero(), z: Zero::zero() }
  }
}

impl<F: RFloat> Default for Quaternion<F> {
  fn default() -> Self {
    Self { w: NumCast::from(1).unwrap(), x: Zero::zero(), y: Zero::zero(), z: Zero::zero() }
  }
}

impl<F: RFloat> Vector2f<F> {
  pub fn new(x: F, y: F) -> Self { Self {x, y} }
  pub fn blank() -> Self { Self::default() }
  pub fn copy_from_v2f(&mut self, other: Self) {
    self.x = other.x;
    self.y = other.y;
  }
  pub fn to_slice(self) -> [F; 2] { [self.x, self.y] }
  pub fn scale_to(self, dest: &mut Self, scale: F) { *dest = self * scale; }
  pub fn divscale_to(self, dest: &mut Self, scale: F) { *dest = self / scale; }
  pub fn negate_to(self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
  }
}

impl<F: RFloat> Vector3f<F> {
  //#[derive_keyword_argument_macro("new", x=0.0, y=0.0, z=0.0)]
  pub fn new(x: F, y: F, z: F) -> Self { Self {x, y, z} }
  pub fn new_from_v3f(other: Vector3f<F>) -> Self { Self { x: other.x, y: other.y, z: other.z, } }
  pub fn new_isize(x: isize, y: isize, z: isize) -> Self {
    Self {x: NumCast::from(x).unwrap(), y: NumCast::from(y).unwrap(), z: NumCast::from(z).unwrap()} 
  }
  pub fn blank() -> Self { Self::default() }
  pub fn copy_from_isize(&mut self, x: isize, y: isize, z: isize) {
    self.x = NumCast::from(x).unwrap();
    self.y = NumCast::from(y).unwrap();
    self.z = NumCast::from(z).unwrap();
  }
  pub fn copy_from_float(&mut self, x: F, y: F, z: F) {
    self.x = x;
    self.y = y;
    self.z = z;
  }
  pub fn copy_from_v3f(&mut self, other: Vector3f<F>) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
  }
  // pub fn xy_from_v3f(&mut self, other: Vector3f<F>) {
  //   self.x = other.x;
  //   self.y = other.y;
  // }
  pub fn xz_from_v3f(&mut self, other: Vector3f<F>) {
    self.x = other.x;
    self.z = other.z;
  }
  // pub fn yz_from_v3f(&mut self, other: Vector3f<F>) {
  //   self.y = other.y;
  //   self.z = other.z;
  // }
  pub fn xz_from_dist_rot_offset(&mut self, other: &Self, dist: F, rot: F) {
    let ry = rot.to_radians();
    self.x = other.x + (dist * ry.sin());
    self.z = other.z + (dist * ry.cos());
  }
  pub fn to_tuple(&self) -> (F, F, F) {
    (self.x, self.y, self.z)
  }
  pub fn is_blank(&self) -> bool {
    self.x == Zero::zero() &&
    self.y == Zero::zero() &&
    self.z == Zero::zero()
  }
  pub fn dist_rot_offset(&self, dist: F, rot: F) -> Self {
    let ry = rot.to_radians();
    Self {
      x: self.x + (dist * ry.sin()),
      y: self.y,
      z: self.z + (dist * ry.cos()),
    }
  }
  pub fn to_slice(&self) -> [F; 3] { [self.x, self.y, self.z] }
  pub fn sub_to(&self, other: Self, dest: &mut Self) {
    (*dest).x = self.x - other.x;
    (*dest).y = self.y - other.y;
    (*dest).z = self.z - other.z;
  }
  pub fn dot(&self, other: Self) -> F {
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
  pub fn scale_to(self, dest: &mut Self, scale: F) { *dest = self * scale; }
  pub fn divscale_to(self, dest: &mut Self, scale: F) { *dest = self / scale; }
  pub fn negate_to(&self, dest: &mut Self) {
    (*dest).x = -self.x;
    (*dest).y = -self.y;
    (*dest).z = -self.z;
  }
  pub fn round_to(&self, dest: &mut Self, places: i32) {
    (*dest).x = self.x.round_to(places);
    (*dest).y = self.y.round_to(places);
    (*dest).z = self.z.round_to(places);
  }
  pub fn clear(&mut self) {
    self.x = Zero::zero();
    self.y = Zero::zero();
    self.z = Zero::zero();
  }
}

impl<F: RFloat> Quaternion<F> {
  pub fn new(w: F, x: F, y: F, z: F) -> Quaternion<F> {
    Self { w, x, y, z, }
  }
  pub fn blank() -> Self {
    Self {
      w: Zero::zero(), x: Zero::zero(), y: Zero::zero(), z: Zero::zero(), 
    }
  }
  pub fn normalize(&mut self) {
    let n: F = self.len();
    if n == Zero::zero() { return }
    self.divscale(n);
  }
  pub fn scale_to(self, dest: &mut Self, scale: F) { *dest = self * scale; }
  pub fn divscale_to(self, dest: &mut Self, scale: F) { *dest = self / scale; }
  pub fn conjugate(&self) -> Quaternion<F> {
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
  fn to_slice(&self) -> [F; 4] { [self.w, self.x, self.y, self.z] }
}

impl<F: RFloat + fmt::Display> fmt::Display for Vector2f<F> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector2f(x:{}, y:{})", self.x, self.y)
  }
}

impl<F: RFloat + fmt::Display> fmt::Display for Vector3f<F> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector3f(x:{}, y:{}, z:{})", self.x, self.y, self.z)
  }
}

impl<F: RFloat + fmt::Display> fmt::Display for Quaternion<F> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "Quaternion({} + {}i + {}j + {}k)", self.w, self.x, self.y, self.z)
  }
}

impl<F: RFloat> PartialEq for Vector2f<F> {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
impl<F: RFloat> Eq for Vector2f<F> {}

impl<F: RFloat> PartialEq for Vector3f<F> {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}
impl<F: RFloat> Eq for Vector3f<F> {}

impl<F: RFloat> PartialEq for Quaternion<F> {
  fn eq(&self, other: &Self) -> bool {
    self.w == other.w && self.x == other.x && self.y == other.y && self.z == other.z
  }
}
impl<F: RFloat> Eq for Quaternion<F> {}

impl<F: RFloat> Add for Vector2f<F> {
  type Output = Vector2f<F>;
  
  fn add(self, other: Vector2f<F>) -> Vector2f<F> {
    Self::Output {x: self.x + other.x, y: self.y + other.y}
  }
}

impl<F: RFloat> Add for Vector3f<F> {
  type Output = Vector3f<F>;
  
  fn add(self, other: Vector3f<F>) -> Vector3f<F> {
    Self::Output {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

impl<F: RFloat> Add for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn add(self, other: Quaternion<F>) -> Self::Output {
    Self::Output {
      w: self.w + other.w,
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl<F: RFloat> Add<F> for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn add(self, other: F) -> Self::Output {
    Self::Output {
      w: self.w + other,
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}

impl<F: RFloat> Add<Quaternion<F>> for f32 {
  type Output = Quaternion<F>;
  
  #[inline]
  fn add(self, other: Quaternion<F>) -> Self::Output {
    Self::Output {
      w: other.w + NumCast::from(self).unwrap(),
      x: other.x,
      y: other.y,
      z: other.z,
    }
  }
}

impl<F: RFloat> Add<Quaternion<F>> for f64 {
  type Output = Quaternion<F>;
  
  #[inline]
  fn add(self, other: Quaternion<F>) -> Self::Output {
    Self::Output {
      w: other.w + NumCast::from(self).unwrap(),
      x: other.x,
      y: other.y,
      z: other.z,
    }
  }
}

impl<F: RFloat + AddAssign> AddAssign for Vector2f<F> {
  fn add_assign(&mut self, other: Vector2f<F>) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl<F: RFloat + AddAssign> AddAssign for Vector3f<F> {
  fn add_assign(&mut self, other: Vector3f<F>) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl<F: RFloat + AddAssign> AddAssign for Quaternion<F> {
  fn add_assign(&mut self, other: Quaternion<F>) {
    self.w += other.w;
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl<F: RFloat> Sub for Vector2f<F> {
  type Output = Vector2f<F>;
  
  fn sub(self, other: Vector2f<F>) -> Vector2f<F> {
    Self::Output {x: self.x - other.x, y: self.y - other.y}
  }
}

impl<F: RFloat> Sub for Vector3f<F> {
  type Output = Vector3f<F>;
  
  fn sub(self, other: Vector3f<F>) -> Vector3f<F> {
    Self::Output {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

impl<F: RFloat> Sub for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn sub(self, other: Quaternion<F>) -> Self::Output {
    Self::Output {
      w: self.w - other.w,
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
impl<F: RFloat> Sub<F> for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn sub(self, other: F) -> Self::Output {
    Self::Output {
      w: self.w - other,
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}
impl<F: RFloat> Sub<Quaternion<F>> for f32 {
  type Output = Quaternion<F>;
  
  #[inline]
  fn sub(self, other: Quaternion<F>) -> Self::Output {
    Self::Output {
      w: other.w - NumCast::from(self).unwrap(),
      x: other.x,
      y: other.y,
      z: other.z,
    }
  }
}

impl<F: RFloat + SubAssign> SubAssign for Vector2f<F> {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl<F: RFloat + SubAssign> SubAssign for Vector3f<F> {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl<F: RFloat + SubAssign> SubAssign for Quaternion<F> {
  fn sub_assign(&mut self, other: Self) {
    self.w -= other.w;
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl<F: RFloat> Mul for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn mul(self, rhs: Quaternion<F>) -> Self::Output {
    Self::Output {
      w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
      x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
      y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
      z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
    }
  }
}

impl<F: RFloat> Mul<F> for Vector2f<F> {
  type Output = Vector2f<F>;
  
  #[inline]
  fn mul(self, other: F) -> Self::Output {
    Self::Output {
      x: self.x * other,
      y: self.y * other,
    }
  }
}
impl<F: RFloat> Mul<F> for Vector3f<F> {
  type Output = Vector3f<F>;
  
  #[inline]
  fn mul(self, other: F) -> Self::Output {
    Self::Output {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

impl<F: RFloat> Mul<F> for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn mul(self, other: F) -> Self::Output {
    Self::Output {
      w: self.w * other,
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

impl<F: RFloat> Mul<Vector2f<F>> for f32 {
  type Output = Vector2f<F>;
  
  #[inline]
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
      x: other.x * NumCast::from(self).unwrap(),
      y: other.y * NumCast::from(self).unwrap(),
    }
  }
}
impl<F: RFloat> Mul<Vector2f<F>> for f64 {
  type Output = Vector2f<F>;
  
  #[inline]
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
      x: other.x * NumCast::from(self).unwrap(),
      y: other.y * NumCast::from(self).unwrap(),
    }
  }
}
impl<F: RFloat> Mul<Vector3f<F>> for f32 {
  type Output = Vector3f<F>;
  
  #[inline]
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
      x: other.x * NumCast::from(self).unwrap(),
      y: other.y * NumCast::from(self).unwrap(),
      z: other.z * NumCast::from(self).unwrap(),
    }
  }
}
impl<F: RFloat> Mul<Vector3f<F>> for f64 {
  type Output = Vector3f<F>;
  
  #[inline]
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
      x: other.x * NumCast::from(self).unwrap(),
      y: other.y * NumCast::from(self).unwrap(),
      z: other.z * NumCast::from(self).unwrap(),
    }
  }
}
impl<F: RFloat> Mul<Quaternion<F>> for f32 {
  type Output = Quaternion<F>;
  
  #[inline]
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
      w: other.w * NumCast::from(self).unwrap(),
      x: other.x * NumCast::from(self).unwrap(),
      y: other.y * NumCast::from(self).unwrap(),
      z: other.z * NumCast::from(self).unwrap(),
    }
  }
}
impl<F: RFloat> Mul<Quaternion<F>> for f64 {
  type Output = Quaternion<F>;
  
  #[inline]
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
      w: other.w * NumCast::from(self).unwrap(),
      x: other.x * NumCast::from(self).unwrap(),
      y: other.y * NumCast::from(self).unwrap(),
      z: other.z * NumCast::from(self).unwrap(),
    }
  }
}

impl<F: RFloat> MulAssign for Quaternion<F> {
  fn mul_assign(&mut self, rhs: Self) {
    self.w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
    self.x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
    self.y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
    self.z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
  }
}

impl<F: RFloat + MulAssign> MulAssign<F> for Vector2f<F> {
  fn mul_assign(&mut self, n: F) {
    self.x *= n;
    self.y *= n;
  }
}

impl<F: RFloat + MulAssign> MulAssign<F> for Vector3f<F> {
  fn mul_assign(&mut self, n: F) {
    self.x *= n;
    self.y *= n;
    self.z *= n;
  }
}

impl<F: RFloat + MulAssign> MulAssign<F> for Quaternion<F> {
  fn mul_assign(&mut self, n: F) {
    self.w *= n;
    self.x *= n;
    self.y *= n;
    self.z *= n;
  }
}

impl<F: RFloat> Div<F> for Vector2f<F> {
  type Output = Vector2f<F>;
  
  #[inline]
  fn div(self, n: F) -> Self::Output {
    Self::Output {
      x: self.x / n,
      y: self.y / n,
    }
  }
}

impl<F: RFloat> Div<F> for Vector3f<F> {
  type Output = Vector3f<F>;
  
  #[inline]
  fn div(self, n: F) -> Self::Output {
    Self::Output {
      x: self.x / n,
      y: self.y / n,
      z: self.z / n,
    }
  }
}

impl<F: RFloat> Div<F> for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn div(self, n: F) -> Self::Output {
    Self::Output {
      w: self.w / n,
      x: self.x / n,
      y: self.y / n,
      z: self.z / n,
    }
  }
}

impl<F: RFloat + DivAssign> DivAssign<F> for Vector2f<F> {
  fn div_assign(&mut self, n: F) {
    self.x /= n;
    self.y /= n;
  }
}

impl<F: RFloat + DivAssign> DivAssign<F> for Vector3f<F> {
  fn div_assign(&mut self, n: F) {
    self.x /= n;
    self.y /= n;
    self.z /= n;
  }
}

impl<F: RFloat + DivAssign> DivAssign<F> for Quaternion<F> {
  fn div_assign(&mut self, n: F) {
    self.w /= n;
    self.x /= n;
    self.y /= n;
    self.z /= n;
  }
}

impl<F: RFloat> Neg for Vector2f<F> {
  type Output = Vector2f<F>;
  
  #[inline]
  fn neg(self) -> Self::Output {
    Self::Output {
      x: -self.x,
      y: -self.y,
    }
  }
}
impl<F: RFloat> Neg for Vector3f<F> {
  type Output = Vector3f<F>;
  
  #[inline]
  fn neg(self) -> Self::Output {
    Self::Output {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
impl<F: RFloat> Neg for Quaternion<F> {
  type Output = Quaternion<F>;
  
  #[inline]
  fn neg(self) -> Self::Output {
    Self::Output {
      w: -self.w,
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
