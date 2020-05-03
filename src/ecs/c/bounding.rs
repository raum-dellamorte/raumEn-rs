#![allow(dead_code,unused_variables)]

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
  pub origin: Vector3f<f32>,
  pub size: Vector3f<f32>,
  pub min: Vector3f<f32>,
  pub max: Vector3f<f32>,
  pub rad: f32,
  pub prox: Vec<Entity>,
  pub overlap: Vec<(Entity, Vector3f<f32>)>,
}
impl Default for Bounds {
  fn default() -> Self {
    Self::new(Vector3f::blank(), Vector3f::new(1.,1.,1.), 1.5)
  }
}
impl Bounds {
  pub fn new(origin: Vector3f<f32>, size: Vector3f<f32>, rad: f32) -> Self {
    let min = origin - size;
    let max = origin + size;
    Self {
      origin, size, min, max, rad, prox: Vec::new(), overlap: Vec::new(), 
    }
  }
  pub fn set_origin(mut self, origin: Vector3f<f32>) -> Self {
    self.origin.copy_from_v3f(origin);
    (&mut self).recalc_min_max();
    self
  }
  pub fn set_size(mut self, size: Vector3f<f32>) -> Self {
    self.size.copy_from_v3f(size);
    (&mut self).recalc_min_max();
    self
  }
  fn recalc_min_max(&mut self) {
    self.min = self.origin - self.size;
    self.max = self.origin + self.size;
  }
  pub fn collide_test(&mut self, ent: Entity, other: &Self, dir: Vector3f<f32>) -> Option<Vector3f<f32>> {
    // let (a, b, c) = self.y_pos();
    // let mut norm = norm_from_points(a, b, c);
    // let (d, e, f) = other.y_neg();
    
    let min_x = self.min_x(other);
    let min_x2 = other.min_x(self);
    let max_x = self.max_x(other);
    let max_x2 = other.min_x(self);
    let min_y = self.min_y(other);
    let min_y2 = other.min_x(self);
    let max_y = self.max_y(other);
    let max_y2 = other.min_x(self);
    let min_z = self.min_z(other);
    let min_z2 = other.min_x(self);
    let max_z = self.max_z(other);
    let max_z2 = other.min_x(self);
    
    if !(&[min_x, max_x, min_y, max_y, min_z, max_z]).iter().any(|t| *t ) { return None }
    
    let x_len = 
      if min_x && max_x { self.max.x - self.min.x }
      else if min_x     { other.max.x - self.min.x } 
      else if max_x     { other.min.x - self.max.x }
      else if min_x2 && max_x2 { other.max.x - other.min.x }
      else { 0.0 };
    let y_len = 
      if min_y && max_y { self.max.y - self.min.y } 
      else if min_y     { other.max.y - self.min.y } 
      else if max_y     { other.min.y - self.max.y }
      else              { other.max.y - other.min.y };
    let z_len = 
      if min_z && max_z { self.max.z - self.min.z } 
      else if min_z     { other.max.z - self.min.z } 
      else if max_z     { other.min.z - self.max.z }
      else              { other.max.z - other.min.z };
    
    
    
    if y_len > x_len.max(z_len) {
      // If the collision area is taller than wide or deep we should be hitting a wall
      if x_len > z_len {
        // If x is longer than z we hit the xy plane
        if min_z {
          self.overlap.push((ent, crate::util::ZVEC));
        } else {
          self.overlap.push((ent, -crate::util::ZVEC));
        }
      } else {
        // Otherwise we hit the yz plane
        
      }
    } else {
      // Either a Floor or Cieling collision
      
    }
    
    None
  }
  fn min_x(&self, other: &Self) -> bool { (other.min.x..other.max.x).contains(&self.min.x) }
  fn max_x(&self, other: &Self) -> bool { (other.min.x..other.max.x).contains(&self.max.x) }
  fn min_y(&self, other: &Self) -> bool { (other.min.y..other.max.y).contains(&self.min.y) }
  fn max_y(&self, other: &Self) -> bool { (other.min.y..other.max.y).contains(&self.max.y) }
  fn min_z(&self, other: &Self) -> bool { (other.min.z..other.max.z).contains(&self.min.z) }
  fn max_z(&self, other: &Self) -> bool { (other.min.z..other.max.z).contains(&self.max.z) }
  fn x1(&self) -> f32 { self.min.x }
  fn y1(&self) -> f32 { self.min.y }
  fn z1(&self) -> f32 { self.min.z }
  fn x2(&self) -> f32 { self.max.x }
  fn y2(&self) -> f32 { self.max.y }
  fn z2(&self) -> f32 { self.max.z }
  fn y_neg(&self) -> (Vector3f<f32>, Vector3f<f32>, Vector3f<f32>) {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_float(self.x1(), self.y1(), self.z2());
    b.copy_from_float(self.x1(), self.y1(), self.z1());
    c.copy_from_float(self.x2(), self.y1(), self.z1());
    (a,b,c)
  }
  fn y_pos(&self) -> (Vector3f<f32>, Vector3f<f32>, Vector3f<f32>) {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_float(self.x1(), self.y2(), self.z2());
    b.copy_from_float(self.x1(), self.y2(), self.z1());
    c.copy_from_float(self.x2(), self.y2(), self.z1());
    (a,b,c)
  }
  fn x_neg(&self) -> (Vector3f<f32>, Vector3f<f32>, Vector3f<f32>) {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_float(self.x1(), self.y1(), self.z2());
    b.copy_from_float(self.x1(), self.y1(), self.z1());
    c.copy_from_float(self.x1(), self.y2(), self.z1());
    (a,b,c)
  }
  fn x_pos(&self) -> (Vector3f<f32>, Vector3f<f32>, Vector3f<f32>) {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_float(self.x2(), self.y1(), self.z2());
    b.copy_from_float(self.x2(), self.y1(), self.z1());
    c.copy_from_float(self.x2(), self.y2(), self.z1());
    (a,b,c)
  }
  fn z_neg(&self) -> (Vector3f<f32>, Vector3f<f32>, Vector3f<f32>) {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_float(self.x1(), self.y2(), self.z1());
    b.copy_from_float(self.x1(), self.y1(), self.z1());
    c.copy_from_float(self.x2(), self.y1(), self.z1());
    (a,b,c)
  }
  fn z_pos(&self) -> (Vector3f<f32>, Vector3f<f32>, Vector3f<f32>) {
    let (mut a, mut b, mut c) = (Vector3f::blank(), Vector3f::blank(), Vector3f::blank());
    a.copy_from_float(self.x1(), self.y2(), self.z2());
    b.copy_from_float(self.x1(), self.y1(), self.z2());
    c.copy_from_float(self.x2(), self.y1(), self.z2());
    (a,b,c)
  }
}

fn norm_from_points(a: Vector3f<f32>, b: Vector3f<f32>, c: Vector3f<f32>) -> Vector3f<f32> {
  let ba = b - a;
  let ca = c - a;
  let mut cross = ba.cross(ca);
  cross.normalize();
  cross
}

fn between(min: f32, max: f32, n: f32) -> bool {
  (min..max).contains(&n)
}

#[cfg(test)]
mod test_bounding {
  use super::*;
  
  #[test]
  fn test_bounds() {
    let mut a = Bounds::default().set_origin(Vector3f::new(-1.,0.,0.)).set_size(Vector3f::new(1.,3.,1.));
    let mut b = Bounds::default().set_origin(Vector3f::new(1.,0.,0.)).set_size(Vector3f::new(1.,3.,1.));
    
    
  }
}