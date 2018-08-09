use util::rvector::{Vector2f, Vector3f, Vector4f};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign}; // , Div, DivAssign

#[derive(Debug, Copy, Clone)]
pub struct Matrix4f {
  pub matrix: [f32; 16],
}

impl Matrix4f {
  pub fn new() -> Self {
    Matrix4f {
      matrix: [ 1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0_f32],
    }
  }
  pub fn m00(&self) -> f32 { self.matrix[0] }
  pub fn m01(&self) -> f32 { self.matrix[1] }
  pub fn m02(&self) -> f32 { self.matrix[2] }
  pub fn m03(&self) -> f32 { self.matrix[3] }
  pub fn m10(&self) -> f32 { self.matrix[4] }
  pub fn m11(&self) -> f32 { self.matrix[5] }
  pub fn m12(&self) -> f32 { self.matrix[6] }
  pub fn m13(&self) -> f32 { self.matrix[7] }
  pub fn m20(&self) -> f32 { self.matrix[8] }
  pub fn m21(&self) -> f32 { self.matrix[9] }
  pub fn m22(&self) -> f32 { self.matrix[10] }
  pub fn m23(&self) -> f32 { self.matrix[11] }
  pub fn m30(&self) -> f32 { self.matrix[12] }
  pub fn m31(&self) -> f32 { self.matrix[13] }
  pub fn m32(&self) -> f32 { self.matrix[14] }
  pub fn m33(&self) -> f32 { self.matrix[15] }
  pub fn set_m00(&mut self, n: f32) { self.matrix[0] = n; }
  pub fn set_m01(&mut self, n: f32) { self.matrix[1] = n; }
  pub fn set_m02(&mut self, n: f32) { self.matrix[2] = n; }
  pub fn set_m03(&mut self, n: f32) { self.matrix[3] = n; }
  pub fn set_m10(&mut self, n: f32) { self.matrix[4] = n; }
  pub fn set_m11(&mut self, n: f32) { self.matrix[5] = n; }
  pub fn set_m12(&mut self, n: f32) { self.matrix[6] = n; }
  pub fn set_m13(&mut self, n: f32) { self.matrix[7] = n; }
  pub fn set_m20(&mut self, n: f32) { self.matrix[8] = n; }
  pub fn set_m21(&mut self, n: f32) { self.matrix[9] = n; }
  pub fn set_m22(&mut self, n: f32) { self.matrix[10] = n; }
  pub fn set_m23(&mut self, n: f32) { self.matrix[11] = n; }
  pub fn set_m30(&mut self, n: f32) { self.matrix[12] = n; }
  pub fn set_m31(&mut self, n: f32) { self.matrix[13] = n; }
  pub fn set_m32(&mut self, n: f32) { self.matrix[14] = n; }
  pub fn set_m33(&mut self, n: f32) { self.matrix[15] = n; }
  
  pub fn as_slice(&self) -> [f32; 16] {
    self.matrix
  }
  
  pub fn set_identity(&mut self) {
    self.matrix = [ 1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0_f32];
  }
  
  pub fn set_zero(&mut self) {
    self.matrix = [ 0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0_f32];
  }
  
  pub fn index_assign(&mut self, idx: usize, value: &Option<f32>) {
    match (idx, *value) {
      (i, Some(val)) if i < 16 => self.matrix[i] = val,
      ( _, None ) => (),
      ( _, Some(_) ) => ()
    }
  }
  
  pub fn from_vec(&mut self, src: [Option<f32>; 16]) {
    for i in 0..16 {
      self.index_assign(i, &src[i])
    }
  }
  
  pub fn from_m4f(&mut self, src: &Matrix4f) {
    for i in 0..16 {
      self.matrix[i] = src.matrix[i];
    }
  }

