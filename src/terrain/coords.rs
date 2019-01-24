

use std::hash::Hash;

#[derive(Debug)]
pub enum TerrainCoords {
  ChunkLoc { x: isize, z: isize }
}
impl std::cmp::PartialEq for TerrainCoords {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (TerrainCoords::ChunkLoc { x: x1, z: z1 }, TerrainCoords::ChunkLoc { x: x2, z: z2 }) if x1 == x2 && z1 == z2 => { true }
      _ => { false }
    }
  }
}
impl std::cmp::Eq for TerrainCoords {}
impl std::hash::Hash for TerrainCoords {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      TerrainCoords::ChunkLoc { x, z } => {
        "ChunkLoc::".hash(state);
        "x:".hash(state);
        x.hash(state);
        "z:".hash(state);
        z.hash(state);
      }
      // _ => {
      //   "None".hash(state);
      // }
    }
  }
}
