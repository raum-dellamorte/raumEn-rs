
use std::collections::HashMap;

use util::rvector::Vector3f;

pub struct World {
  pub chunks: HashMap<(isize, isize), Chunk>,
  pub nearby: Vec<Chunk>,
}
impl World {
  pub fn new() -> Self {
    World {
      chunks: HashMap::new(),
      nearby: Vec::new(),
    }
  }
  pub fn new_chunk(&mut self, x_pos: isize, z_pos:isize) {
    let chunk = Chunk::new(-100.0, 200.0);
    self.chunks.insert((x_pos, z_pos), chunk);
  }
}

pub struct Chunk {
  pub columns: Vec<Vec<ChunkColumn>>,
  pub base: f32,
  pub height: f32,
}
impl Chunk {
  pub fn new(base: f32, height: f32) -> Self {
    let mut columns = Vec::new();
    for r in 0..8 {
      let mut col = Vec::new();
      for c in 0..8 {
        col.push(ChunkColumn::new(r, c));
      }
      columns.push(col);
    }
    Chunk {
      columns: columns,
      base: base,
      height: height,
    }
  }
}

pub struct ChunkColumn {
  pub platforms: Vec<Platform>,
  pub x: u8,
  pub z: u8,
}
impl ChunkColumn {
  pub fn new(x: u8, z: u8) -> Self {
    let mut platforms = Vec::new();
    platforms.push(Platform::new(0.5, 0.05));
    ChunkColumn {
      platforms: platforms,
      x: x,
      z: z,
    }
  }
}

pub struct Platform {
  pub top: f32,
  pub depth: f32,
  pub color: Vector3f,
}
impl Platform {
  pub fn new(top: f32, depth: f32) -> Self {
    Platform {
      top: top,
      depth: depth,
      color: Vector3f::new(0.5, 0.5, 0.5),
    }
  }
}
