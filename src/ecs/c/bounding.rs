
use {
  specs::{
    Component, VecStorage, Entity,
  },
  util::{
    Vector3f, RVec,
  },
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Bounds {
  pub origin: Vector3f,
  pub size: Vector3f,
  pub min: Vector3f,
  pub max: Vector3f,
  pub rad: f32,
  pub prox: Vec<Entity>,
  pub overlap: Vec<Entity>,
}
impl Default for Bounds {
  fn default() -> Self {
    Self::new(Vector3f::blank(), Vector3f::new(1.,1.,1.), 1.5)
  }
}
impl Bounds {
  pub fn new(origin: Vector3f, size: Vector3f, rad: f32) -> Self {
    let min = origin - size;
    let max = origin + size;
    Self {
      origin, size, min, max, rad, prox: Vec::new(), overlap: Vec::new(), 
    }
  }
  pub fn set_origin(mut self, origin: Vector3f) -> Self {
    self.origin.copy_from_v3f(origin);
    self
  }
  pub fn collide_test(&self, other: &Self, dir: Vector3f) -> Option<Vector3f> {
    let mut norm = self.y_pos();
    
    None
  }
  fn x1(&self) -> f32 { self.min.x }
  fn y1(&self) -> f32 { self.min.y }
  fn z1(&self) -> f32 { self.min.z }
  fn x2(&self) -> f32 { self.max.x }
  fn y2(&self) -> f32 { self.max.y }
  fn z2(&self) -> f32 { self.max.z }
  fn y_neg(&self) -> Vector3f {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_f32(self.x1(), self.y1(), self.z2());
    b.copy_from_f32(self.x1(), self.y1(), self.z1());
    c.copy_from_f32(self.x2(), self.y1(), self.z1());
    norm_from_points(a,b,c)
  }
  fn y_pos(&self) -> Vector3f {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_f32(self.x1(), self.y2(), self.z2());
    b.copy_from_f32(self.x1(), self.y2(), self.z1());
    c.copy_from_f32(self.x2(), self.y2(), self.z1());
    norm_from_points(a,b,c)
  }
  fn x_neg(&self) -> Vector3f {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_f32(self.x1(), self.y1(), self.z2());
    b.copy_from_f32(self.x1(), self.y1(), self.z1());
    c.copy_from_f32(self.x1(), self.y2(), self.z1());
    norm_from_points(a,b,c)
  }
  fn x_pos(&self) -> Vector3f {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_f32(self.x2(), self.y1(), self.z2());
    b.copy_from_f32(self.x2(), self.y1(), self.z1());
    c.copy_from_f32(self.x2(), self.y2(), self.z1());
    norm_from_points(a,b,c)
  }
  fn z_neg(&self) -> Vector3f {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_f32(self.x1(), self.y2(), self.z1());
    b.copy_from_f32(self.x1(), self.y1(), self.z1());
    c.copy_from_f32(self.x2(), self.y1(), self.z1());
    norm_from_points(a,b,c)
  }
  fn z_pos(&self) -> Vector3f {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_f32(self.x1(), self.y2(), self.z2());
    b.copy_from_f32(self.x1(), self.y1(), self.z2());
    c.copy_from_f32(self.x2(), self.y1(), self.z2());
    norm_from_points(a,b,c)
  }
}

fn norm_from_points(a: Vector3f, b: Vector3f, c: Vector3f) -> Vector3f {
  let ba = b - a;
  let ca = c - a;
  let mut cross = ba.cross(ca);
  cross.normalize();
  cross
}
