



pub struct Texture {
  pub tex_name: String,
  pub tex_id: u32,
  pub tex_unit: i32,
}
impl Texture {
  pub fn new(name: &str, tex_id: u32) -> Self {
    Texture {
      tex_name: name.to_string(),
      tex_id: tex_id,
      tex_unit: -1,
    }
  }
  pub fn assign_tex_unit(self, unit: i32) -> Self {
    let mut _self = self;
    _self.tex_unit = unit;
    _self
  }
}
