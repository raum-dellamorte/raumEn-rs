
use {
  util::{
    // Arc, Mutex, 
    Matrix4f, 
    // RVec, // to get len() of Vector
    Vector3f, 
    XVEC, YVEC, ZVEC, 
    modulo
  },
};

pub struct PosMarker {
  pub pos: Vector3f,
  pub new_pos: Vector3f,
  // pub near: Feelers,
  // pub far: Feelers,
  pub jump_arc: JumpArc,
  pub rx: f32,
  pub ry: f32,
  pub rz: f32,
  pub scale: f32,
  pub distance: f32,
  // pub grav: Grav,
  pub trans_mat: Matrix4f,
  pub moving: bool,
}

impl PosMarker {
  pub fn new() -> Self {
    PosMarker {
      pos: Vector3f::blank(),
      new_pos: Vector3f::blank(),
      // near: Feelers::blank(),
      // far: Feelers::blank(),
      jump_arc: JumpArc::new(),
      rx: 0_f32,
      ry: 0_f32,
      rz: 0_f32,
      scale: 1_f32,
      distance: 0_f32,
      // grav: Grav {
      //   dy: 0_f32,
      //   upward: 0_f32,
      //   fall: false,
      //   time: 0_f32,
      //   ypos0: 0_f32,
      //   new_ground: 0_f32,
      //   peak: 0_f32,
      // },
      trans_mat: Matrix4f::new(),
      moving: false,
    }
  }
  // pub fn prep(&mut self, _world: &mut Box<World>) {
  //   // let grav = &mut self.grav;
  //   // let (u, _) = world.bounds_under_v3f(&self.pos);
  //   // grav.new_ground = u;
  //   // if !grav.fall { grav.ypos0 = self.pos.y; }
  //   // self.new_pos.from_v3f(&self.pos);
  // }
  pub fn move_to_new_pos(&mut self, rate: f32) {
    self.calc_fall(rate);
    self.pos.from_v3f(&self.new_pos);
  }
  // pub fn move_forward(&mut self, world: &mut Box<World>, forward: bool) {
  //   if self.moving { return }
    
  //   // store orig pos final pos and height
  //   let dist = if forward { 2.0 } else { -2.0 };
  //   let mut dest = self.pos.dist_rot_offset(dist, self.ry);
  //   dest.y = world.bounds_under_v3f(&dest).0;
  //   if (self.pos.y - dest.y).abs() > 5.0 { return }
  //   self.jump_arc.init(&self.pos, &dest);
    
  //   self.moving = true;
  // }
  // pub fn strafe(&mut self, world: &mut Box<World>, right: bool) {
  //   if self.moving { return }
    
  //   // store orig pos final pos and height
  //   let dist = if right { 2.0 } else { -2.0 };
  //   let mut dest = self.pos.dist_rot_offset(dist, (self.ry - 90.0).round() );
  //   dest.y = world.bounds_under_v3f(&dest).0;
  //   if (self.pos.y - dest.y).abs() > 5.0 { return }
  //   self.jump_arc.init(&self.pos, &dest);
    
  //   self.moving = true;
  // }
  // pub fn strafe_left(&mut self, world: &mut Box<World>) { self.strafe(world, false); }
  // pub fn strafe_right(&mut self, world: &mut Box<World>) { self.strafe(world, true); }
  // pub fn calc_move_arc(&mut self, world: &mut Box<World>, rate: f32) {
  //   if self.moving {
  //     // get orig pos final pos and height and add rate to percent of arc completion
  //     self.pos.from_v3f(self.jump_arc.calc_pos(rate));
  //     self.moving = !self.jump_arc.fin;
  //   } else {
  //     self.pos.x = self.pos.x.round();
  //     self.pos.z = self.pos.z.round();
  //     let y = world.bounds_under_v3f(&self.pos).0;
  //     if y != self.pos.y {
  //       if y < self.pos.y {
  //         self.pos.y -= 10_f32 * rate;
  //       } else {
  //         self.pos.y = y;
  //       }
  //     }
  //   }
  // }
  // pub fn forward(&mut self, speed: f32, rate: f32, world: &mut Box<World>) {
  //   let dist = speed * rate;
  //   self.near.update(world, &self.pos, self.ry, dist);
  //   let fdist = if dist < 0.0 { -Feelers::DIST } else { Feelers::DIST };
  //   self.far.update(world, &self.pos, self.ry, fdist);
    
