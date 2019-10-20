use {
  util::{
    RFloat, NumCast, Zero, 
    rvector::{Vector2f, Vector3f, Quaternion},
  },
  std::{
    fmt,
    ops::{
      Add, AddAssign, Sub, SubAssign, Mul, MulAssign, // , Div, DivAssign, 
    },
  },
};

#[derive(Debug, Copy, Clone)]
pub struct Matrix4f<F: RFloat> {
  pub matrix: [F; 16],
}
impl<F: RFloat> Default for Matrix4f<F> {
  fn default() -> Self {
    Self {
      matrix: [ NumCast::from(1).unwrap(), Zero::zero(), Zero::zero(), Zero::zero(),
                Zero::zero(), NumCast::from(1).unwrap(), Zero::zero(), Zero::zero(),
                Zero::zero(), Zero::zero(), NumCast::from(1).unwrap(), Zero::zero(),
                Zero::zero(), Zero::zero(), Zero::zero(), NumCast::from(1).unwrap()],
    }
  }
}
impl<F: RFloat> Matrix4f<F> {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn m00(&self) -> F { self.matrix[0] }
  pub fn m01(&self) -> F { self.matrix[1] }
  pub fn m02(&self) -> F { self.matrix[2] }
  pub fn m03(&self) -> F { self.matrix[3] }
  pub fn m10(&self) -> F { self.matrix[4] }
  pub fn m11(&self) -> F { self.matrix[5] }
  pub fn m12(&self) -> F { self.matrix[6] }
  pub fn m13(&self) -> F { self.matrix[7] }
  pub fn m20(&self) -> F { self.matrix[8] }
  pub fn m21(&self) -> F { self.matrix[9] }
  pub fn m22(&self) -> F { self.matrix[10] }
  pub fn m23(&self) -> F { self.matrix[11] }
  pub fn m30(&self) -> F { self.matrix[12] }
  pub fn m31(&self) -> F { self.matrix[13] }
  pub fn m32(&self) -> F { self.matrix[14] }
  pub fn m33(&self) -> F { self.matrix[15] }
  pub fn set_m00(&mut self, n: F) { self.matrix[0] = n; }
  pub fn set_m01(&mut self, n: F) { self.matrix[1] = n; }
  pub fn set_m02(&mut self, n: F) { self.matrix[2] = n; }
  pub fn set_m03(&mut self, n: F) { self.matrix[3] = n; }
  pub fn set_m10(&mut self, n: F) { self.matrix[4] = n; }
  pub fn set_m11(&mut self, n: F) { self.matrix[5] = n; }
  pub fn set_m12(&mut self, n: F) { self.matrix[6] = n; }
  pub fn set_m13(&mut self, n: F) { self.matrix[7] = n; }
  pub fn set_m20(&mut self, n: F) { self.matrix[8] = n; }
  pub fn set_m21(&mut self, n: F) { self.matrix[9] = n; }
  pub fn set_m22(&mut self, n: F) { self.matrix[10] = n; }
  pub fn set_m23(&mut self, n: F) { self.matrix[11] = n; }
  pub fn set_m30(&mut self, n: F) { self.matrix[12] = n; }
  pub fn set_m31(&mut self, n: F) { self.matrix[13] = n; }
  pub fn set_m32(&mut self, n: F) { self.matrix[14] = n; }
  pub fn set_m33(&mut self, n: F) { self.matrix[15] = n; }
  
  pub fn as_slice(&self) -> [F; 16] {
    self.matrix
  }
  
  pub fn set_identity(&mut self) {
    self.matrix = [ NumCast::from(1).unwrap(), Zero::zero(), Zero::zero(), Zero::zero(),
                    Zero::zero(), NumCast::from(1).unwrap(), Zero::zero(), Zero::zero(),
                    Zero::zero(), Zero::zero(), NumCast::from(1).unwrap(), Zero::zero(),
                    Zero::zero(), Zero::zero(), Zero::zero(), NumCast::from(1).unwrap() ];
  }
  
  pub fn set_zero(&mut self) {
    self.matrix = [ Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(),
                    Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(),
                    Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(),
                    Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero() ];
  }
  
