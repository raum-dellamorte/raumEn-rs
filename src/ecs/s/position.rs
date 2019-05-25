
use {
  glutin::{
    VirtualKeyCode::*,
    MouseButton as MB,
  },
  engine::input::{
    KeyCode as KC,
    KeyCodes as KCS,
  },
  specs::{
    Entities, System, Read, Write, ReadStorage, WriteStorage, Join, 
  },
  Handler, // World, 
  ecs::{
    c::{
      position::{
        *,
        MovementType::*,
      },
      terrain::{
        Platform,
        TerrainNodes,
      },
    },
    // s::{
      
    // },
  },
  flags::*,
  util::{
    Vector3f, 
    modulo,
  },
};

const GRAVITY: f32 = 5.0;

// pub struct MovePlayer;
// impl<'a> System<'a> for MovePlayer {
//   type SystemData = ( Read<'a, Handler>,
//                       ReadStorage<'a, Velocity>,
//                       WriteStorage<'a, TmpVelocity>,
//                       WriteStorage<'a, Position>,
//                     );
//   fn run(&mut self, data: Self::SystemData) {
//     let (handler, vel, mut tvel, mut pos) = data;
//     let delta = handler.timer.delta;
//     let delta = if delta > 0.067 { 0.02 } else { delta };
//     for (vel, tvel, pos) in (&vel, &mut tvel, &mut pos).join() {
//       println!("pos.y {}", pos.y);
//       pos.x += vel.x * delta;
//       pos.y += vel.y * delta;
//       pos.z += vel.z * delta;
//       pos.x += tvel.x * delta;
//       pos.y += tvel.y * delta;
//       pos.z += tvel.z * delta;
//       tvel.clear();
//     }
//   }
// }

pub struct UpdatePos;
impl<'a> System<'a> for UpdatePos {
  type SystemData = (
                      Entities<'a>,
                      ReadStorage<'a, DeltaVelocity>,
                      WriteStorage<'a, PosAdjust>,
                      WriteStorage<'a, Position>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (ent, deltav, mut tvel, mut pos) = data;
    for (_, dvel, tvel, p) in (&ent, &deltav, &mut tvel, &mut pos).join() {
      if dvel.0.is_blank() && tvel.0.is_blank() { continue; }
      p.0 += dvel.0;
      p.0 += tvel.0;
      tvel.0.clear();
    }
  }
}

pub struct ApplyRotation;
impl<'a> System<'a> for ApplyRotation {
  type SystemData = (
                      Entities<'a>,
                      ReadStorage<'a, Rotation>,
                      ReadStorage<'a, Velocity>,
                      WriteStorage<'a, TransformVelocity>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (ent, rot, vel, mut tvel) = data;
    for (_, rot, vel, tvel) in (&ent, &rot, &vel, &mut tvel).join() {
      vel.0.rotate_y_to(&mut tvel.0, rot.0.y);
    }
  }
}

pub struct UpdateDeltaVelocity;
impl<'a> System<'a> for UpdateDeltaVelocity {
  type SystemData = ( Read<'a, Handler>,
                      Entities<'a>,
                      ReadStorage<'a, TransformVelocity>,
                      WriteStorage<'a, JumpArc>,
                      WriteStorage<'a, DeltaVelocity>,
                      WriteStorage<'a, Falling>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (handler, ent, vel, mut jarc, mut dvel, mut falling) = data;
    let delta = handler.timer.delta;
    // println!("delta {}", delta);
    if delta < 0.0 || delta > 0.04 { return }
    for (e, vel, dvel) in (&ent, &vel, &mut dvel).join() {
      match jarc.get_mut(e) {
        Some(ref mut j) if !j.fin => {
          dvel.0 .copy_from_v3f(j.calc_pos(delta));
          if j.check_peak() { falling.insert(e, Falling).expect("Trying to insert Falling flag"); }
        }
        _ => {
          vel.0.scale_to(&mut dvel.0, delta);
        }
      }
    }
  }
}

pub struct ApplyGravity;
impl<'a> System<'a> for ApplyGravity {
  type SystemData = ( Read<'a, Handler>,
                      Entities<'a>,
                      WriteStorage<'a, Velocity>,
                      ReadStorage<'a, Falling>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (handler, ents, mut vel, falling) = data;
    let delta = handler.timer.delta;
    for (_, v, _) in (&ents, &mut vel, &falling).join() {
      // println!("Applying gravity");
      v.0.y -= GRAVITY * delta;
    }
  }
}