  //   if self.grav.fall {
  //     if self.near.can_move_forward() {
  //       self.new_pos.xz_from_v3f(&self.near.forward);
  //       self.new_pos.y = self.pos.y;
  //       self.grav.new_ground = self.near.forward.y;
  //     } else if !(self.near.can_move_left_45() && self.near.can_move_right_45()) {
  //       if self.near.can_move_left_45() {
  //         self.new_pos.xz_from_v3f(&self.near.left_45);
  //         self.new_pos.y = self.pos.y;
  //         self.grav.new_ground = self.near.left_45.y;
  //       } else if self.near.can_move_right_45() {
  //         self.new_pos.xz_from_v3f(&self.near.right_45);
  //         self.new_pos.y = self.pos.y;
  //         self.grav.new_ground = self.near.right_45.y;
  //       } else {
          
  //       }
  //     }
  //   } else {
  //     // Not falling
  //     if self.near.can_move_forward() {
  //       self.new_pos.from_v3f(&self.near.forward);
  //       self.grav.new_ground = self.near.forward.y;
  //     } else {
  //       if self.far.can_jump_forward() {
  //         // auto jump
  //         self.jump();
  //         self.new_pos.from_v3f(&self.near.center);
  //       } else {
  //         // strafe?
  //         if !(self.near.can_move_left_45() && self.near.can_move_right_45()) {
  //           if self.near.can_move_left_45() {
  //             self.new_pos.xz_from_v3f(&self.near.left_45);
  //             self.new_pos.y = self.pos.y;
  //             self.grav.new_ground = self.near.left_45.y;
  //           } else if self.near.can_move_right_45() {
  //             self.new_pos.xz_from_v3f(&self.near.right_45);
  //             self.new_pos.y = self.pos.y;
  //             self.grav.new_ground = self.near.right_45.y;
  //           } else {
              
  //           }
  //         }
  //       }
  //     }
  //   }
  // }
  pub fn calc_fall(&mut self, _rate: f32){
    // let ht = self.new_pos.y;
    // let grav = &mut self.grav;
    // let ground = grav.new_ground;
    // // let prev_dy = grav.dy;
    // if !grav.fall && ht > ground {
    //   grav.fall = true;
    // }
    // if grav.fall && ht < ground {
    //   grav.fall = false;
    //   grav.dy = 0.0;
    //   grav.upward = 0.0;
    //   grav.time = 0.0;
    //   self.new_pos.y = ground;
    // } else if grav.fall && ht >= ground {
    //   // falling
    //   grav.time += rate;
    //   let last_dy = grav.dy;
    //   grav.dy = (grav.upward * grav.time) - (GRAVITY * grav.time * grav.time);
    //   if grav.dy < last_dy && grav.peak == 0.0 { grav.peak = last_dy; println!("Jump Peak {}", grav.peak); }
    //   // println!("grav.dy {}", grav.dy);
    //   self.new_pos.y = grav.ypos0 + grav.dy;
    //   if self.new_pos.y < ground {
    //     grav.fall = false;
    //     grav.dy = 0.0;
    //     grav.upward = 0.0;
    //     grav.time = 0.0;
    //     self.new_pos.y = ground;
    //   }
    // }
  }
  pub fn jump(&mut self) {
    // if !self.grav.fall {
    //   self.grav.upward = GRAVITY * 1.35;
    //   self.grav.fall = true;
    //   self.grav.peak = 0.0;
    // }
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
    let rx = &mut self.rx; let ry = &mut self.ry; let rz = &mut self.rz;
    *rx = modulo(*rx + dx, 360_f32);
    *ry = modulo(*ry + dy, 360_f32);
    *rz = modulo(*rz + dz, 360_f32);
  }
  pub fn turn_left(&mut self) { // + is left! Turning is Counter-Clockwise!
    let ry = &mut self.ry;
    *ry = modulo(*ry + 90_f32, 360_f32);
  }
  pub fn turn_right(&mut self) { // - is right!
    let ry = &mut self.ry;
    *ry = modulo(*ry - 90_f32, 360_f32);
  }
}

#[derive(Clone)]
pub struct Grav {
  pub dy: f32,
  pub upward: f32,
  pub fall: bool,
  pub time: f32,
  pub ypos0: f32,
  pub new_ground: f32,
  pub peak: f32,
}

