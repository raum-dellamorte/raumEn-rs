
// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use util::rmatrix::Matrix4f;
use util::rvector::{Vector3f, }; // XVEC, YVEC, ZVEC, 

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
        columns.push(ChunkColumn::new(x, z, r, c));
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
  pub fn new(cx: isize, cz: isize, x: u8, z: u8) -> Self {
    let mut platforms = Vec::new();
    platforms.push(Platform::new(0.5, 0.05, cx, cz, x, z));
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
  pub x: f32,
  pub z: f32,
  pub color: Vector3f,
  pub trans_mat: Matrix4f,
}
impl Platform {
  pub fn new(top: f32, depth: f32, cx: isize, cz: isize, lx: u8, lz: u8) -> Self {
    let x = ((cx * 16) + ((lx * 2) as isize)) as f32;
    let z = ((cz * 16) + ((lz * 2) as isize)) as f32;
    let top = gen_height(x, z, top, 7);
    Platform {
      material: "dirt".to_string(),
      top: top,
      depth: depth,
      x: x,
      z: z,
      color: Vector3f::new(0.5, 0.5, 0.5),
      trans_mat: Matrix4f::new(),
    }
  }
  pub fn transformation(&self, trans_mat: &mut Matrix4f, base: f32, height: f32) {
    let y = ((height * self.top) - (height * self.depth)) + base;
    let pos = Vector3f::new(self.x, y, self.z);
    let scale = Vector3f::new(1.0, height * self.depth, 1.0);
    trans_mat.set_identity();
    trans_mat.translate_v3f(&pos);
    // trans_mat.rotate(0_f32.to_radians(), &XVEC);
    // trans_mat.rotate(0_f32.to_radians(), &YVEC);
    // trans_mat.rotate(0_f32.to_radians(), &ZVEC);
    trans_mat.scale(&scale);
  }
}


fn gen_height(x: f32, z: f32, weight: f32, mult: i32) -> f32 {
  // use noise::Clamp;
  use noise::NoiseFn;
  use noise::Fbm;
  use noise::Point2;
  use noise::Seedable;
  let x = x as f64 / 2.0_f64;
  let z = z as f64 / 2.0_f64;
  let pt: Point2<f64> = [x, z];
  let noisefn = Fbm::new().set_seed(186074_u32);
  // let noise: Clamp<[f64; 2]> = Clamp::new(&noisefn).set_bounds(0.0_f64, 1.0_f64);
  let mut out: f64 = *(&noisefn.get(pt));
  out = (out + 1.0) / 2.0;
  out = (out + (weight * mult as f32) as f64) / (mult + 1) as f64;
  print!("h<{}>", out);
  out as f32
}