  pub fn index_assign(&mut self, idx: usize, value: Option<F>) {
    match (idx, value) {
      (i, Some(val)) if i < 16 => self.matrix[i] = val,
      ( _, None ) => (),
      ( _, Some(_) ) => ()
    }
  }
  
  pub fn copy_from_vec(&mut self, src: [Option<F>; 16]) {
    for (i, val) in src.iter().enumerate() {
      self.index_assign(i, *val);
    }
  }
  
  pub fn copy_from_m4f(&mut self, src: &Matrix4f<F>) {
    self.matrix[..16].clone_from_slice(&src.matrix[..16]);
  }
  
  pub fn gen_from_quat(&mut self, q: Quaternion<F>) {
    let two: F = NumCast::from(2).unwrap();
    self.matrix = [ 
      q.w.powi(2) + q.x.powi(2) - q.y.powi(2) - q.z.powi(2),
      (two * q.x * q.y) - (two * q.w * q.z),
      (two * q.x * q.z) + (two * q.w * q.y),
      Zero::zero(),
      
      (two * q.x * q.y) + (two * q.w * q.z),
      q.w.powi(2) - q.x.powi(2) + q.y.powi(2) - q.z.powi(2),
      (two * q.y * q.z) + (two * q.w * q.x),
      Zero::zero(),
      
      (two * q.x * q.z) - (two * q.w * q.y),
      (two * q.y * q.z) - (two * q.w * q.x),
      q.w.powi(2) - q.x.powi(2) - q.y.powi(2) + q.z.powi(2),
      Zero::zero(),
      
      Zero::zero(), Zero::zero(), Zero::zero(), NumCast::from(1).unwrap()];
  }

  pub fn determinant(&self) -> F {
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
    if determinant == Zero::zero() { return false }
    let one: F = NumCast::from(1).unwrap();
    let tmp = invert_math(self, one / determinant);
    self.copy_from_vec(tmp);
    true
  }
  
  pub fn invert_from(&mut self, src: &Matrix4f<F>) -> bool {
    let determinant = src.determinant();
    if determinant == Zero::zero() { return false }
    let one: F = NumCast::from(1).unwrap();
    self.copy_from_vec(invert_math(src, one / determinant));
    true
  }
  
  pub fn invert_to(&self, dest: &mut Matrix4f<F>) {
    dest.invert_from(self);
  }
  
  pub fn negate(&mut self) {
    for i in 0..16 {
      self.matrix[i] = -self.matrix[i];
    }
  }
  
  pub fn negate_from(&mut self, src: &Matrix4f<F>) {
    for i in 0..16 {
      self.matrix[i] = -src.matrix[i];
    }
  }
  
  pub fn negate_to(&self, dest: &mut Matrix4f<F>) { dest.negate_from(self); }
  
  pub fn rotate(&mut self, angle: F, axis: Vector3f<F>) {
    let tmp = rotate_math(angle, axis, self);
    self.copy_from_vec(tmp);
  }
  
  pub fn rotate_from(&mut self, angle: F, axis: Vector3f<F>, src: &Matrix4f<F>) { self.copy_from_vec(rotate_math(angle, axis, src)); }
  
  pub fn rotate_to(&self, angle: F, axis: Vector3f<F>, dest: &mut Matrix4f<F>) { dest.copy_from_vec(rotate_math(angle, axis, self)); }
  
  pub fn scale(&mut self, vec: Vector3f<F>) {
    let tmp = scale_math(vec, self);
    self.copy_from_vec(tmp);
  }
  
  pub fn scale_to(&self, vec: Vector3f<F>, dest: &mut Matrix4f<F>) { dest.copy_from_vec(scale_math(vec, self)) }
  
  pub fn translate_v2f(&mut self, vec: Vector2f<F>) {
    let tmp = translate_math_v2f(vec, self);
    self.copy_from_vec(tmp);
  }
  
  pub fn translate_from_v2f(&mut self, vec: Vector2f<F>, src: &Matrix4f<F>) { self.copy_from_vec(translate_math_v2f(vec, src)) }
  