pub struct PlayerInput;
impl<'a> System<'a> for PlayerInput {
  type SystemData = ( Write<'a, Handler>,
                      Entities<'a>,
                      WriteStorage<'a, Rotation>,
                      WriteStorage<'a, Velocity>,
                      WriteStorage<'a, PosAdjust>,
                      WriteStorage<'a, Falling>,
                      WriteStorage<'a, MovementType>,
                      WriteStorage<'a, Moving>,
                      ReadStorage<'a, ActivePlayer>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (mut handler, ents, mut rot, mut vel, mut padj, mut falling, mut movement, mut moving, player) = data;
    
    let mut d = (&ents, &mut rot, &mut vel, &mut padj, &player).join().collect::<Vec<_>>();
    if d.is_empty() { return }
    let _d = &mut d[0];
    
    let (_mx, _my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_f64, 0_f64),
    };
    
    if moving.get(_d.0).is_none() && movement.get(_d.0).is_none() && falling.get(_d.0).is_none() {
      if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { // Move Forward
        println!("Trying to go forward");
        movement.insert(_d.0, MovementType::MoveForward).expect("Failed to insert new MoveForward");
        moving.insert(_d.0, Moving).expect("Failed to insert new MoveForward");
      }
      if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { // Move Backward
        movement.insert(_d.0, MovementType::MoveBackward).expect("Failed to insert new MoveBackward");
        moving.insert(_d.0, Moving).expect("Failed to insert new MoveForward");
      }
      if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { // Strafe Left
        movement.insert(_d.0, MovementType::StrafeLeft).expect("Failed to insert new StrafeLeft");
        moving.insert(_d.0, Moving).expect("Failed to insert new MoveForward");
      }
      if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { // Strafe Right
        movement.insert(_d.0, MovementType::StrafeRight).expect("Failed to insert new StrafeRight");
        moving.insert(_d.0, Moving).expect("Failed to insert new MoveForward");
      } 
    }
    if handler.read_kb_single_any_of(KCS::new(&[Q])) {
      let ry = &mut _d.1 .0.y;
      *ry = modulo(*ry + 90_f32, 360_f32);
    }                  // Turn Left
    if handler.read_kb_single_any_of(KCS::new(&[E])) {
      let ry = &mut _d.1 .0.y;
      *ry = modulo(*ry + 90_f32, 360_f32);
    }                 // Turn Right
    if handler.read_kb_single(KC::new(Space))        {
      _d.2 .0.y += 10.0;
      falling.insert(_d.0, Falling).expect("Trying to set Falling flag.");
    }                        // Jumping... is useless
    if handler.read_mouse_single(MB::Left)                 { println!("mouse x: {} y: {}", _mx, _my); } // Fire/Select
  }
}

pub struct ApplyMovement;
impl<'a> System<'a> for ApplyMovement {
  type SystemData = ( Read<'a, Handler>,
                      Entities<'a>,
                      ReadStorage<'a, Platform>,
                      ReadStorage<'a, LocalToPlayer>,
                      ReadStorage<'a, Position>,
                      ReadStorage<'a, Rotation>,
                      WriteStorage<'a, Moving>,
                      WriteStorage<'a, MovementType>,
                      WriteStorage<'a, JumpArc>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (
      handler, ents, wtiles, _near, pos, rot, 
      mut moving, mut movement, mut jump_arcs
    ) = data;
    
    let _delta = handler.timer.delta;
    for (e, pos, rot, jarc, _) in (&ents, &pos, &rot, &mut jump_arcs, &moving).join() {
      let mvmnt = movement.remove(e);
      if let Some(mvmnt) = mvmnt {
        let _tiles = (&ents, &wtiles, /* &near */ ).join();
        let (x, z) = match mvmnt {
          MoveForward => { 
            println!("MoveForward: Getting x z coords to move to", ); 
            pos.grid_fore(rot, 2) 
          }
          MoveBackward => { pos.grid_back(rot, 2) }
          StrafeLeft => { pos.grid_left(rot, 2) }
          StrafeRight => { pos.grid_rigt(rot, 2) }
        };
        println!("Looking for World Tile at x:{} z:{}", x, z);
        let tiles = _tiles.filter_map(|(_, tile /* , _ */)| {
          if tile.x == x && tile.z == z {
            // println!("Found World Tile");
            Some(tile)
          } else {
            // println!("No World Tile Found");
            None
          }
        }).collect::<Vec<_>>();
        for tile in tiles {
          let ymin = tile.pos.y;
          let ymax = ymin + tile.scale.y;
          if (jarc.fin || jarc.dest.y < ymax) && (ymax < (pos.0 .y + JumpArc::PEAK)) {
            jarc.init(&pos.0, &Vector3f { x: tile.pos.x, y: ymax, z: tile.pos.z });
          }
        }
      }
    }
    let mut cleanup = Vec::new();
    for (e, jump_arc, _) in (&ents, &mut jump_arcs, &moving).join() {
      if jump_arc.fin { cleanup.push(e); }
    }
    for e in &cleanup { moving.remove(*e); }
  }
}
// pub fn move_forward(pos: &Position, rot: &Rotation, moving: bool, forward: bool) {
//   if moving { return }
  
