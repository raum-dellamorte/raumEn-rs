
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

const GRAVITY: f32 = 10.0;

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

pub struct ApplyRotation;
impl<'a> System<'a> for ApplyRotation {
  type SystemData = (
                      Entities<'a>,
                      WriteStorage<'a, Rotator>,
                      ReadStorage<'a, Rotation>,
                      ReadStorage<'a, Velocity>,
                      WriteStorage<'a, TransformVelocity>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (ent, mut rtr, rot, vel, mut tvel) = data;
    for (_, rt, rot, vel, tvel) in (&ent, &mut rtr, &rot, &vel, &mut tvel).join() {
      rt.set_point(vel.0)
        .set_angle(rot.0.y)
        .rotate()
        .get_point(&mut tvel.0);
    }
  }
}

pub struct UpdateDeltaVelocity;
impl<'a> System<'a> for UpdateDeltaVelocity {
  type SystemData = ( Read<'a, Handler>,
                      Entities<'a>,
                      ReadStorage<'a, TransformVelocity>,
                      WriteStorage<'a, DeltaVelocity>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (handler, ent, tvel, mut dvel) = data;
    let delta = handler.timer.delta;
    // println!("delta {}", delta);
    if delta < 0.0 || delta > 0.04 { return }
    for (_, vel, dvel) in (&ent, &tvel, &mut dvel).join() {
      vel.0.scale_to(&mut dvel.0, delta);
    }
  }
}

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

pub struct PlayerInput;
impl<'a> System<'a> for PlayerInput {
  type SystemData = ( Write<'a, Handler>,
                      Entities<'a>,
                      WriteStorage<'a, Rotation>,
                      WriteStorage<'a, Velocity>,
                      WriteStorage<'a, Falling>,
                      ReadStorage<'a, ActivePlayer>,
                    );
  fn run(&mut self, data: Self::SystemData) {
    let (mut handler, ents, mut rot, mut vel, mut falling, player) = data;
    let delta = handler.timer.delta;
    
    for (e, rot, vel, _) in (&ents, &mut rot, &mut vel, &player).join() {
      let (_mx, _my) = match handler.cursor_pos {
        Some(xy) => xy,
        None     => (0_f64, 0_f64),
      };
      
      if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { // Move Forward
        vel.0.z += 10.0 * delta;
        if vel.0.z > 10.0 { vel.0.z = 10.0; }
      } else if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { // Move Backward
        vel.0.z -= 10.0 * delta;
        if vel.0.z < -10.0 { vel.0.z = -10.0; }
      } else {
        vel.0.z = if vel.0.z > 0.1 {
          vel.0.z - (20.0 * delta)
        } else if vel.0.z < -0.1 {
          vel.0.z + (20.0 * delta)
        } else {
          0.0
        }
      }
      if handler.read_kb_multi_any_of(KCS::new(&[E])) { // Strafe Right
        vel.0.x -= 10.0 * delta;
        if vel.0.x < -10.0 { vel.0.x = -10.0; }
      } else if handler.read_kb_multi_any_of(KCS::new(&[Q])) { // Strafe Left
        vel.0.x += 10.0 * delta;
        if vel.0.x > 10.0 { vel.0.x = 10.0; }
      } else {
        vel.0.x = if vel.0.x > 0.1 {
          vel.0.x - (20.0 * delta)
        } else if vel.0.z < -0.1 {
          vel.0.x + (20.0 * delta)
        } else {
          0.0
        }
      }
      if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { // Turn Right
        rot.0.y = modulo(rot.0.y - (30_f32 * delta), 360_f32);
      }
      if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { // Turn Left
        rot.0.y = modulo(rot.0.y + (30_f32 * delta), 360_f32);
      }
      if handler.read_kb_single(KC::new(Space)) { // Jumping... is useless
        vel.0.y += 10.0;
        falling.insert(e, Falling).expect("Trying to set Falling flag.");
      }
      if handler.read_kb_single(KC::new(I)) {
        println!("I for Info");
        println!("vel {:?}", vel.0);
        println!("rot {:?}", rot.0);
      }
      if handler.read_mouse_single(MB::Left) { println!("mouse x: {} y: {}", _mx, _my); } // Fire/Select
    }
  }
}

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
      let fpos = &mut (p.0 + dv.0); // future position
      fpos.x += 0.1;
      fpos.z += 0.1;
      let fpos = *fpos;
      let _p_size = Vector3f::new(1.8, 2.0, 1.8);
      let mut _t_size = &mut Vector3f::new(2.0, 0.0, 2.0);
      let _p1m = p.0 + _p_size;
      let _p2m = fpos + _p_size;
      for _tile in (&pform).join() {
        _t_size.y = _tile.d * 200.0;
        let _t = _tile.pos;
        let _tm = _t + *_t_size;
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

fn terrain_collide(player_min: Vector3f, player_max: Vector3f, terrain_min: Vector3f, terrain_max: Vector3f) -> TerrainCollideType {
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
