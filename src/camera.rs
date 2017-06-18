
use entities::entity::Entity;
use entities::position::PosMarker;
use util::rmatrix::Matrix4f;
use util::rvector::{RVec, Vector3f, XVEC, YVEC}; // , ZVEC

pub struct Camera {
  pub pos: Vector3f,
  pub posBak: Vector3f,
  pub pitch: f32,
  pub pitchBak: f32,
  pub yaw: f32,
  pub yawBak: f32,
  pub roll: f32,
  pub rollBak: f32,
  pub distFromFocusPos: f32,
  pub angleAroundFocusPos: f32,
  
  toPos: Vector3f,
  toFocusPos: Vector3f,
  
  pub viewMat: Matrix4f,
}

impl Camera {
  pub fn create() -> Self {
    Camera {
      pos: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      posBak: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      pitch: 25_f32,
      pitchBak: 25_f32,
      yaw: 0_f32,
      yawBak: 0_f32,
      roll: 0_f32,
      rollBak: 0_f32,
      distFromFocusPos: 40_f32,
      angleAroundFocusPos: 0_f32,
      toPos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      toFocusPos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      viewMat: Matrix4f::new()
    }
  }
  
  pub fn view_matrix(&mut self) -> [[f32; 4]; 4] { self.createViewMatrix(); self.viewMat.as_slice() }
  
  pub fn attachListeners() {
    //var camera = this
    //DisplayMgr.mouse.scroll.setListener { dx: f32, dy: f32 ->
    //  camera.distFromFocusPos -= dy * 0.5
    //}
  }
  
  pub fn store(&mut self) {
    self.posBak.from_v3f(&self.pos);
    self.pitchBak = self.pitch;
    self.yawBak = self.yaw;
    self.rollBak = self.roll;
  }

  pub fn restore(&mut self) {
    self.pos.from_v3f(&self.posBak);
    self.pitch = self.pitchBak;
    self.yaw = self.yawBak;
    self.roll = self.rollBak;
  }

  pub fn calc_pos(&mut self, follow: &PosMarker) {
    self.calcPitch();
    self.calcAngle();
    self.calcCamPos(follow);
  }
  
  fn calcPitch(&mut self) {
    //if (DisplayMgr.mouse.isButtonDown(2)) self.pitch -= DisplayMgr.mouse.pos.getDY() * 0.1
  }

  fn calcAngle(&mut self) {
    //if (DisplayMgr.mouse.isButtonDown(2)) self.angleAroundFocusPos -= DisplayMgr.mouse.pos.getDX() * 0.3
  }

  fn calcCamPos(&mut self, follow: &PosMarker) {
    let hDist: f32 = self.calcHDistance();
    let vDist: f32 = self.calcVDistance() + 10_f32;
    let theta = follow.ry + self.angleAroundFocusPos;
    let xOffset = hDist * theta.to_radians().sin();
    let zOffset = hDist * theta.to_radians().cos();
    self.pos.x = follow.pos.x - xOffset;
    self.pos.z = follow.pos.z - zOffset;
    self.pos.y = follow.pos.y + vDist;
    self.yaw = 180_f32 - (follow.ry + self.angleAroundFocusPos);
  }

  fn calcHDistance(&self) -> f32 {self.distFromFocusPos * self.pitch.to_radians().cos()}
  fn calcVDistance(&self) -> f32 {self.distFromFocusPos * self.pitch.to_radians().sin()}

  pub fn reflection(&mut self, height: f32) {
    self.store();
    self.pos.y -= 2.0 * (self.pos.y - height); // y -= dist
    self.invertPitch();
  }

  pub fn invertPitch(&mut self) {
    self.pitch = -self.pitch;
  }

  pub fn distToPos(&mut self, vec: &Vector3f) -> f32 {
    vec.subTo(&self.pos, &mut self.toPos);
    self.toPos.len()
  }

  pub fn angleToEntity(&mut self, focus_pos: &Vector3f, entity: &mut Entity) -> f32 {
    entity.distance = self.distToPos(&entity.marker.pos);
    self.toPos.normalize();
    focus_pos.subTo(&self.pos, &mut self.toFocusPos);
    self.toFocusPos.normalize();
    self.toFocusPos.dot(&self.toPos)
  }
  
  pub fn createViewMatrix(&mut self) {
    self.viewMat.setIdentity();
    self.viewMat.rotate(self.pitch.to_radians(), &XVEC);
    self.viewMat.rotate(self.yaw.to_radians(), &YVEC);
    let pos = self.pos;
    let mut negCam = Vector3f::blank();
    pos.negateTo(&mut negCam);
    self.viewMat.translate_v3f(&negCam);
  }
}

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
  let f = {
    let f = direction;
    let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
    let len = len.sqrt();
    [f[0] / len, f[1] / len, f[2] / len]
  };
  
  let s = [up[1] * f[2] - up[2] * f[1],
           up[2] * f[0] - up[0] * f[2],
           up[0] * f[1] - up[1] * f[0]];
           
  let s_norm = {
    let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    let len = len.sqrt();
    [s[0] / len, s[1] / len, s[2] / len]
  };
  
  let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
           f[2] * s_norm[0] - f[0] * s_norm[2],
           f[0] * s_norm[1] - f[1] * s_norm[0]];
           
  let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
           -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
           -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
           
  [
    [s[0], u[0], f[0], 0.0],
    [s[1], u[1], f[1], 0.0],
    [s[2], u[2], f[2], 0.0],
    [p[0], p[1], p[2], 1.0],
  ]
}
