

use {
  // GameMgr,
  Loader,
  util::{
    rgl::*,
    HashMap, 
  },
};

#[derive(Default)]
pub struct Models(pub HashMap<String,Model>);
impl Models {
  pub fn load_model(&mut self, loader: &mut Loader, name: &str) { // mgr: &GameMgr, 
    let model = loader.load_to_vao(name);
    self.0.insert(name.to_string(), model);
  }
  pub fn load_models(&mut self, loader: &mut Loader, names: &[&str]) {
    for name in names {
      self.load_model(loader, name);
    }
  }
}

#[derive (Copy, Clone, Default, Debug)]
pub struct Model {
    pub vao_id: VaoID,
    pub vertex_count: VertexCount,
}
impl Model {
  pub fn new(vao_id: VaoID, vertex_count: VertexCount) -> Self {
    Model { vao_id, vertex_count }
  }
}
impl PartialEq for Model {
  fn eq(&self, other: &Self) -> bool {
    (self.vao_id.0 == other.vao_id.0) && 
    (self.vertex_count.0 == other.vertex_count.0)
  }
}
impl Eq for Model {}
// impl Hash for Model {
//   //
// }