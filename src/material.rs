


use shader::lighting::Lighting;

pub struct Material {
  pub name: String,
  pub tex_name: String,
  pub tex_id: u32,
  pub tex_r: u32,
  pub tex_b: u32,
  pub tex_g: u32,
  pub row_count: u32,
  pub offset: f32,
  pub lighting: Option<Lighting>,
  pub texture: GLuint,
  pub use_tex: bool,
  
}
