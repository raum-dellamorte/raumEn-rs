
use entities::position::PosMarker;
use model::import::load_obj;
use model::mesh::Mesh;

pub struct Entity {
  pub marker: PosMarker,
  pub mesh: Option<Mesh>,
  pub h: f32,
  pub w: f32,
  pub distance: f32,
}

impl Entity {
  pub fn new() -> Self {
    Entity {
      marker: PosMarker::new(),
      mesh: None,
      h: 0_f32,
      w: 0_f32,
      distance: 0_f32,
    }
  }
  
  pub fn load_mesh(&mut self, mesh_name: &str) -> &Self {
    self.mesh = match load_obj(mesh_name) {
      Ok(mesh) => Some(mesh),
      Err(_) => {println!("Mesh {} failed to load.", mesh_name); None },
    };
    self
  }
}
