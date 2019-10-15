

use {
  std::cmp::Ordering,
  num::Integer,
  specs::{
    Component, VecStorage, DenseVecStorage, 
  },
  crate::{
    util::{
      RVec, RFloat,
      NumCast, Zero,
      Vector2f, Vector3f, Quaternion, Matrix4f, 
      modulo,
    },
  },
};

// Declarations

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct CamDistance(pub f64);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct DeltaVelocity(pub Vector3f<f32>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GravPercent(pub f32);

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct JumpArc {
  pub orig: Vector3f<f32>,
  pub dest: Vector3f<f32>,
  pub last: Vector3f<f32>,
  pub current: Vector3f<f32>,
  pub delta: Vector3f<f32>,
  pub time: f32,
  pub fin: bool,
  peak: bool,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct LightingName(pub String);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct ModelName(pub String);

// #[derive(Component, Debug)]
// #[storage(VecStorage)]
// pub enum MovementType {
//   MoveForward,
//   MoveBackward,
//   StrafeLeft,
//   StrafeRight,
//   TurnLeft,
//   TurnRight,
//   Jump,
//   // JumpForward,
//   // JumpBackward,
//   // JumpLeft,
//   // JumpRight,
// }

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct PlayerGridLoc(pub i32,pub i32);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct PosAdjust(pub Vector3f<f32>);

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Position(pub Vector3f<f32>);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Rotation(pub Vector3f<f32>);

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Rotator<F: RFloat> {
  q: Quaternion<F>,
  q1: Quaternion<F>,
  p: Quaternion<F>,
  axis: Vector3f<F>,
  theta: F,
  mag: F,
  ops: u32,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct RowCount(pub u32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ScaleFloat(pub f32);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexIndex(pub u32);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexName(pub String);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexOffset(pub Vector2f<f32>);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TexOffsets{
  pub a: Vector2f<f64>,
  pub b: Vector2f<f64>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TimedLife {
  pub total: f64,
  pub elapsed: f64,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TransformVelocity(pub Vector3f<f32>);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3f<f32>);


// Implementations

impl Default for GravPercent {
  fn default() -> Self {
    Self(1.0)
  }
}

impl Default for JumpArc {
  fn default() -> Self {
    Self {
      orig: Vector3f::blank(),
      dest: Vector3f::blank(),
      last: Vector3f::blank(),
      current: Vector3f::blank(),
      delta: Vector3f::blank(),
      time: 0_f32,
      fin: true,
      peak: false,
    }
  }
}
impl JumpArc {
  pub const PEAK: f32 = 3.0;
  const JUMPTIME: f32 = 5.0;
  pub fn new() -> Self {
    Self::default()
  }
  pub fn init(&mut self, _orig: Vector3f<f32>, _dest: Vector3f<f32>) {
    {
      let (orig, last, dest, time) = (
        &mut self.orig, &mut self.last, &mut self.dest, &mut self.time
      );
      *time = 0_f32;
      orig.copy_from_v3f(_orig);
      last.copy_from_v3f(_orig);
      dest.copy_from_v3f(_dest);
      self.peak = false;
      self.fin = false;
    }
    println!("{:?}", self);
  }
  pub fn calc_pos(&mut self, delta: f32) -> Vector3f<f32> {
    if !self.fin {
      let (orig, dest, last, current, time) = (
        &self.orig, &self.dest, &mut self.last, &mut self.current, &mut self.time
      );
      *time += delta; // 5_f32 * 
      last.copy_from_v3f(*current);
      if *time >= Self::JUMPTIME {
        *time = Self::JUMPTIME;
        current.copy_from_v3f(*dest);
        self.fin = true;
      } else {
        let percent = *time / Self::JUMPTIME;
        self.peak = percent > 0.5;
        current.x = orig.x + (percent * ( dest.x - orig.x ));
        current.z = orig.z + (percent * ( dest.z - orig.z ));
        let y = orig.y + (percent * ( dest.y - orig.y));
        current.y = y + (Self::PEAK * if percent < 0.5 { percent * 2.0 } else { (1.0 - percent) * 2.0 });
      }
    }
    self.delta.copy_from_float(self.current.x - self.last.x, self.current.y - self.last.y, self.current.z - self.last.z);
    self.delta
  }
  pub fn check_peak(&mut self) -> bool {
    if self.peak {
      self.peak = false;
      true
    } else {
      false
    }
  }
}

impl PartialEq for ModelName {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
impl Eq for ModelName {}
impl Ord for ModelName {
  fn cmp(&self, other: &Self) -> Ordering {
    if self == other { return Ordering::Equal }
    let mut a: Vec<char> = self.0.chars().collect();
    let mut b: Vec<char> = other.0.chars().collect();
    while !a.is_empty() && !b.is_empty() {
      match (a.pop(), b.pop()) {
        (ac,bc) if bc < ac => { return Ordering::Less } 
        (ac,bc) if bc > ac => { return Ordering::Greater } 
        _ => {}
      };
    };
    if a.is_empty() { Ordering::Greater } else { Ordering::Less }
  }
}
impl PartialOrd for ModelName {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other { return Some(Ordering::Equal) }
    if self < other { Some(Ordering::Less) } else { Some(Ordering::Greater) }
  }
}

impl PosAdjust {
  pub fn clear(&mut self) {
    self.0.clear();
  }
}

impl Position {
  pub fn to_grid(&self) -> (i32, i32) {
    let x = self.0 .x.round() as i32;
    let z = self.0 .z.round() as i32;
    let x = if x.is_odd() { x - 1 } else { x };
    let z = if z.is_odd() { z - 1 } else { z };
    (x, z)
  }
  pub fn grid_fore(&self, rot: &Rotation, dist: i32) -> (i32, i32) {
    grid_move(self.0 .x, self.0 .z, dist as f32, rot.0 .y)
  }
  pub fn grid_left(&self, rot: &Rotation, dist: i32) -> (i32, i32) {
    grid_move(self.0 .x, self.0 .z, dist as f32, modulo(rot.0 .y + 90.0, 360.0))
  }
  pub fn grid_rigt(&self, rot: &Rotation, dist: i32) -> (i32, i32) {
    grid_move(self.0 .x, self.0 .z, dist as f32, modulo(rot.0 .y - 90.0, 360.0))
  }
  pub fn grid_back(&self, rot: &Rotation, dist: i32) -> (i32, i32) {
    grid_move(self.0 .x, self.0 .z, dist as f32, modulo(rot.0 .y - 180.0, 360.0))
  }
}
fn grid_move(x: f32, z: f32, dist: f32, ry: f32) -> (i32, i32) {
  let ry = ry.to_radians();
  let x = (x + (dist * ry.sin())).round() as i32;
  let z = (z + (dist * ry.cos())).round() as i32;
  let x = if x.is_odd() { x - 1 } else { x };
  let z = if z.is_odd() { z - 1 } else { z };
  (x, z)
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
    rx.set_axis(crate::util::XVEC64);
    let mut rz = Rotator::default();
    rz.set_axis(crate::util::ZVEC64);
    Self {
      rx, ry: Rotator::default(), rz,
      rstrange: Rotator::default(),
    }
  }
}

impl Default for ScaleFloat {
  fn default() -> Self {
    Self(1.0)
  }
}

impl TimedLife {
  pub fn inc_time(&mut self, delta: f64) {
    self.elapsed += delta;
  }
  pub fn is_alive(&self) -> bool {
    self.elapsed < self.total
  }
  pub fn set_life(&mut self, life_length: f64) {
    self.elapsed = 0.0;
    self.total = life_length;
  }
}

