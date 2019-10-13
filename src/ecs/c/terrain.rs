
use {
  specs::{
    Component, VecStorage,
  },
  util::{
    Vector3f,
    HashSet,
  },
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Platform {
  pub x: i32,
  pub z: i32,
  pub h: f32,
  pub d: f32,
  pub pos: Vector3f<f32>,
  pub scale: Vector3f<f32>,
}
impl Platform {
  pub fn new(world_height: f32, base: f32, x: i32, z: i32, top: f32, depth: f32) -> Self {
    let y = ((world_height * top) - (world_height * depth)) + base;
    let ys = world_height * depth;
    Self {
      x, z, h: top, d: depth,
      pos: Vector3f::new(x as f32, y, z as f32),
      scale: Vector3f::new(1.0, ys, 1.0),
    }
  }
}

pub struct TerrainNode(pub i32, pub i32);
impl std::cmp::PartialEq for TerrainNode {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (TerrainNode(x1, z1), TerrainNode(x2, z2)) if x1 == x2 && z1 == z2 => { true }
      _ => { false }
    }
  }
}
impl std::cmp::Eq for TerrainNode {}
impl std::hash::Hash for TerrainNode {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let TerrainNode(x, z) = self;
    format!("TerrainNode::{:09}:{:09}", x, z).hash(state);
  }
}

pub struct TerrainNodes(pub HashSet<TerrainNode>);
impl Default for TerrainNodes {
  fn default() -> Self { Self(HashSet::new()) }
}