  pub fn determinant(&self) -> f32 {
    (self.m00() * (
      ( self.m11() * self.m22() * self.m33() + self.m12() * self.m23() * self.m31() + self.m13() * self.m21() * self.m32() )
      - self.m13() * self.m22() * self.m31() - self.m11() * self.m23() * self.m32() - self.m12() * self.m21() * self.m33() ))
    - (self.m01() * (
      ( self.m10() * self.m22() * self.m33() + self.m12() * self.m23() * self.m30() + self.m13() * self.m20() * self.m32() )
      - self.m13() * self.m22() * self.m30() - self.m10() * self.m23() * self.m32() - self.m12() * self.m20() * self.m33() ))
    + (self.m02() * (
      ( self.m10() * self.m21() * self.m33() + self.m11() * self.m23() * self.m30() + self.m13() * self.m20() * self.m31() )
      - self.m13() * self.m21() * self.m30() - self.m10() * self.m23() * self.m31() - self.m11() * self.m20() * self.m33() ))
    - (self.m03() * (
      ( self.m10() * self.m21() * self.m32() + self.m11() * self.m22() * self.m30() + self.m12() * self.m20() * self.m31() )
      - self.m12() * self.m21() * self.m30() - self.m10() * self.m22() * self.m31() - self.m11() * self.m20() * self.m32() ))
  }
  
  pub fn invert(&mut self) -> bool {
    let determinant = self.determinant();
    if determinant == 0.0_f32 { return false }
    let tmp = invert_math(self, 1_f32 / determinant);
    self.from_vec(tmp);
    true
  }
  
  pub fn invert_from(&mut self, src: &Matrix4f) -> bool {
    let determinant = src.determinant();
    if determinant == 0.0_f32 { return false }
    self.from_vec(invert_math(src, 1_f32 / determinant));
    true
  }
  
  pub fn invert_to(&self, dest: &mut Matrix4f) {
    dest.invert_from(self);
  }
  
  pub fn negate(&mut self) {
    for i in 0..16 {
      self.matrix[i] = -self.matrix[i];
    }
  }
  
  pub fn negate_from(&mut self, src: &Matrix4f) {
    for i in 0..16 {
      self.matrix[i] = -src.matrix[i];
    }
  }
  
  pub fn negate_to(&self, dest: &mut Matrix4f) { dest.negate_from(self); }
  
  pub fn rotate(&mut self, angle: f32, axis: &Vector3f) {
    let tmp = rotate_math(angle, axis, self);
    self.from_vec(tmp);
  }
  
  pub fn rotate_from(&mut self, angle: f32, axis: &Vector3f, src: &Matrix4f) { self.from_vec(rotate_math(angle, axis, src)); }
  
  pub fn rotate_to(&self, angle: f32, axis: &Vector3f, dest: &mut Matrix4f) { dest.from_vec(rotate_math(angle, axis, self)); }
  
  pub fn scale(&mut self, vec: &Vector3f) {
    let tmp = scale_math(vec, self);
    self.from_vec(tmp);
  }
  
  pub fn scale_to(&self, vec: &Vector3f, dest: &mut Matrix4f) { dest.from_vec(scale_math(vec, self)) }
  
  pub fn translate_v2f(&mut self, vec: &Vector2f) {
    let tmp = translate_math_v2f(vec, self);
    self.from_vec(tmp);
  }
  
  pub fn translate_from_v2f(&mut self, vec: &Vector2f, src: &Matrix4f) { self.from_vec(translate_math_v2f(vec, src)) }
  
  pub fn translate_to_v2f(&self, vec: &Vector2f, dest: &mut Matrix4f) { dest.translate_from_v2f(vec, self) }
  
  pub fn translate_v3f(&mut self, vec: &Vector3f) {
    let tmp = translate_math_v3f(vec, self);
    self.from_vec(tmp);
  }
  
  pub fn translate_from_v3f(&mut self, vec: &Vector3f, src: &Matrix4f) { self.from_vec(translate_math_v3f(vec, src)) }
  
  pub fn translate_to_v3f(&self, vec: &Vector3f, dest: &mut Matrix4f) { dest.translate_from_v3f(vec, self) }
  
  pub fn transpose(&mut self) {
    let tmp = transpose_math(self);
    self.from_vec(tmp);
  }
  
  pub fn transpose_from(&mut self, src: &Matrix4f) { self.from_vec(transpose_math(src)); }
  
  pub fn transpose_to(&self, dest: &mut Matrix4f) { dest.transpose_from(self); }
  
  pub fn to_string(&self) -> String {
    format!("[[{}, {}, {}, {}],\n [{}, {}, {}, {}],\n [{}, {}, {}, {}],\n [{}, {}, {}, {}]]",
      self.m00(), self.m01(), self.m02(), self.m03(),
      self.m10(), self.m11(), self.m12(), self.m13(),
      self.m20(), self.m21(), self.m22(), self.m23(),
      self.m30(), self.m31(), self.m32(), self.m33())
  }
}

