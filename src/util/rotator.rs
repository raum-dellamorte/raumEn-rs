
use {
  
  crate::{
    util::{
      Matrix4f, Quaternion, Vector3f, 
      RFloat, RVec, 
      NumCast, Zero, 
    },
  },
};


pub struct Rotator<F: RFloat> {
  q: Quaternion<F>,
  q1: Quaternion<F>,
  p: Quaternion<F>,
  axis: Vector3f<F>,
  theta: F,
  mag: F,
  ops: u32,
}
impl<F: RFloat> Default for Rotator<F> {
  fn default() -> Self {
    Self {
      q: Quaternion::default(),
      q1: Quaternion::default(),
      p: Quaternion::blank(),
      axis: Vector3f {x: Zero::zero(), y: NumCast::from(1).unwrap(), z: Zero::zero() },
      theta: Zero::zero(),
      mag: NumCast::from(1).unwrap(),
      ops: 0,
    }
  }
}
impl<F: RFloat> Rotator<F> {
  pub fn calibrate(&mut self) -> &mut Self {
    let one: F = NumCast::from(1).unwrap();
    let tolerance: F = NumCast::from(0.00001).unwrap();
    if (self.p.len_sqr() - one ).abs() > tolerance {
      self.mag = self.p.len();
      if self.mag != Zero::zero() { self.p.divscale(self.mag); }
    };
    self
  }
  pub fn auto_cal(&mut self) -> &mut Self {
    // The way I'm using the rotator I think this is completely unnecessary
    // self.p is getting overwritten all the time, so there's no reason to 
    // normalize it.  The math seems to work fine without being normalized.
    // It seemed like a good idea at the time.  I just wasn't paying
    // attention to what is actually happening.
    if self.ops > 10_0000 { // 十万
      self.calibrate();
      self.ops = 0;
    }
    self
  }
  pub fn set_axis(&mut self, axis: Vector3f<F>) -> &mut Self {
    let one: F = NumCast::from(1).unwrap();
    let tolerance: F = NumCast::from(0.0001).unwrap();
    self.axis = axis;
    if (self.axis.len_sqr() - one ).abs() > tolerance {
      let mag = self.axis.len();
      if mag != Zero::zero() { self.axis.divscale(mag); }
    };
    self
  }
  pub fn set_point(&mut self, point: Vector3f<F>) -> &mut Self {
    self.p.w = Zero::zero();
    self.p.x = point.x;
    self.p.y = point.y;
    self.p.z = point.z;
    self
  }
  pub fn set_angle(&mut self, theta: F) -> &mut Self {
    self.theta = theta;
    self
  }
  pub fn rotate(&mut self) -> &mut Self {
    let theta = self.theta.to_radians() / NumCast::from(2).unwrap();
    self.q.w = theta.cos();
    self.q.x = self.axis.x * theta.sin();
    self.q.y = self.axis.y * theta.sin();
    self.q.z = self.axis.z * theta.sin();
    self.q1 = self.q.conjugate();
    self.p = (self.q * self.p) * self.q1;
    self.ops += 1;
    self
  }
  pub fn get_point<R: RFloat>(&mut self, dest: &mut Vector3f<R>) -> &mut Self {
    let mag: R = NumCast::from(self.mag).unwrap();
    let px: R = NumCast::from(self.p.x).unwrap();
    let py: R = NumCast::from(self.p.y).unwrap();
    let pz: R = NumCast::from(self.p.z).unwrap();
    dest.x = px * mag;
    dest.y = py * mag;
    dest.z = pz * mag;
    self
  }
  pub fn get_matrix(&self, dest: &mut Matrix4f<F>) {
    dest.gen_from_quat(self.p);
  }
}
pub struct Rotators {
  pub rx: Rotator<f64>,
  pub ry: Rotator<f64>,
  pub rz: Rotator<f64>,
  pub rstrange: Rotator<f64>,
}
impl Default for Rotators {
  fn default() -> Self {
    let mut rx = Rotator::default();
    rx.set_axis(crate::constants::XVEC64);
    let mut rz = Rotator::default();
    rz.set_axis(crate::constants::ZVEC64);
    Self {
      rx, ry: Rotator::default(), rz,
      rstrange: Rotator::default(),
    }
  }
}