  pub fn translate_to_v2f(&self, vec: Vector2f<F>, dest: &mut Matrix4f<F>) { dest.translate_from_v2f(vec, self) }
  
  pub fn translate_v3f(&mut self, vec: Vector3f<F>) {
    let tmp = translate_math_v3f(vec, self);
    self.copy_from_vec(tmp);
  }
  
  pub fn translate_from_v3f(&mut self, vec: Vector3f<F>, src: &Matrix4f<F>) { self.copy_from_vec(translate_math_v3f(vec, src)) }
  
  pub fn translate_to_v3f(&self, vec: Vector3f<F>, dest: &mut Matrix4f<F>) { dest.translate_from_v3f(vec, self) }
  
  pub fn transpose(&mut self) {
    let tmp = transpose_math(self);
    self.copy_from_vec(tmp);
  }
  
  pub fn transpose3x3(&mut self, other: &Self) {
    self.set_m00(other.m00());
    self.set_m01(other.m10());
    self.set_m02(other.m20());
    self.set_m10(other.m01());
    self.set_m11(other.m11());
    self.set_m12(other.m21());
    self.set_m20(other.m02());
    self.set_m21(other.m12());
    self.set_m22(other.m22());
  }
  
  pub fn transpose_from(&mut self, src: &Matrix4f<F>) { self.copy_from_vec(transpose_math(src)); }
  
  pub fn transpose_to(&self, dest: &mut Matrix4f<F>) { dest.transpose_from(self); }
}
impl<F: RFloat> fmt::Display for Matrix4f<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "[[{}, {}, {}, {}],\n [{}, {}, {}, {}],\n [{}, {}, {}, {}],\n [{}, {}, {}, {}]]",
      self.m00(), self.m01(), self.m02(), self.m03(),
      self.m10(), self.m11(), self.m12(), self.m13(),
      self.m20(), self.m21(), self.m22(), self.m23(),
      self.m30(), self.m31(), self.m32(), self.m33()
    )
  }
}

impl<F: RFloat> Add for Matrix4f<F> {
  type Output = Matrix4f<F>;
  
  fn add(self, other: Self::Output) -> Self::Output {
    Matrix4f {
      matrix: [ self.m00() + other.m00(), self.m01() + other.m01(), self.m02() + other.m02(), self.m03() + other.m03(),
                self.m10() + other.m10(), self.m11() + other.m11(), self.m12() + other.m12(), self.m13() + other.m13(),
                self.m20() + other.m20(), self.m21() + other.m21(), self.m22() + other.m22(), self.m23() + other.m23(),
                self.m30() + other.m30(), self.m31() + other.m31(), self.m32() + other.m32(), self.m33() + other.m33()],
    }
  }
}

impl<F: RFloat> AddAssign for Matrix4f<F> {
  fn add_assign(&mut self, other: Self) {
    self.matrix = self.add(other).matrix;
  }
}

impl<F: RFloat> Sub for Matrix4f<F> {
  type Output = Matrix4f<F>;
  
  fn sub(self, other: Self::Output) -> Self::Output {
    Self::Output {
      matrix: [ self.m00() - other.m00(), self.m01() - other.m01(), self.m02() - other.m02(), self.m03() - other.m03(),
                self.m10() - other.m10(), self.m11() - other.m11(), self.m12() - other.m12(), self.m13() - other.m13(),
                self.m20() - other.m20(), self.m21() - other.m21(), self.m22() - other.m22(), self.m23() - other.m23(),
                self.m30() - other.m30(), self.m31() - other.m31(), self.m32() - other.m32(), self.m33() - other.m33()],
    }
  }
}

impl<F: RFloat> SubAssign for Matrix4f<F> {
  fn sub_assign(&mut self, other: Self) {
    self.matrix = self.sub(other).matrix;
  }
}

impl<F: RFloat> Mul for Matrix4f<F> {
  type Output = Matrix4f<F>;
  