impl Add for Matrix4f {
  type Output = Matrix4f;
  
  fn add(self, other: Matrix4f) -> Matrix4f {
    Matrix4f {
      matrix: [ self.m00() + other.m00(), self.m01() + other.m01(), self.m02() + other.m02(), self.m03() + other.m03(),
                self.m10() + other.m10(), self.m11() + other.m11(), self.m12() + other.m12(), self.m13() + other.m13(),
                self.m20() + other.m20(), self.m21() + other.m21(), self.m22() + other.m22(), self.m23() + other.m23(),
                self.m30() + other.m30(), self.m31() + other.m31(), self.m32() + other.m32(), self.m33() + other.m33()],
    }
  }
}

impl AddAssign for Matrix4f {
  fn add_assign(&mut self, other: Matrix4f) {
    self.matrix = self.add(other).matrix.clone();
  }
}

impl Sub for Matrix4f {
  type Output = Matrix4f;
  
  fn sub(self, other: Matrix4f) -> Matrix4f {
    Matrix4f {
      matrix: [ self.m00() - other.m00(), self.m01() - other.m01(), self.m02() - other.m02(), self.m03() - other.m03(),
                self.m10() - other.m10(), self.m11() - other.m11(), self.m12() - other.m12(), self.m13() - other.m13(),
                self.m20() - other.m20(), self.m21() - other.m21(), self.m22() - other.m22(), self.m23() - other.m23(),
                self.m30() - other.m30(), self.m31() - other.m31(), self.m32() - other.m32(), self.m33() - other.m33()],
    }
  }
}

impl SubAssign for Matrix4f {
  fn sub_assign(&mut self, other: Matrix4f) {
    self.matrix = self.sub(other).matrix.clone();
  }
}

impl Mul for Matrix4f {
  type Output = Matrix4f;
  
  fn mul(self, other: Matrix4f) -> Matrix4f {
    Matrix4f {
      matrix: [ self.m00() * other.m00() + self.m10() * other.m01() + self.m20() * other.m02() + self.m30() * other.m03(),
                self.m01() * other.m00() + self.m11() * other.m01() + self.m21() * other.m02() + self.m31() * other.m03(),
                self.m02() * other.m00() + self.m12() * other.m01() + self.m22() * other.m02() + self.m32() * other.m03(),
                self.m03() * other.m00() + self.m13() * other.m01() + self.m23() * other.m02() + self.m33() * other.m03(),
                self.m00() * other.m10() + self.m10() * other.m11() + self.m20() * other.m12() + self.m30() * other.m13(),
                self.m01() * other.m10() + self.m11() * other.m11() + self.m21() * other.m12() + self.m31() * other.m13(),
                self.m02() * other.m10() + self.m12() * other.m11() + self.m22() * other.m12() + self.m32() * other.m13(),
                self.m03() * other.m10() + self.m13() * other.m11() + self.m23() * other.m12() + self.m33() * other.m13(),
                self.m00() * other.m20() + self.m10() * other.m21() + self.m20() * other.m22() + self.m30() * other.m23(),
                self.m01() * other.m20() + self.m11() * other.m21() + self.m21() * other.m22() + self.m31() * other.m23(),
                self.m02() * other.m20() + self.m12() * other.m21() + self.m22() * other.m22() + self.m32() * other.m23(),
                self.m03() * other.m20() + self.m13() * other.m21() + self.m23() * other.m22() + self.m33() * other.m23(),
                self.m00() * other.m30() + self.m10() * other.m31() + self.m20() * other.m32() + self.m30() * other.m33(),
                self.m01() * other.m30() + self.m11() * other.m31() + self.m21() * other.m32() + self.m31() * other.m33(),
                self.m02() * other.m30() + self.m12() * other.m31() + self.m22() * other.m32() + self.m32() * other.m33(),
                self.m03() * other.m30() + self.m13() * other.m31() + self.m23() * other.m32() + self.m33() * other.m33()],
    }
  }
}

