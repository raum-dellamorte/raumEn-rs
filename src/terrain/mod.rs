
pub mod builder;
pub mod coords;
pub mod world;

pub use terrain::builder::*;
pub use terrain::coords::*;
pub use terrain::world::*;

pub fn from_world_to_chunk_space(x: f32, z: f32) -> (isize, isize) {
  ((x / 16.0).floor() as isize, (z / 16.0).floor() as isize)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_from_world_to_chunk_space() {
    use terrain::from_world_to_chunk_space;
    {
      let (x, z) = from_world_to_chunk_space(8.0, 8.0);
      assert_eq!(x, 0);
      assert_eq!(z, 0);
    }
    {
      let (x, z) = from_world_to_chunk_space(-8.0, -8.0);
      assert_eq!(x, -1);
      assert_eq!(z, -1);
    }
    {
      let (x, z) = from_world_to_chunk_space(-24.0, 24.0);
      assert_eq!(x, -2);
      assert_eq!(z, 1);
    }
    {
      let (x, z) = from_world_to_chunk_space(40.0, -40.0);
      assert_eq!(x, 2);
      assert_eq!(z, -3);
    }
    {
      let (x, z) = from_world_to_chunk_space(((12*16)+8) as f32, ((-15*16)+8) as f32);
      assert_eq!(x, 12);
      assert_eq!(z, -15);
    }
  }

  #[test]
  fn test_chunk_loc() -> Result<(), String> {
    use terrain::coords::TerrainCoords;
    use terrain::coords::TerrainCoords::ChunkLoc;
    use terrain::from_world_to_chunk_space;
    use std::collections::HashMap;

    {
      let a = ChunkLoc {x: 0, z: 0};
      let b = ChunkLoc {x: 0, z: 0};
      if a != b { return Err(String::from("ChunkLoc {x: 0, z: 0} should equal ChunkLoc {x: 0, z: 0}")) }
    }
    {
      let a = ChunkLoc {x: -10, z: 32};
      let b = ChunkLoc {x: -10, z: 32};
      if a != b { return Err(String::from("ChunkLoc {x: -10, z: 32} should equal ChunkLoc {x: -10, z: 32}")) }
    }
    {
      let a = ChunkLoc {x: -10, z: 32};
      let b = ChunkLoc {x: 32, z: -10};
      if a == b { return Err(String::from("ChunkLoc {x: -10, z: 32} should not equal ChunkLoc {x: 10, z: -32}")) }
    }
    {
      let mut m: HashMap<TerrainCoords, String> = HashMap::new();
      let (x_chunk, z_chunk) = from_world_to_chunk_space(((12*16)+8) as f32, ((-15*16)+8) as f32);
      for x_pos in 0..11 {
        for z_pos in 0..11 {
          let (x, z) = (x_chunk + x_pos - 5, z_chunk + z_pos - 5);
          let s: String = format!("{}, {}", x, z);
          m.insert(ChunkLoc {x: x, z: z}, s);
        }
      }
      if !(m.get(&ChunkLoc {x: 12, z: -15}).unwrap() == "12, -15") { 
        return Err(String::from("HashMap at ChunkLoc {x: 12, z: -15} should be String '12, -15'")) }
      if !(m.get(&ChunkLoc {x: 7, z: -10}).unwrap() == "7, -10") { 
        return Err(String::from("HashMap at ChunkLoc {x: 7, z: -10} should be String '7, -10'")) }
      if !(m.get(&ChunkLoc {x: 6, z: -10}).is_none()) { 
        return Err(String::from("HashMap at ChunkLoc {x: 6, z: -10} should be None")) }
    }
    Ok(())
  }
  
//   #[test]
//   fn fail() {
//         panic!("Make this test fail");
//   }
}