  fn mul(self, other: Self::Output) -> Self::Output {
    Self::Output {
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

impl<F: RFloat> MulAssign for Matrix4f<F> {
  fn mul_assign(&mut self, other: Matrix4f<F>) {
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

pub fn add<F: RFloat>(left: &Matrix4f<F>, right: &Matrix4f<F>, dest: &mut Matrix4f<F>) {
  dest.matrix = [ left.m00() + right.m00(), left.m01() + right.m01(), left.m02() + right.m02(), left.m03() + right.m03(),
                  left.m10() + right.m10(), left.m11() + right.m11(), left.m12() + right.m12(), left.m13() + right.m13(),
                  left.m20() + right.m20(), left.m21() + right.m21(), left.m22() + right.m22(), left.m23() + right.m23(),
                  left.m30() + right.m30(), left.m31() + right.m31(), left.m32() + right.m32(), left.m33() + right.m33()];
}

pub fn sub<F: RFloat>(left: &Matrix4f<F>, right: &Matrix4f<F>, dest: &mut Matrix4f<F>) {
  dest.matrix = [ left.m00() - right.m00(), left.m01() - right.m01(), left.m02() - right.m02(), left.m03() - right.m03(),
                  left.m10() - right.m10(), left.m11() - right.m11(), left.m12() - right.m12(), left.m13() - right.m13(),
                  left.m20() - right.m20(), left.m21() - right.m21(), left.m22() - right.m22(), left.m23() - right.m23(),
                  left.m30() - right.m30(), left.m31() - right.m31(), left.m32() - right.m32(), left.m33() - right.m33()]
}

pub fn mul<F: RFloat>(left: &Matrix4f<F>, right: &Matrix4f<F>, dest: &mut Matrix4f<F>) {
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

fn determinant3x3<F: RFloat>(t00: F, t01: F, t02: F, t10: F, t11: F, t12: F, t20: F, t21: F, t22: F) -> F {
  t00 * (t11 * t22 - t12 * t21) + t01 * (t12 * t20 - t10 * t22) + t02 * (t10 * t21 - t11 * t20)
}

fn invert_math<F: RFloat>(src: &Matrix4f<F>, di: F) -> [Option<F>; 16] {
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

fn rotate_math<F: RFloat>(angle: F, axis: Vector3f<F>, src: &Matrix4f<F>) -> [Option<F>; 16] {
  // let angle = angle.to_radians();
  let c = angle.cos();
  let s = angle.sin();
  let one: F = NumCast::from(1).unwrap();
  let oneminusc = one - c;
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

fn scale_math<F: RFloat>(vec: Vector3f<F>, src: &Matrix4f<F>) -> [Option<F>; 16] {
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

pub fn transform<F: RFloat>(left: &Matrix4f<F>, right: Quaternion<F>, dest: &mut Quaternion<F>) { // This is prolly wrong and not needed
  dest.w = left.m00() * right.w + left.m10() * right.x + left.m20() * right.y + left.m30() * right.z;
  dest.x = left.m01() * right.w + left.m11() * right.x + left.m21() * right.y + left.m31() * right.z;
  dest.y = left.m02() * right.w + left.m12() * right.x + left.m22() * right.y + left.m32() * right.z;
  dest.z = left.m03() * right.w + left.m13() * right.x + left.m23() * right.y + left.m33() * right.z;
}

pub fn translate_math_v3f<F: RFloat>(vec: Vector3f<F>, src: &Matrix4f<F>) -> [Option<F>; 16] {
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

pub fn translate_math_v2f<F: RFloat>(vec: Vector2f<F>, src: &Matrix4f<F>) -> [Option<F>; 16] {
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

pub fn transpose_math<F: RFloat>(src: &Matrix4f<F>) -> [Option<F>; 16] {
  [
    Some(src.m00()), Some(src.m10()), Some(src.m20()), Some(src.m30()),
    Some(src.m01()), Some(src.m11()), Some(src.m21()), Some(src.m31()),
    Some(src.m02()), Some(src.m12()), Some(src.m22()), Some(src.m32()),
    Some(src.m03()), Some(src.m13()), Some(src.m23()), Some(src.m33())
  ]
}
