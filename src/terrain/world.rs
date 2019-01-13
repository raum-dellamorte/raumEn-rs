
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use terrain::from_world_to_chunk_space;
use util::rmatrix::Matrix4f;
use util::rvector::{Vector3f, }; // XVEC, YVEC, ZVEC, 

const LOWER_BOUNDS: (f32, f32) = (-1000_f32, -1100_f32);

use terrain::TerrainCoords;
use terrain::TerrainCoords::ChunkLoc;

pub struct World {
  pub chunks: HashMap<TerrainCoords, Option<Box<Chunk>>>,
      base: f32,
      height: f32,
  pub model: String,
}
impl World {
  pub fn new() -> Self {
    World {
      chunks: HashMap::new(),
      base: -100_f32,
      height: 200_f32,
      model: "platform".to_string(),
    }
  }
  pub fn add_chunk(&mut self, chunk: Box<Chunk>, x_pos: isize, z_pos: isize) -> TerrainCoords {
    let loc = ChunkLoc {x: x_pos, z: z_pos};
    if self.chunks.get(&loc).is_none() {
      self.chunks.insert(loc, Some(chunk));
    }
    ChunkLoc {x: x_pos, z: z_pos}
  }
  pub fn take_nearby(&mut self, x_pos: f32, z_pos:f32) -> Vec<Box<Chunk>> {
    let mut out: Vec<Box<Chunk>> = Vec::new();
    let (x, z) = from_world_to_chunk_space(x_pos, z_pos);
    for xc in 0..11 {
      for zc in 0..11 {
        let cl = ChunkLoc {x: (x + xc - 5), z: (z + zc - 5)};
        if let Some(ref mut _chunk) = self.chunks.get_mut(&cl) {
          if let Some(chunk) = _chunk.take() {
            out.push(chunk);
          }
        }
      }
    }
    out
  }
  pub fn take_chunk(&mut self, x: isize, z: isize) -> Option<Box<Chunk>> {
    let loc = ChunkLoc {x: x, z: z};
    if let Some(ref mut _chunk) = self.chunks.get_mut(&loc) {
      let mut chunk = _chunk.take();
      let out = chunk.take();
      return out
    }
    return None
  }
  pub fn return_chunk(&mut self, chunk: Box<Chunk>) {
    let loc = chunk.loc();
    self.chunks.insert(loc, Some(chunk));
  }
  pub fn return_chunks(&mut self, chunks: Vec<Box<Chunk>>) {
    let mut chunks = chunks;
    while chunks.len() > 0 {
      let chunk = chunks.pop().unwrap();
      self.return_chunk(chunk);
    }
  }
  pub fn bounds_under(&self, x: f32, z: f32, ht: f32) -> (f32, f32) {
    let cx = (x / 16.0).floor() as isize;
    let cz = (z / 16.0).floor() as isize;
    let loc = ChunkLoc {x: cx, z: cz};
    if let Some(Some(chunk)) = self.chunks.get(&loc) {
      return chunk.bounds_under(self, x, z, ht);
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
  pub fn loc(&self) -> TerrainCoords {
    ChunkLoc {x: self.x, z: self.z}
  }
  fn bounds_under(&self, world: &World, x: f32, z: f32, ht: f32) -> (f32, f32) {
    let cx = ((x.abs() / 2.0) % 8.0).floor() as usize;
    let cz = ((z.abs() / 2.0) % 8.0).floor() as usize;
    let c = if x >= 0.0 { cx } else { 7 - cx };
    let r = if z >= 0.0 { cz } else { 7 - cz };
    let i = (r * 8) + c;
    let col = &self.columns[i];
    col.bounds_under(world, ht)
  }
}

pub struct ChunkColumn {
  pub platforms: Vec<Platform>,
  pub x: u8,
  pub z: u8,
}
impl ChunkColumn {
  pub fn new(x: u8, z: u8) -> Self {
    ChunkColumn {
      platforms: Vec::new(),
      x: x,
      z: z,
    }
  }
  fn bounds_under(&self, world: &World, ht: f32) -> (f32, f32)  {
    let mut out = LOWER_BOUNDS;
    for plat in &self.platforms {
      let (u, l) = plat.world_upper_lower(world);
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

pub enum TerrainUnit {
  Grass,
  Water,
  Hole,
}

pub struct Platform {  // Platform needs to be enum
  pub material: String,
  pub x: f32,
  pub z: f32,
  pub top: f32,
  pub depth: f32,
  pub color: Vector3f,
  pub trans_mat: Matrix4f,
}
impl Platform {
  pub fn new(x: f32, z: f32, top: f32, depth: f32) -> Self {
    Platform {
      material: "dirt".to_string(),
      x: x,
      z: z,
      top: top,
      depth: depth,
      color: Vector3f::new(0.5, 0.5, 0.5),
      trans_mat: Matrix4f::new(),
    }
  }
  pub fn world_upper_lower(&self, world: &World) -> (f32, f32) {
    let y = (world.height * self.top) + world.base;
    // println!("World height @{},{}: {}", self.x, self.z, y);
    (y, y - (world.height * self.depth))
  }
  pub fn transformation(&self, world: &World, trans_mat: &mut Matrix4f) {
    let y = ((world.height * self.top) - (world.height * self.depth)) + world.base;
    let pos = Vector3f::new(self.x, y, self.z);
    let scale = Vector3f::new(1.0, world.height * self.depth, 1.0);
    trans_mat.set_identity();
    trans_mat.translate_v3f(&pos);
    // trans_mat.rotate(0_f32.to_radians(), &XVEC);
    // trans_mat.rotate(0_f32.to_radians(), &YVEC);
    // trans_mat.rotate(0_f32.to_radians(), &ZVEC);
    trans_mat.scale(&scale);
  }
}
