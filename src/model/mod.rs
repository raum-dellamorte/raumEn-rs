#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod mesh;
pub mod import;

pub use model::mesh::Mesh;

#[derive (Debug, Eq, Hash, PartialEq)]
pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: i32,
}
impl RawModel {
  pub fn new(id: u32, count: i32) -> Self {
    RawModel { vao_id: id, vertex_count: count }
  }
}