//   // store orig pos final pos and height
//   let dist = if forward { 2.0 } else { -2.0 };
//   let mut dest = pos.0.dist_rot_offset(dist, rot.0.y);
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

pub struct Collision;
impl<'a> System<'a> for Collision {
  type SystemData = ( Read<'a, Handler>,
                      Read<'a, TerrainNodes>,
                      Entities<'a>,
                      WriteStorage<'a, Position>,
                      ReadStorage<'a, DeltaVelocity>,
                      WriteStorage<'a, Velocity>,
                      WriteStorage<'a, PosAdjust>,
                      WriteStorage<'a, Falling>,
                      ReadStorage<'a, Platform>,
                      ReadStorage<'a, ActivePlayer>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (handler, _nodes, ents, mut pos, dvel, mut vel, mut padj, mut falling, pform, player) = data;
    let delta = handler.timer.delta; // 
    if delta < 0.0 || delta > 0.04 { return }
    for (e, p, dv, v, pa, _) in (&ents, &mut pos, &dvel, &mut vel, &mut padj, &player).join() {
      let fpos = &mut (&p.0 + &dv.0); // future position
      fpos.x += 0.1;
      fpos.z += 0.1;
      let fpos = &(*fpos);
      let _p_size = &Vector3f::new(1.8, 2.0, 1.8);
      let _t_size = &mut Vector3f::new(2.0, 0.0, 2.0);
      let _p1m = &(&p.0 + _p_size);
      let _p2m = &(fpos + _p_size);
      for _tile in (&pform).join() {
        _t_size.y = _tile.d * 200.0;
        let _t = &_tile.pos;
        let _tm = &(_t + _t_size);
        let collided = terrain_collide(fpos, _p2m, _t, _tm);
        match collided {
          TerrainCollideType::Floor(n) => {
            println!("Collision: Floor({}) | Player: {} Object {}", n, fpos, _t);
            // panic!("Stop the show!");
            // Stop Falling and Y velocity
            v.0.y = 0.0;
            pa.0.y = dv.0.y + n;
            // println!("{} = {} + {}", tv.0.y, dv.0.y, n);
            falling.remove(e);
          }
          TerrainCollideType::Ceiling(n) => {
            // Stop Y velocity
            println!("Collision: Ceiling({}) | Player: {} Object {}", n, fpos, _t);
            // v.0.y = 0.0;
            // pa.0.y = (dv.0.y - n) - 0.001; // make sure player head doesn't stick in ceiling causing another collide
            // println!("{} = ({} - {}) - 0.001", pa.0.y, dv.0.y, n);
            // panic!("Stop the show!");
          }
          TerrainCollideType::WallXY(n) => {
            // n is how close the foot of the player is to the top of the wall
            // if we're close enough, we should just climb it.
            println!("Collision: WallXY({}) | Player: {} Object {}", n, fpos, _t);
            // if n <= 0.5 {
            //   pa.0.y = dv.0.z;
            // }
            // v.0.z = 0.0;
            // panic!("Stop the show!");
          }
          TerrainCollideType::WallYZ(n) => {
            println!("Collision: WallYZ({}) | Player: {} Object {}", n, fpos, _t);
            // if n <= 0.5 {
            //   pa.0.y = dv.0.x;
            // }
            // v.0.x = 0.0;
            // panic!("Stop the show!");
          }
          TerrainCollideType::None => {
            // 
          }
        }
      }
    }
  }
}

#[derive(Debug)]
enum TerrainCollideType {
  Floor(f32),
  WallXY(f32),
  WallYZ(f32),
  Ceiling(f32),
  None,
}

