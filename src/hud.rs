
use util::rmatrix::Matrix4f;
use util::rvector::Vector2f;
use util::rvertex::Vertex2D;

// -1.0,1.0,-1.0,-1.0,1.0,1.0,1.0,-1.0

pub struct GuiObj {
  pub verts: Vec<Vertex2D>,
  pub pos: Vector2f,
  pub scale: Vector2f,
  pub transMat: Matrix4f,
}
impl GuiObj {
  pub fn new() -> Self {
    let verts: Vec<Vertex2D> = Vec::new();
    verts.push(Vertex2D {x: -1.0, y:  1.0});
    verts.push(Vertex2D {x: -1.0, y: -1.0});
    verts.push(Vertex2D {x:  1.0, y:  1.0});
    verts.push(Vertex2D {x:  1.0, y: -1.0});
    GuiObj {
      verts: verts,
      pos: Vector2f::new(),
      scale: Vector2f {x: 0.1, y: 0.1},
      transMat: Matrix4f::new(),
    }
  }
  pub fn transformation(&mut self) -> [[f32; 4]; 4] {
    self.calc_transformation();
    self.transMat.as_slice()
  }
  fn calc_transformation(&mut self) {
    self.transMat.setIdentity();
    let x = (&self.pos.x *  2.0_f32) - 1.0_f32;
    let y = (&self.pos.y * -2.0_f32) + 1.0_f32;
    self.transMat.translate_v3f(&Vector3f {x: x, y: y, z: 0.0_f32});
    self.transMat.scale(&Vector3f::new(self.scale.x, self.scale.y, 1.0));
  }
}

pub struct HUD {
  pub elements: Vec<GuiObj>,
}
impl HUD {
  pub fn new() -> Self {
    HUD {
      elements: Vec::new(),
    }
  }
}