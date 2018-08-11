


use shader::lighting::Lighting;

pub struct Material {
  pub name: String,
  pub tex_name: String,
  pub row_count: u32,
  pub offset: f32,
  pub lighting: Option<Lighting>,
  pub multi_tex: bool,
}
impl Material {
  pub fn new(name: &str) -> Self {
    Material {
      name: name.to_string(),
      tex_name: String::new(),
      row_count: 0,
      offset: 0.0,
      lighting: None,
      multi_tex: false
    }
  }
}