// fn terrain_same_col(player_min: &Vector3f, player_max: &Vector3f, terrain_min: &Vector3f, terrain_max: &Vector3f) -> bool {
//   let pminx = ( terrain_min.x <= player_min.x && player_min.x < terrain_max.x );
//   let pmaxx = ( terrain_min.x <= player_max.x && player_max.x < terrain_max.x );
//   let pminz = ( terrain_min.z <= player_min.z && player_min.z < terrain_max.z );
//   let pmaxz = ( terrain_min.z <= player_max.z && player_max.z < terrain_max.z );
//   (pminx && pminz) || 
//   (pmaxx && pminz) || 
//   (pminx && pmaxz) || 
//   (pmaxx && pmaxz)
// }

fn terrain_collide(player_min: &Vector3f, player_max: &Vector3f, terrain_min: &Vector3f, terrain_max: &Vector3f) -> TerrainCollideType {
  let pminx = terrain_min.x <= player_min.x && player_min.x < terrain_max.x;
  let pmaxx = terrain_min.x <= player_max.x && player_max.x < terrain_max.x;
  let xl = if pminx { player_min.x - terrain_max.x } else if pmaxx { player_max.x - terrain_min.x } else { 0.0 };
  
  let pminy = terrain_min.y <= player_min.y && player_min.y < terrain_max.y;
  let pmaxy = terrain_min.y <= player_max.y && player_max.y < terrain_max.y;
  let yl = if pminy { player_min.y - terrain_max.y } else if pmaxy { player_max.y - terrain_min.y } else { 0.0 };
  
  let pminz = terrain_min.z <= player_min.z && player_min.z < terrain_max.z;
  let pmaxz = terrain_min.z <= player_max.z && player_max.z < terrain_max.z;
  let zl = if pminz { player_min.z - terrain_max.z } else if pmaxz { player_max.z - terrain_min.z } else { 0.0 };
  
  let allzero = xl == 0.0 && yl == 0.0 && zl == 0.0;
  
  if allzero || !( pminx || pmaxx || pminy || pmaxy || pminz || pmaxz ) { return TerrainCollideType::None }
  
  let xy = xl * yl;
  let xz = xl * zl;
  let yz = yl * zl;
  let xyp = xy > xz && xy > yz;
  let xzp = xz > xy && xz > yz;
  let yzp = yz > xy && yz > xz;
  let nnn = pminx && pminy && pminz;
  let xnn = pmaxx && pminy && pminz;
  let nxn = pminx && pmaxy && pminz;
  let nnx = pminx && pminy && pmaxz;
  let xxn = pmaxx && pmaxy && pminz;
  let xnx = pmaxx && pminy && pmaxz;
  let nxx = pminx && pmaxy && pmaxz;
  let xxx = pmaxx && pmaxy && pmaxz;
  
  let collided = nnn || xnn || nxn || nnx || xxn || xnx || nxx || xxx;
  
  if !collided { return TerrainCollideType::None }
  
  // println!("terrain_min.x({}) <= player_min.x({}) < terrain_max.x({}): {:?}", 
  //     terrain_min.x, player_min.x, terrain_max.x, pminx);
  // println!("terrain_min.x({}) <= player_max.x({}) < terrain_max.x({}): {:?}", 
  //     terrain_min.x, player_max.x, terrain_max.x, pmaxx);
  // println!("terrain_min.y({}) <= player_min.y({}) < terrain_max.y({}): {:?}", 
  //     terrain_min.y, player_min.y, terrain_max.y, pminy);
  // println!("terrain_min.y({}) <= player_max.y({}) < terrain_max.y({}): {:?}", 
  //     terrain_min.y, player_max.y, terrain_max.y, pmaxy);
  // println!("terrain_min.z({}) <= player_min.z({}) < terrain_max.z({}): {:?}", 
  //     terrain_min.z, player_min.z, terrain_max.z, pminz);
  // println!("terrain_min.z({}) <= player_max.z({}) < terrain_max.z({}): {:?}", 
  //     terrain_min.z, player_max.z, terrain_max.z, pmaxz);
  
  // println!("xy({}) xz({}) yz({}) ", xy, xz, yz);
  
  let from_top = terrain_max.y - player_min.y;
  let head_space = terrain_min.y - player_max.y;
  
  if xyp { // XY Plane
    TerrainCollideType::WallXY(from_top)
  } else if xzp { // XZ Plane
    if nnn || xnn || nnx || xnx { TerrainCollideType::Floor(from_top) } else { TerrainCollideType::Ceiling(head_space) }
  } else if yzp { // YZ Plane
    TerrainCollideType::WallYZ(from_top)
  } else {
    TerrainCollideType::None
  }
}
