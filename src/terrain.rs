
// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use model::loader::Loader;
use model::Model;
use util::rmatrix::Matrix4f;
use util::rvector::Vector3f;

pub struct World {
  pub chunks: Vec<Arc<Mutex<Chunk>>>,
  pub model: Model,
}
impl World {
  pub fn new(loader_arc: Arc<Mutex<Loader>>) -> Self {
    let mut loader = loader_arc.lock().unwrap();
    let mut  model = Model::new("platform");
    model.init_with_texture(&mut loader, "dirt"); // todo: texture should be per platform
    World {
      chunks: Vec::new(),
      model: model,
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
  pub fn transformation(&self, 
      base: f32, height: f32, cx: isize, cz: isize, lx: u8, lz: u8, ) -> [f32; 16] {
    let mut trans_mat = Matrix4f::new();
    let x = ((cx * 8) + lx as isize) as f32 - 0.5;
    let z = ((cz * 8) + lz as isize) as f32 - 0.5;
    let y = (height * self.top) - ((height * self.depth) / 2.0) + base;
    let pos = Vector3f::new(x, y, z);
    trans_mat.set_identity();
    trans_mat.translate_v3f(&pos);
    // trans_mat.rotate(self.rx.to_radians(), &XVEC);
    // trans_mat.rotate(self.ry.to_radians(), &YVEC);
    // trans_mat.rotate(self.rz.to_radians(), &ZVEC);
    // trans_mat.scale(&Vector3f::new(self.scale, self.scale, self.scale));
    trans_mat.as_slice()
  }
}
