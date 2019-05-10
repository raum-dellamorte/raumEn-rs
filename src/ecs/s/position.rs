
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
      position::*,
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
  },
};

const GRAVITY: f32 = 0.5;

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
  type SystemData = ( Read<'a, Handler>,
                      Entities<'a>,
                      ReadStorage<'a, Velocity>,
                      WriteStorage<'a, TmpVelocity>,
                      WriteStorage<'a, Position>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (handler, ent, vel, mut tvel, mut pos) = data;
    let delta = handler.timer.delta;
    // println!("delta {}", delta);
    if delta < 0.0 || delta > 0.04 { return }
    for (_, v, tv, p) in (&ent, &vel, &mut tvel, &mut pos).join() {
      // println!("pos.y {}", pos.y);
      let mut dvel = Vector3f::blank();
      v.vel.scale_to(&mut dvel, delta);
      p.pos += dvel;
      tv.tvel.scale_to(&mut dvel, delta);
      p.pos += dvel;
      tv.tvel.clear();
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
      v.vel.y -= GRAVITY * delta;
    }
  }
}

pub struct PlayerInput;
impl<'a> System<'a> for PlayerInput {
  type SystemData = ( Write<'a, Handler>,
                      Entities<'a>,
                      WriteStorage<'a, Velocity>,
                      WriteStorage<'a, TmpVelocity>,
                      WriteStorage<'a, Falling>,
                      ReadStorage<'a, ActivePlayer>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (mut handler, ents, mut vel, mut tvel, mut falling, player) = data;
    
    let mut d = (&ents, &mut vel, &mut tvel, &player).join().collect::<Vec<_>>();
    if d.len() < 1 { return }
    let _d = &mut d[0];
    
    let (_mx, _my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_f64, 0_f64),
    };
    
    if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { ; }  // Move Forward
    if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { ; } // Move Backward
    if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { ; }         // Strafe Left
    if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { ; }        // Strafe Right
    if handler.read_kb_single_any_of(KCS::new(&[Q]))       { ; }                  // Turn Left
    if handler.read_kb_single_any_of(KCS::new(&[E]))       { ; }                 // Turn Right
    if handler.read_kb_single(KC::new(Space))              {
      _d.1 .vel.y += 10.0;
      falling.insert(_d.0, Falling).expect("Trying to set Falling flag.");
    }                        // Jumping... is useless
    if handler.read_mouse_single(MB::Left)                 { println!("mouse x: {} y: {}", _mx, _my); } // Fire/Select
  }
}

pub struct Collision;
impl<'a> System<'a> for Collision {
  type SystemData = ( Read<'a, Handler>,
                      Read<'a, TerrainNodes>,
                      Entities<'a>,
                      WriteStorage<'a, Position>,
                      WriteStorage<'a, Velocity>,
                      WriteStorage<'a, TmpVelocity>,
                      WriteStorage<'a, Falling>,
                      ReadStorage<'a, Platform>,
                      ReadStorage<'a, ActivePlayer>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (handler, _nodes, ents, mut pos, mut vel, mut tvel, mut falling, pform, player) = data;
    let delta = handler.timer.delta; // 
    if delta < 0.0 || delta > 0.04 { return }
    for (e, p, v, tv, _) in (&ents, &mut pos, &mut vel, &mut tvel, &player).join() {
      let _p2 = &(&p.pos + &v.vel);
      let _p_size = &Vector3f::new(2.0, 2.0, 2.0);
      let _t_size = &mut Vector3f::new(2.0, 0.0, 2.0);
      let _p1m = &(&p.pos + _p_size);
      let _p2m = &(_p2 + _p_size);
      for _tile in (&pform).join() {
        _t_size.y = _tile.d * 200.0;
        let _t = &Vector3f::new(_tile.x as f32, ((_tile.h * 200.0) - 100.0) - _t_size.y + (_p_size.y * 0.5), _tile.z as f32);
        let _tm = &(_t + _t_size);
        // let same_col = terrain_same_col(_p2, _p2m, _t, _tm);
        let collided = terrain_collide(_p2, _p2m, _t, _tm);
        match collided {
          TerrainCollideType::Floor(n) => {
            println!("Collision: Floor({}) | Player: {} Object {}", n, _p2, _t);
            // panic!("Stop the show!");
            // Stop Falling and Y velocity
            v.vel.y = 0.0;
            p.pos.y += n; // make sure the player min is the terrain max
            falling.remove(e);
          }
          TerrainCollideType::Ceiling(n) => {
            // Stop Y velocity
            println!("Collision: Ceiling({}) | Player: {} Object {}", n, _p2, _t);
            v.vel.y = 0.0;
            p.pos.y += n - 0.001; // make sure player head doesn't stick in ceiling causing another collide
            panic!("Stop the show!");
          }
          TerrainCollideType::WallXY(n) => {
            // n is how close the foot of the player is to the top of the wall
            // if we're close enough, we should just climb it.
            println!("Collision: WallXY({}) | Player: {} Object {}", n, _p2, _t);
            if n <= 0.5 {
              tv.tvel.y = v.vel.z;
            }
            v.vel.z = 0.0;
            panic!("Stop the show!");
          }
          TerrainCollideType::WallYZ(n) => {
            println!("Collision: WallYZ({}) | Player: {} Object {}", n, _p2, _t);
            if n <= 0.5 {
              tv.tvel.y = v.vel.x;
            }
            v.vel.x = 0.0;
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
  
  println!("terrain_min.x({}) <= player_min.x({}) < terrain_max.x({}): {:?}", 
      terrain_min.x, player_min.x, terrain_max.x, pminx);
  println!("terrain_min.x({}) <= player_max.x({}) < terrain_max.x({}): {:?}", 
      terrain_min.x, player_max.x, terrain_max.x, pmaxx);
  println!("terrain_min.y({}) <= player_min.y({}) < terrain_max.y({}): {:?}", 
      terrain_min.y, player_min.y, terrain_max.y, pminy);
  println!("terrain_min.y({}) <= player_max.y({}) < terrain_max.y({}): {:?}", 
      terrain_min.y, player_max.y, terrain_max.y, pmaxy);
  println!("terrain_min.z({}) <= player_min.z({}) < terrain_max.z({}): {:?}", 
      terrain_min.z, player_min.z, terrain_max.z, pminz);
  println!("terrain_min.z({}) <= player_max.z({}) < terrain_max.z({}): {:?}", 
      terrain_min.z, player_max.z, terrain_max.z, pmaxz);
  
  println!("xy({}) xz({}) yz({}) ", xy, xz, yz);
  
  let from_top = terrain_max.y - player_min.y;
  let head_space = terrain_min.y - player_max.y;
  
  if xyp { // XY Plane
    TerrainCollideType::WallXY(from_top)
  } else 
  if xzp { // XZ Plane
    if nnn || xnn || nnx || xnx { TerrainCollideType::Floor(from_top) } else { TerrainCollideType::Ceiling(head_space) }
  } else 
  if yzp { // YZ Plane
    TerrainCollideType::WallYZ(from_top)
  } else {
    TerrainCollideType::None
  }
}