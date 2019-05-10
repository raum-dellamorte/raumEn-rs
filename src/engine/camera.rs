#![allow(unused_imports,dead_code)]

use {
  glutin::{
    MouseButton as MB,
    // VirtualKeyCode::*,
  },
  // GameMgr,
  Handler,
  ecs::{
    c::{
      Position,
    },
  },
  entities::{
    mobs::Mob,
    // position::PosMarker,
  },
  util::{
    // ZVEC, RVertex,
    Matrix4f, RVec, Vector3f, XVEC, YVEC, modulo, Rc, RefCell, 
  },
};

pub struct Camera {
  pub pos: Vector3f,
  pub pos_bak: Vector3f,
  pub pitch: f32,
  pub pitch_bak: f32,
  pub yaw: f32,
  pub yaw_bak: f32,
  pub roll: f32,
  pub roll_bak: f32,
  pub dist_from_focus_pos: f32,
  pub focus_angle: f32,
  pub focus_ry: f32,
  pub mouse_rate: f32,
  
  to_pos: Vector3f,
  to_focus_pos: Vector3f,
  
  pub view_mat: Matrix4f,
}
impl Default for Camera {
  fn default() -> Self {
    Camera {
      pos: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      pos_bak: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      pitch: 25_f32,
      pitch_bak: 25_f32,
      yaw: 0_f32,
      yaw_bak: 0_f32,
      roll: 0_f32,
      roll_bak: 0_f32,
      dist_from_focus_pos: 40_f32,
      focus_angle: 0_f32,
      focus_ry: 0_f32,
      mouse_rate: 1.0_f32,
      to_pos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      to_focus_pos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      view_mat: Matrix4f::new(),
    }
  }
}
impl Camera {
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
  
  pub fn drift_to_origin(&mut self, rate: f32) {
    if self.pitch != 25.0 {
      self.pitch = drift_to_zero(self.pitch - 25.0, rate, 0.05) + 25.0;
    }
    if self.focus_angle != 0.0 {
      self.focus_angle = drift_to_zero(self.focus_angle, rate, 0.05);
    }
  }

  pub fn calc_pos(&mut self, handler: &mut Handler, follow: &Position) {
    {
      if handler.read_mouse_multi(MB::Right) {
        match handler.cursor_delta {
          Some((dx, dy)) => {
            self.pitch -= (dy as f32) * self.mouse_rate;
            self.focus_angle -= (dx as f32) * self.mouse_rate;
          }
          _ => ()
        }
      } else {
        self.drift_to_origin(handler.timer.delta);
      }
    }
    self.calc_cam_pos(follow, handler.timer.delta);
  }

  pub fn calc_cam_pos(&mut self, follow: &Position, rate: f32) {
    let h_dist: f32 = self.calc_h_distance();
    let v_dist: f32 = self.calc_v_distance() + 10_f32;
    
    let ry_new = follow.rot.y;
    let ry_diff = self.focus_ry - ry_new;
    if ry_diff.abs() > 0.01 {
      self.focus_ry = drift_to_zero(ry_diff, rate, 0.1) + ry_new;
    }
    self.yaw = 180_f32 - (self.focus_ry + self.focus_angle);
    
    let theta = self.focus_ry + self.focus_angle;
    let x_offset = h_dist * theta.to_radians().sin();
    let z_offset = h_dist * theta.to_radians().cos();
    self.pos.x = follow.pos.x - x_offset;
    self.pos.z = follow.pos.z - z_offset;
    self.pos.y = follow.pos.y + v_dist;
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
    let mut marker = mob.pos.borrow_mut();
    marker.distance = self.dist_to_pos(&marker.pos);
    self.to_pos.normalize();
    focus_pos.sub_to(&self.pos, &mut self.to_focus_pos);
    self.to_focus_pos.normalize();
    self.to_focus_pos.dot(&self.to_pos)
  }
  
  pub fn create_view_matrix(&mut self, view_mat: &mut Matrix4f) {
    view_mat.set_identity();
    view_mat.rotate(self.pitch.to_radians(), &XVEC);
    view_mat.rotate(self.yaw.to_radians(), &YVEC);
    let pos = self.pos;
    let mut neg_cam = Vector3f::blank();
    pos.negate_to(&mut neg_cam);
    view_mat.translate_v3f(&neg_cam);
  }
}

pub fn degree_cap(val: f32) -> f32 {
  if (val <= 180.0) && (val > -180.0) { val } else {
    if val > 0.0 { degree_cap(val - 360.0) } else { degree_cap(val + 360.0) }
  }
}

pub fn drift_to_zero(val: f32, rate: f32, min: f32) -> f32 {
  if val == 0.0 { 0.0 } else {
    let min = if (val < 0.0 && min > 0.0) || (val > 0.0 && min < 0.0) { -min } else { min };
    let val = degree_cap(val);
    let out = val - (val * rate);
    if out.abs() >= min.abs() { out } else {
      let out = val - (min * rate);
      if (val < 0.0 && out >= 0.0) || (val > 0.0 && out <= 0.0) { 0.0 } else { out }
    }
  }
}
