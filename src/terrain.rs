
// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use util::rmatrix::Matrix4f;
use util::rvector::{Vector3f, XVEC, YVEC, ZVEC};

pub struct World {
  pub chunks: Vec<Arc<Mutex<Chunk>>>,
  pub model: String,
}
impl World {
  pub fn new() -> Self {
    World {
      chunks: Vec::new(),
      model: "platform".to_string(),
    }
  }
  pub fn new_chunk(&mut self, x_pos: isize, z_pos:isize) {
    let chunk = Arc::new(Mutex::new(Chunk::new(x_pos, z_pos, -100.0, 200.0)));
    self.chunks.push(chunk);
  }
  pub fn nearby(&mut self) -> Vec<Arc<Mutex<Chunk>>> {
    let mut out = Vec::new();
    for chunk in &self.chunks {
      out.push(chunk.clone());
    }
    out
  }
}

pub struct Chunk {
  pub columns: Vec<ChunkColumn>,
  pub x: isize,
  pub z: isize,
  pub base: f32,
  pub height: f32,
}
impl Chunk {
  pub fn new(x: isize, z: isize, base: f32, height: f32) -> Self {
    let mut columns = Vec::new();
    for r in 0..8 {
      for c in 0..8 {
        columns.push(ChunkColumn::new(r, c));
      }
    }
    Chunk {
      columns: columns,
      x: x,
      z: z,
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
  pub material: String,
  pub top: f32,
  pub depth: f32,
  pub color: Vector3f,
  pub trans_mat: Matrix4f,
}
impl Platform {
  pub fn new(top: f32, depth: f32) -> Self {
    Platform {
      material: "dirt".to_string(),
      top: top,
      depth: depth,
      color: Vector3f::new(0.5, 0.5, 0.5),
      trans_mat: Matrix4f::new(),
    }
  }
  pub fn transformation(&self, trans_mat: &mut Matrix4f,
      base: f32, height: f32, cx: isize, cz: isize, lx: u8, lz: u8, ) {
    let x = ((cx * 16) + ((lx * 2) as isize)) as f32;
    let z = ((cz * 16) + ((lz * 2) as isize)) as f32;
    let y = ((height * self.top) - (height * self.depth)) + base;
    let pos = Vector3f::new(x, y, z);
    let scale = Vector3f::new(1.0, height * self.depth, 1.0);
    trans_mat.set_identity();
    trans_mat.translate_v3f(&pos);
    trans_mat.rotate(0_f32.to_radians(), &XVEC);
    trans_mat.rotate(0_f32.to_radians(), &YVEC);
    trans_mat.rotate(0_f32.to_radians(), &ZVEC);
    trans_mat.scale(&scale);
  }
}
