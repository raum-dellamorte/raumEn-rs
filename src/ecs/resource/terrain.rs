
use {
  util::{
    HashSet,
  },
};

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