pub struct Feelers {
  pub center: Vector3f,
  pub forward: Vector3f,
  pub left_45: Vector3f,
  pub right_45: Vector3f,
  pub left_90: Vector3f,
  pub right_90: Vector3f,
  pub left_135: Vector3f,
  pub right_135: Vector3f,
  pub backward: Vector3f,
  pub ry: f32,
}
impl Feelers {
  // const DIST: f32 = 2.0;
  pub fn blank() -> Self {
    Self {
      center: Vector3f::blank(),
      forward:   Vector3f::blank(),
      left_45:   Vector3f::blank(),
      right_45:  Vector3f::blank(),
      left_90:   Vector3f::blank(),
      right_90:  Vector3f::blank(),
      left_135:  Vector3f::blank(),
      right_135: Vector3f::blank(),
      backward:  Vector3f::blank(),
      ry: 0.0,
    }
  }
  // pub fn update(&mut self, world: &mut Box<World>, pos: &Vector3f, ry: f32, dist: f32) { // + is left! - is right! Turns are Counter-Clockwise!
  //   self.center.from_v3f(pos);
  //   self.forward.xz_from_dist_rot_offset(&self.center, dist, ry);
  //   self.left_45.xz_from_dist_rot_offset(&self.center, dist, modulo(ry + 45.0, 360.0));
  //   self.right_45.xz_from_dist_rot_offset(&self.center, dist, modulo(ry - 45.0, 360.0));
  //   self.left_90.xz_from_dist_rot_offset(&self.center, dist, modulo(ry + 90.0, 360.0));
  //   self.right_90.xz_from_dist_rot_offset(&self.center, dist, modulo(ry - 90.0, 360.0));
  //   self.left_135.xz_from_dist_rot_offset(&self.center, dist, modulo(ry + 135.0, 360.0));
  //   self.right_135.xz_from_dist_rot_offset(&self.center, dist, modulo(ry - 135.0, 360.0));
  //   self.backward.xz_from_dist_rot_offset(&self.center, dist, modulo(ry - 180.0, 360.0));
  //   self.ry = ry;
    
  //   self.forward.y = world.bounds_under_v3f(&self.forward).0;
  //   self.left_45.y = world.bounds_under_v3f(&self.left_45).0;
  //   self.right_45.y = world.bounds_under_v3f(&self.right_45).0;
  //   self.left_90.y = world.bounds_under_v3f(&self.left_90).0;
  //   self.right_90.y = world.bounds_under_v3f(&self.right_90).0;
  //   self.left_135.y = world.bounds_under_v3f(&self.left_135).0;
  //   self.right_135.y = world.bounds_under_v3f(&self.right_135).0;
  //   self.backward.y = world.bounds_under_v3f(&self.backward).0;
  // }
  pub fn can_jump_forward(&self) -> bool {
    if self.center.y < self.forward.y {
      if self.forward.y < 5.0 + self.center.y {
        return true
      }
    }
    false
  }
  pub fn can_move_forward(&self) -> bool {
    if self.center.y >= self.forward.y {
      if self.center.y < 50.0 + self.forward.y {
        return true
      }
    }
    false
  }
  pub fn can_move_left_45(&self) -> bool {
    if self.center.y >= self.left_45.y {
      if self.center.y < 50.0 + self.left_45.y {
        return true
      }
    }
    false
  }
  pub fn can_move_right_45(&self) -> bool {
    if self.center.y >= self.right_45.y {
      if self.center.y < 50.0 + self.right_45.y {
        return true
      }
    }
    false
  }
  pub fn can_move_left_90(&self) -> bool {
    if self.center.y >= self.left_45.y {
      if self.center.y < 50.0 + self.left_45.y {
        return true
      }
    }
    false
  }
  pub fn can_move_right_90(&self) -> bool {
    if self.center.y >= self.right_45.y {
      if self.center.y < 50.0 + self.right_45.y {
        return true
      }
    }
    false
  }
}

#[derive(Debug)]
pub struct JumpArc {
  pub orig: Vector3f,
  pub dest: Vector3f,
  pub current: Vector3f,
  pub time: f32,
  pub fin: bool,
}
impl JumpArc {
  const PEAK: f32 = 3.0;
  const JUMPTIME: f32 = 1.5;
  pub fn new() -> Self {
    Self {
      orig: Vector3f::blank(),
      dest: Vector3f::blank(),
      current: Vector3f::blank(),
      time: 0_f32,
      fin: true,
    }
  }
  pub fn init(&mut self, _orig: &Vector3f, _dest: &Vector3f) {
    {
      let (orig, dest, time) = (&mut self.orig, &mut self.dest, &mut self.time);
      *time = 0_f32;
      orig.from_v3f(_orig);
      dest.from_v3f(_dest);
      self.fin = false;
    }
    println!("JumpArc\n{:?}", self);
  }
  pub fn calc_pos(&mut self, delta: f32) -> &Vector3f {
    if !self.fin {
      let (orig, dest, current, time) = (&self.orig, &self.dest, &mut self.current, &mut self.time);
      *time += 5_f32 * delta;
      if *time >= Self::JUMPTIME {
        *time = Self::JUMPTIME;
        current.from_v3f(dest);
        self.fin = true;
      } else {
        let percent = *time / Self::JUMPTIME;
        current.x = orig.x + (percent * ( dest.x - orig.x ));
        current.z = orig.z + (percent * ( dest.z - orig.z ));
        let y = orig.y + (percent * ( dest.y - orig.y));
        current.y = y + (Self::PEAK * if percent < 0.5 { percent * 2.0 } else { (1.0 - percent) * 2.0 });
      }
    }
    &self.current
  }
}
