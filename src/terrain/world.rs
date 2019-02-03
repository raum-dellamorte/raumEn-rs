
use terrain::{from_world_to_chunk_space, world_to_local, local_to_world};
use util::{Matrix4f, Vector3f, modulo, HashMap, Arc, Mutex, }; // XVEC, YVEC, ZVEC, 

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
    // let cx = (x / 16.0).floor() as isize;
    // let cz = (z / 16.0).floor() as isize;
    // let loc = ChunkLoc {x: cx, z: cz};
    let (c_loc, l_loc) = world_to_local(x, z);
    if let Some(Some(chunk)) = self.chunks.get(&c_loc) {
      return chunk.bounds_under(self, l_loc, ht );
    } 
    LOWER_BOUNDS
  }
  pub fn bounds_under_v3f(&self, pos: &Vector3f) -> (f32, f32) {
    self.bounds_under(pos.x, pos.z, pos.y)
  }
}

pub struct Chunk {
  pub columns: HashMap<TerrainCoords, ChunkColumn>,
  pub x: isize,
  pub z: isize,
  pub base: f32,
  pub height: f32,
}
impl Chunk {
  pub fn new(x: isize, z: isize, base: f32, height: f32) -> Self {
    let mut columns = HashMap::new();
    for cx in 0..8 {
      for cz in 0..8 {
        columns.insert(ChunkLoc {x: cx, z: cz}, ChunkColumn::new( cx , cz ));
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
  fn bounds_under(&self, world: &World, xz: TerrainCoords, ht: f32) -> (f32, f32) {
    let col = &self.columns.get(&xz).unwrap();
    col.bounds_under(world, xz, ht)
  }
}

pub struct ChunkColumn {
  pub platforms: Vec<Platform>,
  pub x: isize,
  pub z: isize,
}
impl ChunkColumn {
  pub fn new(x: isize, z: isize) -> Self {
    ChunkColumn {
      platforms: Vec::new(),
      x: x,
      z: z,
    }
  }
  pub fn loc(&self) -> TerrainCoords {
    ChunkLoc {x: self.x, z: self.z}
  }
  fn bounds_under(&self, world: &World, xz: TerrainCoords, _ht: f32) -> (f32, f32)  {
    let mut out = LOWER_BOUNDS;
    let t_loc = self.loc();
    if t_loc != xz { panic!("got plat loc {:?} for xz {:?}", t_loc, xz ) }
    for plat in &self.platforms {
      let (_chunk, local) = world_to_local(plat.x as f32, plat.z as f32);
      if local != xz { panic!("got plat loc {:?} for xz {:?}", local, xz ) }
      out = plat.world_upper_lower(world);
      break
      // let (u, l) = plat.world_upper_lower(world);
      // if ht >= l {
      //   if l > out.1 {
      //     out = (u, l);
      //   }
      // }
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
  pub x: isize,
  pub z: isize,
  pub top: f32,
  pub depth: f32,
  pub color: Vector3f,
  pub trans_mat: Matrix4f,
}
impl Platform {
  pub fn new(x: isize, z: isize, top: f32, depth: f32) -> Self {
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
  pub fn x(&self) -> f32 { self.x as f32 }
  pub fn z(&self) -> f32 { self.z as f32 }
  pub fn loc(&self) -> TerrainCoords {
    ChunkLoc {x: self.x, z: self.z}
  }
  pub fn world_upper_lower(&self, world: &World) -> (f32, f32) {
    let y = (world.height * self.top) + world.base;
    // println!("World height @{},{}: {}", self.x, self.z, y);
    (y, y - (world.height * self.depth))
  }
  pub fn transformation(&self, world: &World, trans_mat: &mut Matrix4f) {
    //           200            0.5              200           0.05         -100
    //                  100                              10                 -100
    //                               90                                     -100
    let y = ((world.height * self.top) - (world.height * self.depth)) + world.base;
    // y is -10
    let pos = Vector3f::new(self.x(), y, self.z()); // so we draw from -10 up
    let scale = Vector3f::new(1.0, world.height * self.depth, 1.0); // cube dims are 2,1,2
    trans_mat.set_identity();
    trans_mat.translate_v3f(&pos);
    // trans_mat.rotate(0_f32.to_radians(), &XVEC);
    // trans_mat.rotate(0_f32.to_radians(), &YVEC);
    // trans_mat.rotate(0_f32.to_radians(), &ZVEC);
    trans_mat.scale(&scale);
  }
}

