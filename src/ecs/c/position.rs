
use {
  num::Integer,
  specs::{
    Component, VecStorage, DenseVecStorage
  },
  util::{
    RVec,
    Vector3f,
    Quaternion,
    Matrix4f,
    modulo,
  },
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Vector3f);
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rotation(pub Vector3f);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3f);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformVelocity(pub Vector3f);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct DeltaVelocity(pub Vector3f);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PosAdjust(pub Vector3f);

impl PosAdjust {
  pub fn clear(&mut self) {
    self.0.clear();
  }
}

#[derive(Debug)]
pub struct PlayerLoc(pub i32,pub i32);
impl Default for PlayerLoc { fn default() -> Self { Self(0,0) } }

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct JumpArc {
  pub orig: Vector3f,
  pub dest: Vector3f,
  pub last: Vector3f,
  pub current: Vector3f,
  pub delta: Vector3f,
  pub time: f32,
  pub fin: bool,
  peak: bool,
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
  pub fn init(&mut self, _orig: Vector3f, _dest: Vector3f) {
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
  pub fn calc_pos(&mut self, delta: f32) -> Vector3f {
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
    self.delta.copy_from_f32(self.current.x - self.last.x, self.current.y - self.last.y, self.current.z - self.last.z);
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

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Rotator {
  q: Quaternion,
  q1: Quaternion,
  p: Quaternion,
  axis: Vector3f,
  theta: f32,
  mag: f32,
}
impl Default for Rotator {
  fn default() -> Self {
    Self {
      q: Quaternion::new(1.,0.,0.,0.),
      q1: Quaternion::new(1.,0.,0.,0.),
      p: Quaternion::new(0.,0.,0.,0.),
      axis: crate::util::YVEC,
      theta: 0.,
      mag: 1.,
    }
  }
}
impl Rotator {
  pub fn calibrate(&mut self) -> &mut Self {
    if (self.p.len_sqr() - 1.).abs() > 0.00001 {
      self.mag = self.p.len();
      if self.mag != 0. { self.p.divscale(self.mag); }
    };
    self
  }
  pub fn set_point(&mut self, point: Vector3f) -> &mut Self {
    self.p.w = 0.;
    self.p.x = point.x;
    self.p.y = point.y;
    self.p.z = point.z;
    self
  }
  pub fn set_axis(&mut self, axis: Vector3f) -> &mut Self {
    self.axis = axis;
    if (self.axis.len_sqr() - 1.).abs() > 0.0001 {
      let mag = self.axis.len();
      if mag != 0. { self.axis.divscale(mag); }
    };
    self
  }
  pub fn set_angle(&mut self, theta: f32) -> &mut Self {
    self.theta = theta;
    self
  }
  pub fn rotate(&mut self) -> &mut Self {
    let theta = self.theta.to_radians() / 2.;
    self.q.w = theta.cos();
    self.q.x = self.axis.x * theta.sin();
    self.q.y = self.axis.y * theta.sin();
    self.q.z = self.axis.z * theta.sin();
    self.q1 = self.q.conjugate();
    self.p = (self.q * self.p) * self.q1;
    self
  }
  pub fn get_point(&mut self, dest: &mut Vector3f) -> &mut Self {
    dest.x = self.p.x * self.mag;
    dest.y = self.p.y * self.mag;
    dest.z = self.p.z * self.mag;
    self
  }
  pub fn get_matrix(&self, dest: &mut Matrix4f) {
    dest.gen_from_quat(self.p);
  }
}