impl MulAssign for Matrix4f {
  fn mul_assign(&mut self, other: Matrix4f) {
    let t00 = self.m00() * other.m00() + self.m10() * other.m01() + self.m20() * other.m02() + self.m30() * other.m03();
    let t01 = self.m01() * other.m00() + self.m11() * other.m01() + self.m21() * other.m02() + self.m31() * other.m03();
    let t02 = self.m02() * other.m00() + self.m12() * other.m01() + self.m22() * other.m02() + self.m32() * other.m03();
    let t03 = self.m03() * other.m00() + self.m13() * other.m01() + self.m23() * other.m02() + self.m33() * other.m03();
    let t10 = self.m00() * other.m10() + self.m10() * other.m11() + self.m20() * other.m12() + self.m30() * other.m13();
    let t11 = self.m01() * other.m10() + self.m11() * other.m11() + self.m21() * other.m12() + self.m31() * other.m13();
    let t12 = self.m02() * other.m10() + self.m12() * other.m11() + self.m22() * other.m12() + self.m32() * other.m13();
    let t13 = self.m03() * other.m10() + self.m13() * other.m11() + self.m23() * other.m12() + self.m33() * other.m13();
    let t20 = self.m00() * other.m20() + self.m10() * other.m21() + self.m20() * other.m22() + self.m30() * other.m23();
    let t21 = self.m01() * other.m20() + self.m11() * other.m21() + self.m21() * other.m22() + self.m31() * other.m23();
    let t22 = self.m02() * other.m20() + self.m12() * other.m21() + self.m22() * other.m22() + self.m32() * other.m23();
    let t23 = self.m03() * other.m20() + self.m13() * other.m21() + self.m23() * other.m22() + self.m33() * other.m23();
    let t30 = self.m00() * other.m30() + self.m10() * other.m31() + self.m20() * other.m32() + self.m30() * other.m33();
    let t31 = self.m01() * other.m30() + self.m11() * other.m31() + self.m21() * other.m32() + self.m31() * other.m33();
    let t32 = self.m02() * other.m30() + self.m12() * other.m31() + self.m22() * other.m32() + self.m32() * other.m33();
    let t33 = self.m03() * other.m30() + self.m13() * other.m31() + self.m23() * other.m32() + self.m33() * other.m33();
    self.matrix[0] = t00;
    self.matrix[1] = t01;
    self.matrix[2] = t02;
    self.matrix[3] = t03;
    self.matrix[4] = t10;
    self.matrix[5] = t11;
    self.matrix[6] = t12;
    self.matrix[7] = t13;
    self.matrix[8] = t20;
    self.matrix[9] = t21;
    self.matrix[10] = t22;
    self.matrix[11] = t23;
    self.matrix[12] = t30;
    self.matrix[13] = t31;
    self.matrix[14] = t32;
    self.matrix[15] = t33;
  }
}

pub fn add(left: &Matrix4f, right: &Matrix4f, dest: &mut Matrix4f) {
  dest.matrix = [ left.m00() + right.m00(), left.m01() + right.m01(), left.m02() + right.m02(), left.m03() + right.m03(),
                  left.m10() + right.m10(), left.m11() + right.m11(), left.m12() + right.m12(), left.m13() + right.m13(),
                  left.m20() + right.m20(), left.m21() + right.m21(), left.m22() + right.m22(), left.m23() + right.m23(),
                  left.m30() + right.m30(), left.m31() + right.m31(), left.m32() + right.m32(), left.m33() + right.m33()];
}

pub fn sub(left: &Matrix4f, right: &Matrix4f, dest: &mut Matrix4f) {
  dest.matrix = [ left.m00() - right.m00(), left.m01() - right.m01(), left.m02() - right.m02(), left.m03() - right.m03(),
                  left.m10() - right.m10(), left.m11() - right.m11(), left.m12() - right.m12(), left.m13() - right.m13(),
                  left.m20() - right.m20(), left.m21() - right.m21(), left.m22() - right.m22(), left.m23() - right.m23(),
                  left.m30() - right.m30(), left.m31() - right.m31(), left.m32() - right.m32(), left.m33() - right.m33()]
}

