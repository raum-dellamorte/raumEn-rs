



pub struct Texture {
  pub tex_name: String,
  pub tex_id: u32,
  pub tex_r: u32,
  pub tex_g: u32,
  pub tex_b: u32,
}
impl Texture {
  pub fn new(name: &str, tex_id: u32) -> Self {
    Texture {
      tex_name: name.to_string(),
      tex_id: tex_id,
      tex_r: 0,
      tex_g: 0,
      tex_b: 0,
    }
  }
}
