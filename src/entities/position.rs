
use std::sync::{Arc, Mutex};

use terrain::World;
use util::rvector::{Vector3f, XVEC, YVEC, ZVEC};
use util::rmatrix::Matrix4f;

const GRAVITY: f32 = 9.8;

pub struct PosMarker {
  pub pos: Vector3f,
  pub new_pos: Vector3f,
  pub fut_pos: Vector3f,
  pub rx: f32,
  pub ry: f32,
  pub rz: f32,
  pub scale: f32,
  pub distance: f32,
  pub grav: Grav,
  pub trans_mat: Matrix4f,
}

impl PosMarker {
  pub fn new() -> Self {
    PosMarker {
      pos: Vector3f::blank(),
      new_pos: Vector3f::blank(),
      fut_pos: Vector3f::blank(),
      rx: 0_f32,
      ry: 0_f32,
      rz: 0_f32,
      scale: 1_f32,
      distance: 0_f32,
      grav: Grav {
        dy: 0_f32,
        upward: 0_f32,
        fall: true,
        time: 0_f32,
        ground: 0_f32,
        new_ground: 0_f32,
        peak: 0_f32,
      },
      trans_mat: Matrix4f::new(),
    }
  }
  pub fn prep(&mut self, world_arc: Arc<Mutex<World>>) {
    let world = world_arc.lock().unwrap();
    let grav = &mut self.grav;
    let (u, _) = world.bounds_under_v3f(&self.pos);
    grav.ground = u;
    grav.new_ground = u;
    self.new_pos.from_v3f(&self.pos);
  }
  pub fn move_to_new_pos(&mut self, rate: f32) {
    self.calc_fall(rate);
    self.pos.from_v3f(&self.new_pos);
  }
  pub fn forward(&mut self, speed: f32, rate: f32, world_arc: Arc<Mutex<World>>) {
    let dist = speed * rate;
    let world = world_arc.lock().unwrap();
    self.new_pos.x = self.pos.x + (dist * self.ry.to_radians().sin());
    self.new_pos.z = self.pos.z + (dist * self.ry.to_radians().cos());
    self.fut_pos.x = self.pos.x + ((dist + 0.5) * self.ry.to_radians().sin());
    self.fut_pos.z = self.pos.z + ((dist + 0.5) * self.ry.to_radians().cos());
    self.new_pos.y = self.pos.y;
    let (u, _) = world.bounds_under_v3f(&self.new_pos);
    self.grav.new_ground = u;
    self.fut_pos.y = self.new_pos.y + 1.0;
    let (fu, _) = world.bounds_under_v3f(&self.fut_pos); // fu :) future upper
    if !self.grav.fall && self.new_pos.y < u {
      // terrain obsticle
      // try left and right to see if we can progress by sliding
      self.new_pos.from_v3f(&self.pos);
    } else {
      if !self.grav.fall && ((self.fut_pos.y >= fu) && (self.new_pos.y < fu)) {
        // auto jump
        self.jump(rate);
      }
    }
  }
  pub fn calc_fall(&mut self, rate: f32){
    let ht = self.new_pos.y;
    let grav = &mut self.grav;
    let ground = grav.new_ground;
    if grav.fall && ht < ground {
      grav.fall = false;
      grav.dy = 0.0;
      grav.upward = 0.0;
      grav.time = 0.0;
      self.new_pos.y = ground;
    } else if grav.fall && ht >= ground {
      // falling
      grav.time += rate;
      if grav.upward > 0.0 {
        grav.dy = -grav.upward;
        grav.upward -= GRAVITY * grav.time * grav.time;
      }
      if grav.upward <= 0.0 && grav.dy < 0.0 {
        grav.upward = 0.0;
        grav.dy = 0.0;
        grav.time = 0.0;
      }
      if grav.upward == 0.0 {
        grav.dy += GRAVITY * grav.time * grav.time;
      }
      println!("grav.dy {}", grav.dy);
      self.new_pos.y -= grav.dy;
      if self.new_pos.y < ground {
        grav.fall = false;
        grav.dy = 0.0;
        grav.upward = 0.0;
        grav.time = 0.0;
        self.new_pos.y = ground;
      }
    } else if !grav.fall && ht > ground {
      grav.fall = true;
    }
  }
  pub fn jump(&mut self, rate: f32) {
    if !self.grav.fall {
      self.grav.upward = 35.0 * rate;
      self.grav.fall = true;
    }
  }
  pub fn transformation(&mut self) -> &Matrix4f {
    self.calc_transformation();
    &self.trans_mat
  }
  fn calc_transformation(&mut self) {
    self.trans_mat.set_identity();
    self.trans_mat.translate_v3f(&self.pos);
    self.trans_mat.rotate(self.rx.to_radians(), &XVEC);
    self.trans_mat.rotate(self.ry.to_radians(), &YVEC);
    self.trans_mat.rotate(self.rz.to_radians(), &ZVEC);
    self.trans_mat.scale(&Vector3f::new(self.scale, self.scale, self.scale));
  }
  pub fn inc_rot(&mut self, dx: f32, dy: f32, dz: f32) {
    self.rx += dx;
    if self.rx > 360_f32 { self.rx -= 360_f32; }
    self.ry += dy;
    if self.ry > 360_f32 { self.ry -= 360_f32; }
    self.rz += dz;
    if self.rz > 360_f32 { self.rz -= 360_f32; }
  }
}

#[derive(Clone)]
pub struct Grav {
  pub dy: f32,
  pub upward: f32,
  pub fall: bool,
  pub time: f32,
  pub ground: f32,
  pub new_ground: f32,
  pub peak: f32,
}
