
use {
  specs::{
    Component, VecStorage,
  },
  util::{
    Vector3f,
  },
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Bounds {
  pub min: Vector3f,
  pub max: Vector3f,
  // pub rot: Vector3f,
}
impl Default for Bounds {
  fn default() -> Self {
    Self {
      min: Vector3f::blank(),
      max: Vector3f::blank(),
      // rot: Vector3f::blank(),
    }
  }
}
