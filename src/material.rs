



pub struct Material {
  pub name: String,
  pub texture: String,
  pub lighting: String,
  pub row_count: u32,
  pub offset: f32,
  pub multi_tex: bool,
}
impl Material {
  pub fn new(name: &str, texture: &str, lighting: &str) -> Self {
    Material {
      name: name.to_string(),
      texture: texture.to_string(),
      lighting: lighting.to_string(),
      row_count: 0,
      offset: 0.0,
      multi_tex: false
    }
  }
}
