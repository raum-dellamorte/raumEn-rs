#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod mesh;
pub mod import;

pub use model::mesh::Mesh;

use specs::*;
use util::{rgl::*, HashMap, };

#[derive(Default)]
pub struct Models(pub HashMap<String,Model>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelComponent(pub String);

#[derive (Debug)]
pub struct Model {
    pub vao_id: VaoID,
    pub vertex_count: VertexCount,
}
impl Model {
  pub fn new(id: u32, count: i32) -> Self {
    Model { vao_id: VaoID(id), vertex_count: VertexCount(count) }
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
