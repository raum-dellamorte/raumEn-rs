
// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use util::rmatrix::Matrix4f;
use util::rvector::{Vector3f, }; // XVEC, YVEC, ZVEC, 

const LOWER_BOUNDS: (f32, f32) = (-1000_f32, -1100_f32);

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
  pub fn get_chunk(&self, x: isize, z: isize) -> Option<Arc<Mutex<Chunk>>> {
    for chunk_arc in &self.chunks {
      let chunk = chunk_arc.lock().unwrap();
      if chunk.x == x && chunk.z == z { return Some(chunk_arc.clone()) }
    }
    None
  }
  pub fn bounds_under(&self, x: f32, z: f32, ht: f32) -> (f32, f32) {
    let cx = ((x.abs() / 16.0).floor() as isize) * if x >= 0.0 { 1 } else { -1 };
    let cz = ((z.abs() / 16.0).floor() as isize) * if z >= 0.0 { 1 } else { -1 };
    println!("cx {} cz{}", cx, cz);
    if let Some(chunk_arc) = self.get_chunk(cx, cz) {
      let chunk = chunk_arc.lock().unwrap();
      return chunk.bounds_under(x, z, ht);
    } 
    LOWER_BOUNDS
  }
  pub fn bounds_under_v3f(&self, pos: &Vector3f) -> (f32, f32) {
    self.bounds_under(pos.x, pos.z, pos.y)
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
        columns.push(ChunkColumn::new(base, height, x, z, r, c));
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
  fn bounds_under(&self, x: f32, z: f32, ht: f32) -> (f32, f32) {
    let cx = ((x.abs() / 2.0) % 8.0).floor() as usize;
    let cz = ((z.abs() / 2.0) % 8.0).floor() as usize;
    let c = if x >= 0.0 { cx } else { 7 - cx };
    let r = if z >= 0.0 { cz } else { 7 - cz };
    let i = (r * 8) + c;
    let col = &self.columns[i];
    col.bounds_under(ht)
  }
}

pub struct ChunkColumn {
  pub platforms: Vec<Platform>,
  pub x: u8,
  pub z: u8,
}
impl ChunkColumn {
  pub fn new(base: f32, height: f32, cx: isize, cz: isize, x: u8, z: u8) -> Self {
    let mut platforms = Vec::new();
    platforms.push(Platform::new(base, height, 0.5, 0.05, cx, cz, x, z));
    ChunkColumn {
      platforms: platforms,
      x: x,
      z: z,
    }
  }
  fn bounds_under(&self, ht: f32) -> (f32, f32)  {
    let mut out = LOWER_BOUNDS;
    for plat in &self.platforms {
      let (u, l) = plat.world_upper_lower();
      if ht >= l {
        let (_, tl) = out;
        if l > tl {
          out = (u, l);
        }
      }
    }
    out
  }
}

pub struct Platform {
  pub material: String,
  pub world_base: f32,
  pub world_height: f32,
  pub top: f32,
  pub depth: f32,
  pub x: f32,
  pub z: f32,
  pub color: Vector3f,
  pub trans_mat: Matrix4f,
}
impl Platform {
  pub fn new(base: f32, height: f32, top: f32, depth: f32, cx: isize, cz: isize, lx: u8, lz: u8) -> Self {
    let x = ((cx * 16) + ((lx * 2) as isize)) as f32;
    let z = ((cz * 16) + ((lz * 2) as isize)) as f32;
    let top = gen_height(x, z, top, 3);
    Platform {
      material: "dirt".to_string(),
      world_base: base,
      world_height: height,
      top: top,
      depth: depth,
      x: x,
      z: z,
      color: Vector3f::new(0.5, 0.5, 0.5),
      trans_mat: Matrix4f::new(),
    }
  }
  pub fn world_upper_lower(&self) -> (f32, f32) {
    let y = (self.world_height * self.top) + self.world_base;
    // println!("World height @{},{}: {}", self.x, self.z, y);
    (y, y - (self.world_height * self.depth))
  }
  pub fn transformation(&self, trans_mat: &mut Matrix4f) {
    let y = ((self.world_height * self.top) - (self.world_height * self.depth)) + self.world_base;
    let pos = Vector3f::new(self.x, y, self.z);
    let scale = Vector3f::new(1.0, self.world_height * self.depth, 1.0);
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
  let x = x as f64 / 256.0_f64;
  let z = z as f64 / 256.0_f64;
  let pt: Point2<f64> = [x, z];
  let noisefn = Fbm::new().set_seed(186074_u32);
  // let noise: Clamp<[f64; 2]> = Clamp::new(&noisefn).set_bounds(0.0_f64, 1.0_f64);
  let mut out: f64 = *(&noisefn.get(pt));
  out = (out + 1.0) / 2.0;
  out = (out + (weight * mult as f32) as f64) / (mult + 1) as f64;
  print!("h<{}>", out);
  out as f32
}
