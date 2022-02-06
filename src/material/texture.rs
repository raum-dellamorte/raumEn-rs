
use {
  GameMgr,
  util::{
    HashMap, 
    rgl::*, 
  },
};

#[derive(Default)]
pub struct Textures(pub HashMap<String,Texture>);
impl Textures {
  pub fn new_texture(&mut self, mgr: &GameMgr, name: &str) {
    let texture =  mgr.loader.borrow_mut().load_texture(name);
    // println!("texture: image<{}> tex_id<{}>", name, texture.tex_id);
    self.0.insert(name.to_string(), texture);
  }
}

pub struct Texture {
  pub tex_name: String,
  pub tex_id: TextureID,
  pub tex_unit: TextureUnit,
}
impl Texture {
  pub fn new(name: &str, tex_id: u32) -> Self {
    Texture {
      tex_name: name.to_string(),
      tex_id: TextureID(tex_id),
      tex_unit: TextureUnit(-1),
    }
  }
  pub fn assign_tex_unit(self, unit: i32) -> Self {
    let mut _self = self;
    _self.tex_unit = TextureUnit(unit);
    _self
  }
}
