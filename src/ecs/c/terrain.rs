
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
}
impl Platform {
  pub fn pos(&self, world_height: f32, base: f32) -> Vector3f {
    let y = ((world_height * self.h) - base) - (world_height * self.d * 0.5); // 
    Vector3f::new(self.x as f32, y, self.z as f32)
  }
  pub fn scale(&self, wh: f32) -> Vector3f {
    Vector3f::new(1.0, wh * self.d, 1.0)
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