pub fn mul(left: &Matrix4f, right: &Matrix4f, dest: &mut Matrix4f) {
  dest.matrix = [ left.m00() * right.m00() + left.m10() * right.m01() + left.m20() * right.m02() + left.m30() * right.m03(),
                  left.m01() * right.m00() + left.m11() * right.m01() + left.m21() * right.m02() + left.m31() * right.m03(),
                  left.m02() * right.m00() + left.m12() * right.m01() + left.m22() * right.m02() + left.m32() * right.m03(),
                  left.m03() * right.m00() + left.m13() * right.m01() + left.m23() * right.m02() + left.m33() * right.m03(),
                  left.m00() * right.m10() + left.m10() * right.m11() + left.m20() * right.m12() + left.m30() * right.m13(),
                  left.m01() * right.m10() + left.m11() * right.m11() + left.m21() * right.m12() + left.m31() * right.m13(),
                  left.m02() * right.m10() + left.m12() * right.m11() + left.m22() * right.m12() + left.m32() * right.m13(),
                  left.m03() * right.m10() + left.m13() * right.m11() + left.m23() * right.m12() + left.m33() * right.m13(),
                  left.m00() * right.m20() + left.m10() * right.m21() + left.m20() * right.m22() + left.m30() * right.m23(),
                  left.m01() * right.m20() + left.m11() * right.m21() + left.m21() * right.m22() + left.m31() * right.m23(),
                  left.m02() * right.m20() + left.m12() * right.m21() + left.m22() * right.m22() + left.m32() * right.m23(),
                  left.m03() * right.m20() + left.m13() * right.m21() + left.m23() * right.m22() + left.m33() * right.m23(),
                  left.m00() * right.m30() + left.m10() * right.m31() + left.m20() * right.m32() + left.m30() * right.m33(),
                  left.m01() * right.m30() + left.m11() * right.m31() + left.m21() * right.m32() + left.m31() * right.m33(),
                  left.m02() * right.m30() + left.m12() * right.m31() + left.m22() * right.m32() + left.m32() * right.m33(),
                  left.m03() * right.m30() + left.m13() * right.m31() + left.m23() * right.m32() + left.m33() * right.m33()];
}

fn determinant3x3(t00: f32, t01: f32, t02: f32, t10: f32, t11: f32, t12: f32, t20: f32, t21: f32, t22: f32) -> f32 {
  t00 * (t11 * t22 - t12 * t21) + t01 * (t12 * t20 - t10 * t22) + t02 * (t10 * t21 - t11 * t20)
}

fn invert_math(src: &Matrix4f, di: f32) -> [Option<f32>; 16] {
  /*
  * m00 m01 m02 m03
  * m10 m11 m12 m13
  * m20 m21 m22 m23
  * m30 m31 m32 m33
  **/
  // transpose
  //  m00 = t00
  //  m01 = t10
  //  m02 = t20
  //  m03 = t30
  //  m10 = t01
  //  m11 = t11
  //  m12 = t21
  //  m13 = t31
  //  m20 = t02
  //  m21 = t12
  //  m22 = t22
  //  m23 = t32
  //  m30 = t03
  //  m31 = t13
  //  m32 = t23
  //  m33 = t33
  [
    Some(determinant3x3(src.m11(), src.m12(), src.m13(), src.m21(), src.m22(), src.m23(), src.m31(), src.m32(), src.m33()) * di),  //00
    Some(-determinant3x3(src.m01(), src.m02(), src.m03(), src.m21(), src.m22(), src.m23(), src.m31(), src.m32(), src.m33()) * di), //10
    Some(determinant3x3(src.m01(), src.m02(), src.m03(), src.m11(), src.m12(), src.m13(), src.m31(), src.m32(), src.m33()) * di),  //20
    Some(-determinant3x3(src.m01(), src.m02(), src.m03(), src.m11(), src.m12(), src.m13(), src.m21(), src.m22(), src.m23()) * di), //30
    Some(-determinant3x3(src.m10(), src.m12(), src.m13(), src.m20(), src.m22(), src.m23(), src.m30(), src.m32(), src.m33()) * di), //01
    Some(determinant3x3(src.m00(), src.m02(), src.m03(), src.m20(), src.m22(), src.m23(), src.m30(), src.m32(), src.m33()) * di),  //11
    Some(-determinant3x3(src.m00(), src.m02(), src.m03(), src.m10(), src.m12(), src.m13(), src.m30(), src.m32(), src.m33()) * di), //21
    Some(determinant3x3(src.m00(), src.m02(), src.m03(), src.m10(), src.m12(), src.m13(), src.m20(), src.m22(), src.m23()) * di),  //31
    Some(determinant3x3(src.m10(), src.m11(), src.m13(), src.m20(), src.m21(), src.m23(), src.m30(), src.m31(), src.m33()) * di),  //02
    Some(-determinant3x3(src.m00(), src.m01(), src.m03(), src.m20(), src.m21(), src.m23(), src.m30(), src.m31(), src.m33()) * di), //12
    Some(determinant3x3(src.m00(), src.m01(), src.m03(), src.m10(), src.m11(), src.m13(), src.m30(), src.m31(), src.m33()) * di),  //22
    Some(-determinant3x3(src.m00(), src.m01(), src.m03(), src.m10(), src.m11(), src.m13(), src.m20(), src.m21(), src.m23()) * di), //32
    Some(-determinant3x3(src.m10(), src.m11(), src.m12(), src.m20(), src.m21(), src.m22(), src.m30(), src.m31(), src.m32()) * di), //03
    Some(determinant3x3(src.m00(), src.m01(), src.m02(), src.m20(), src.m21(), src.m22(), src.m30(), src.m31(), src.m32()) * di),  //13
    Some(-determinant3x3(src.m00(), src.m01(), src.m02(), src.m10(), src.m11(), src.m12(), src.m30(), src.m31(), src.m32()) * di), //23
    Some(determinant3x3(src.m00(), src.m01(), src.m02(), src.m10(), src.m11(), src.m12(), src.m20(), src.m21(), src.m22()) * di)   //33
  ]
}

