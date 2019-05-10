
use {
  specs::{
    Component, VecStorage, 
  },
  util::{
    Vector3f, // Vector4f,
  },
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
  pub pos: Vector3f,
  pub rot: Vector3f,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3f);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct DeltaVelocity(pub Vector3f);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TmpVelocity(pub Vector3f);

impl TmpVelocity {
  pub fn clear(&mut self) {
    self.0.clear();
  }
}

#[derive(Debug)]
pub struct PlayerLoc(pub i32,pub i32);
impl Default for PlayerLoc { fn default() -> Self { Self(0,0) } }
