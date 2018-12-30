
use terrain::{Chunk, Platform, World, from_world_to_chunk_space};

use terrain::TerrainCoords;
use terrain::TerrainCoords::ChunkLoc;

// use noise::Clamp;
use noise::NoiseFn;
use noise::Fbm;
use noise::Point2;
use noise::Seedable;

pub struct WorldBuilder {
  pub landscape: noise::Fbm,
  pub l_weight: f32,
  pub l_mult: i32,
  pub holes: noise::Fbm,
  
}
impl WorldBuilder {
  pub fn new() -> Self {
    Self {
      landscape: Fbm::new().set_seed(186074_u32),
      l_weight: 0_f32,
      l_mult: 0_i32,
      holes: Fbm::new().set_seed(441952_u32),
    }
  }
  pub fn set_landscape_weight_and_mult(&mut self, weight: f32, mult: i32) {
    self.l_weight = weight;
    self.l_mult = mult;
  }
  pub fn gen_world(&mut self, world: &mut World, x_pos: f32, z_pos: f32) {
    let (x_chunk, z_chunk) = from_world_to_chunk_space(x_pos, z_pos);
    for x_pos in 0..11 {
      for z_pos in 0..11 {
        let (x, z) = (x_chunk + x_pos - 5, z_chunk + z_pos - 5);
        if world.chunks.get(&ChunkLoc {x, z}).is_some() { continue }
        println!("Creating Chunk at {:?} from {:?}", ChunkLoc {x,z}, ChunkLoc {x: x_chunk, z: z_chunk} );
        let chunk = Box::new(Chunk::new(x, z, -100_f32, 200_f32));
        let loc = world.add_chunk(chunk, x, z);
        if let Some(Some(ref mut chunk)) = world.chunks.get_mut(&loc) {
          for column in &mut chunk.columns {
            let px = ((x * 16) + ((column.x * 2) as isize)) as f32;
            let pz = ((z * 16) + ((column.z * 2) as isize)) as f32;
            let hpt = point(px, pz, 6);
            let hole: bool = self.holes.get(hpt) < -0.2_f64;
            if !hole {
              let pt = point(px, pz, 7);
              let mut top: f64 = self.landscape.get(pt);
              top = (top + 1.0) / 2.0;
              top = (top + (self.l_weight * self.l_mult as f32) as f64) / (self.l_mult + 1) as f64;
              // print!("h<{}>", top);
              let pform = Platform::new(px, pz, top as f32, 0.05);
              column.platforms.push(pform);
            }
          }
        }
      }
    }
  }
}

fn point(x: f32, z: f32, precision: u32) -> Point2<f64> {
  let p = 2_isize.pow(precision) as f64;
  let x = x as f64 / p;
  let z = z as f64 / p;
  let pt: Point2<f64> = [x, z];
  pt
}