fn rotate_math(angle: f32, axis: &Vector3f, src: &Matrix4f) -> [Option<f32>; 16] {
  let c = angle.cos();
  let s = angle.sin();
  let oneminusc = 1_f32 - c;
  let xy = axis.x * axis.y;
  let yz = axis.y * axis.z;
  let xz = axis.x * axis.z;
  let xs = axis.x * s;
  let ys = axis.y * s;
  let zs = axis.z * s;
  let f00 = axis.x * axis.x * oneminusc + c;
  let f01 = xy * oneminusc + zs;
  let f02 = xz * oneminusc - ys;
  // n[3] not used
  let f10 = xy * oneminusc - zs;
  let f11 = axis.y * axis.y * oneminusc + c;
  let f12 = yz * oneminusc + xs;
  // n[7] not used
  let f20 = xz * oneminusc + ys;
  let f21 = yz * oneminusc - xs;
  let f22 = axis.z * axis.z * oneminusc + c;
  [
    Some(src.m00() * f00 + src.m10() * f01 + src.m20() * f02), // m00
    Some(src.m01() * f00 + src.m11() * f01 + src.m21() * f02), // m01
    Some(src.m02() * f00 + src.m12() * f01 + src.m22() * f02), // m02
    Some(src.m03() * f00 + src.m13() * f01 + src.m23() * f02), // m03
    Some(src.m00() * f10 + src.m10() * f11 + src.m20() * f12), // m10
    Some(src.m01() * f10 + src.m11() * f11 + src.m21() * f12), // m11
    Some(src.m02() * f10 + src.m12() * f11 + src.m22() * f12), // m12
    Some(src.m03() * f10 + src.m13() * f11 + src.m23() * f12), // m13
    Some(src.m00() * f20 + src.m10() * f21 + src.m20() * f22), // m20
    Some(src.m01() * f20 + src.m11() * f21 + src.m21() * f22), // m21
    Some(src.m02() * f20 + src.m12() * f21 + src.m22() * f22), // m22
    Some(src.m03() * f20 + src.m13() * f21 + src.m23() * f22), // m23
    None, None, None, None
  ]
}

fn scale_math(vec: &Vector3f, src: &Matrix4f) -> [Option<f32>; 16] {
  [
    Some(src.m00() * vec.x),
    Some(src.m01() * vec.x),
    Some(src.m02() * vec.x),
    Some(src.m03() * vec.x),
    Some(src.m10() * vec.y),
    Some(src.m11() * vec.y),
    Some(src.m12() * vec.y),
    Some(src.m13() * vec.y),
    Some(src.m20() * vec.z),
    Some(src.m21() * vec.z),
    Some(src.m22() * vec.z),
    Some(src.m23() * vec.z),
    None, None, None, None
  ]
}

