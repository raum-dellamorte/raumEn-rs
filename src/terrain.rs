


pub struct World{
  pub chunks: Vec<Chunk>,
  pub nearby: Vec<Chunk>,
}

pub struct Chunk{
  pub columns: [[ChunkColumn; 8]; 8],
}

pub struct ChunkColumn{
  pub platforms: Vec<Platform>,
}

pub struct Platform{
  pub top: f32,
  pub depth: f32,
}
