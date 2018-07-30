
//use glium::{Display, Depth, DepthTest, DrawParameters, Frame, IndexBuffer, Surface, VertexBuffer}; // Program,
//use glium::draw_parameters::BackfaceCullingMode;
//use glium::texture::CompressedSrgbTexture2d;

use entities::mobs::Mob;
use entities::position::PosMarker;
use util::rmatrix::Matrix4f;
use util::rvector::{RVec, Vector3f, XVEC, YVEC}; // , ZVEC
// use util::rvertex::RVertex;

pub struct Camera {
  // pub display: Display,
  // pub target: Option<Frame>,
  pub dimensions: (u32, u32),
  pub pos: Vector3f,
  pub pos_bak: Vector3f,
  pub pitch: f32,
  pub pitch_bak: f32,
  pub yaw: f32,
  pub yaw_bak: f32,
  pub roll: f32,
  pub roll_bak: f32,
  pub dist_from_focus_pos: f32,
  pub angle_around_focus_pos: f32,
  
  to_pos: Vector3f,
  to_focus_pos: Vector3f,
  
  pub view_mat: Matrix4f,
  pub proj_mat: Matrix4f,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      // display: display,
      // target: None,
      dimensions: (0, 0),
      pos: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      pos_bak: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      pitch: 25_f32,
      pitch_bak: 25_f32,
      yaw: 0_f32,
      yaw_bak: 0_f32,
      roll: 0_f32,
      roll_bak: 0_f32,
      dist_from_focus_pos: 40_f32,
      angle_around_focus_pos: 0_f32,
      to_pos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      to_focus_pos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      view_mat: Matrix4f::new(),
      proj_mat: Matrix4f::new(),
    }
  }
  
  pub fn update_size(&mut self, dimensions: (u32, u32)) {
    self.dimensions = dimensions;
  }
  
  // pub fn finish(&mut self) {
  //   self.target.take().unwrap().finish().unwrap();
  // }
  
  // pub fn draw_entity(&mut self, ent: &mut Entity) {
  //   let mut frame: Option<Frame> = self.target.take();
  //   self.draw_entity_surface(ent, &mut frame);
  //   self.target = frame;
  // }
  
  // pub fn draw_entity_surface<S>(&mut self, ent: &mut Entity, fbo: &mut Option<S>) where S: Surface {
  //   let light = [0.0, 1000.0, -7000.0_f32];
  //   let mesh = ent.model.mesh.as_ref().unwrap().buffers.as_ref().unwrap();
  //   let trans = ent.marker.transformation();
  //   let view = self.view_matrix();
  //   let proj = self.projection();
  //   let params = DrawParameters {
  //     depth: Depth {
  //       test: DepthTest::IfLess,
  //       write: true,
  //       .. Default::default()
  //     },
  //     backface_culling: BackfaceCullingMode::CullClockwise,
  //     .. Default::default()
  //   };
  //   match ent.model.texture.as_ref() {
  //     Some(texture) => self.draw_with_texture(texture, &params, &mesh.verts, &mesh.indcs, trans, view, proj, light, fbo),
  //     None => self.draw_with_color(&params, &mesh.verts, &mesh.indcs, trans, view, proj, light, fbo),
  //   };
    
  // }
  
  // fn draw_with_texture<S>(&mut self, texture: &CompressedSrgbTexture2d, params: &DrawParameters,
  //     verts: &VertexBuffer<Vertex>, indcs: &IndexBuffer<u16>,
  //     trans: [[f32; 4]; 4], view: [[f32; 4]; 4], proj: [[f32; 4]; 4], light: [f32; 3],
  //     fbo: &mut Option<S>) where S: Surface {
  //   let program = model::get_shader(&self.display, true);
  //   match fbo.as_mut() {
  //     Some(target) => target.draw(
  //                       verts, indcs, &program,
  //                       &uniform! {
  //                         transform: trans,
  //                         view: view,
  //                         projection: proj,
  //                         u_light: light,
  //                         tex: texture,
  //                       },
  //                       params
  //                     ).unwrap(),
  //     None => (),
  //   };
  // }
  
  // fn draw_with_color<S>(&mut self, params: &DrawParameters, verts: &VertexBuffer<Vertex>, indcs: &IndexBuffer<u16>,
  //     trans: [[f32; 4]; 4], view: [[f32; 4]; 4], proj: [[f32; 4]; 4], light: [f32; 3],
  //     fbo: &mut Option<S>) where S: Surface {
  //   let program = model::get_shader(&self.display, false);
  //   fbo.as_mut().unwrap().draw(
  //     verts, indcs, &program,
  //     &uniform! {
  //       transform: trans,
  //       view: view,
  //       projection: proj,
  //       u_light: light },
  //     params
  //   ).unwrap();
  // }
  
  pub fn view_matrix(&mut self) -> [f32; 16] { self.create_view_matrix(); self.view_mat.as_slice() }
  
  // pub fn get_dimensions(&self) -> (u32, u32) {
  //   match self.target.as_ref() {
  //     Some(target) => target.get_dimensions(),
  //     None => (0, 0),
  //   }
  // }
  
  pub fn projection(&mut self) -> [f32; 16] {
    let (width, height) = self.dimensions;
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;
    let y_scale = 1_f32 / (fov / 2_f32).tan();
    let frustum_length = zfar - znear;
    
    self.proj_mat.set_identity();
    self.proj_mat.m00 = y_scale / aspect_ratio;
    self.proj_mat.m11 = y_scale;
    self.proj_mat.m22 = -((zfar + znear) / frustum_length);
    self.proj_mat.m23 = -1_f32;
    self.proj_mat.m32 = -(2_f32 * znear * zfar) / frustum_length;
    self.proj_mat.m33 = 0_f32;
    self.proj_mat.as_slice()
  }
  
  pub fn attach_listeners() {
    //var camera = this
    //DisplayMgr.mouse.scroll.set_listener { dx: f32, dy: f32 ->
    //  camera.dist_from_focus_pos -= dy * 0.5
    //}
  }
  
  pub fn store(&mut self) {
    self.pos_bak.from_v3f(&self.pos);
    self.pitch_bak = self.pitch;
    self.yaw_bak = self.yaw;
    self.roll_bak = self.roll;
  }

  pub fn restore(&mut self) {
    self.pos.from_v3f(&self.pos_bak);
    self.pitch = self.pitch_bak;
    self.yaw = self.yaw_bak;
    self.roll = self.roll_bak;
  }

  pub fn calc_pos(&mut self, follow: &PosMarker) {
    self.calc_pitch();
    self.calc_angle();
    self.calc_cam_pos(follow);
  }
  
  fn calc_pitch(&mut self) {
    //if (DisplayMgr.mouse.is_button_down(2)) self.pitch -= DisplayMgr.mouse.pos.get_dy() * 0.1
  }

  fn calc_angle(&mut self) {
    //if (DisplayMgr.mouse.is_button_down(2)) self.angle_around_focus_pos -= DisplayMgr.mouse.pos.get_dx() * 0.3
  }

  fn calc_cam_pos(&mut self, follow: &PosMarker) {
    let h_dist: f32 = self.calc_h_distance();
    let v_dist: f32 = self.calc_v_distance() + 10_f32;
    let theta = follow.ry + self.angle_around_focus_pos;
    let x_offset = h_dist * theta.to_radians().sin();
    let z_offset = h_dist * theta.to_radians().cos();
    self.pos.x = follow.pos.x - x_offset;
    self.pos.z = follow.pos.z - z_offset;
    self.pos.y = follow.pos.y + v_dist;
    self.yaw = 180_f32 - (follow.ry + self.angle_around_focus_pos);
  }

  fn calc_h_distance(&self) -> f32 {self.dist_from_focus_pos * self.pitch.to_radians().cos()}
  fn calc_v_distance(&self) -> f32 {self.dist_from_focus_pos * self.pitch.to_radians().sin()}

  pub fn reflection(&mut self, height: f32) {
    self.store();
    self.pos.y -= 2.0 * (self.pos.y - height); // y -= dist
    self.invert_pitch();
  }

  pub fn invert_pitch(&mut self) {
    self.pitch = -self.pitch;
  }

  pub fn dist_to_pos(&mut self, vec: &Vector3f) -> f32 {
    vec.sub_to(&self.pos, &mut self.to_pos);
    self.to_pos.len()
  }

  pub fn angle_to_entity(&mut self, focus_pos: &Vector3f, mob: &mut Mob) -> f32 {
    let mut marker = mob.pos.lock().unwrap();
    marker.distance = self.dist_to_pos(&marker.pos);
    self.to_pos.normalize();
    focus_pos.sub_to(&self.pos, &mut self.to_focus_pos);
    self.to_focus_pos.normalize();
    self.to_focus_pos.dot(&self.to_pos)
  }
  
  pub fn create_view_matrix(&mut self) -> [f32; 16] {
    self.view_mat.set_identity();
    self.view_mat.rotate(self.pitch.to_radians(), &XVEC);
    self.view_mat.rotate(self.yaw.to_radians(), &YVEC);
    let pos = self.pos;
    let mut neg_cam = Vector3f::blank();
    pos.negate_to(&mut neg_cam);
    self.view_mat.translate_v3f(&neg_cam);
    self.view_mat.as_slice()
  }
}