pub fn transform(left: &Matrix4f, right: &Vector4f, dest: &mut Vector4f) {
  dest.x = left.m00() * right.x + left.m10() * right.y + left.m20() * right.z + left.m30() * right.w;
  dest.y = left.m01() * right.x + left.m11() * right.y + left.m21() * right.z + left.m31() * right.w;
  dest.z = left.m02() * right.x + left.m12() * right.y + left.m22() * right.z + left.m32() * right.w;
  dest.w = left.m03() * right.x + left.m13() * right.y + left.m23() * right.z + left.m33() * right.w;
}

pub fn translate_math_v3f(vec: &Vector3f, src: &Matrix4f) -> [Option<f32>; 16] {
  [
    None, None, None, None,
    None, None, None, None,
    None, None, None, None,
    Some(src.m30() + (src.m00() * vec.x + src.m10() * vec.y + src.m20() * vec.z)),
    Some(src.m31() + (src.m01() * vec.x + src.m11() * vec.y + src.m21() * vec.z)),
    Some(src.m32() + (src.m02() * vec.x + src.m12() * vec.y + src.m22() * vec.z)),
    Some(src.m33() + (src.m03() * vec.x + src.m13() * vec.y + src.m23() * vec.z))
  ]
}

pub fn translate_math_v2f(vec: &Vector2f, src: &Matrix4f) -> [Option<f32>; 16] {
  [
    None, None, None, None,
    None, None, None, None,
    None, None, None, None,
    Some(src.m30() + (src.m00() * vec.x + src.m10() * vec.y)),
    Some(src.m31() + (src.m01() * vec.x + src.m11() * vec.y)),
    Some(src.m32() + (src.m02() * vec.x + src.m12() * vec.y)),
    Some(src.m33() + (src.m03() * vec.x + src.m13() * vec.y))
  ]
}

pub fn transpose_math(src: &Matrix4f) -> [Option<f32>; 16] {
  [
    Some(src.m00()), Some(src.m10()), Some(src.m20()), Some(src.m30()),
    Some(src.m01()), Some(src.m11()), Some(src.m21()), Some(src.m31()),
    Some(src.m02()), Some(src.m12()), Some(src.m22()), Some(src.m32()),
    Some(src.m03()), Some(src.m13()), Some(src.m23()), Some(src.m33())
  ]
}

//pub fn load(buf: FloatBuffer) {
//  self.matrix[0] = buf.get() as f32;
//  self.matrix[1] = buf.get() as f32;
//  self.matrix[2] = buf.get() as f32;
//  self.matrix[3] = buf.get() as f32;
//  self.matrix[4] = buf.get() as f32;
//  self.matrix[5] = buf.get() as f32;
//  self.matrix[6] = buf.get() as f32;
//  self.matrix[7] = buf.get() as f32;
//  self.matrix[8] = buf.get() as f32;
//  self.matrix[9] = buf.get() as f32;
//  self.matrix[10] = buf.get() as f32;
//  self.matrix[11] = buf.get() as f32;
//  self.matrix[12] = buf.get() as f32;
//  self.matrix[13] = buf.get() as f32;
//  self.matrix[14] = buf.get() as f32;
//  self.matrix[15] = buf.get() as f32;
//}
//
//pub fn load_transpose(buf: FloatBuffer) {
//  self.matrix[0] = buf.get() as f32;
//  self.matrix[4] = buf.get() as f32;
//  self.matrix[8] = buf.get() as f32;
//  self.matrix[12] = buf.get() as f32;
//  self.matrix[1] = buf.get() as f32;
//  self.matrix[5] = buf.get() as f32;
//  self.matrix[9] = buf.get() as f32;
//  self.matrix[13] = buf.get() as f32;
//  self.matrix[2] = buf.get() as f32;
//  self.matrix[6] = buf.get() as f32;
//  self.matrix[10] = buf.get() as f32;
//  self.matrix[14] = buf.get() as f32;
//  self.matrix[3] = buf.get() as f32;
//  self.matrix[7] = buf.get() as f32;
//  self.matrix[11] = buf.get() as f32;
//  self.matrix[15] = buf.get() as f32;
//}
