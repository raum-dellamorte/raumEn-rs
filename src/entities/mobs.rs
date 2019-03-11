
use glutin::VirtualKeyCode::*;
use glutin::MouseButton as MB;

use entities::PosMarker;
use Handler;
use engine::input::KeyCode as KC;
use engine::input::KeyCodes as KCS;
// use terrain::World;
use util::{Vector3f, HashMap, Rc, RefCell, };

pub struct Mob {
  pub name: String,
  pub entity: String,
  pub pos: Rc<RefCell<PosMarker>>,
  pub speed: f32,
  pub turn: f32,
  pub stats: HashMap<String, f32>,
}

impl Mob {
  pub fn new(entity: &str, pos: Rc<RefCell<PosMarker>>) -> Self {
    Mob {
      name: "".to_string(),
      entity: entity.to_string(),
      pos: pos,
      speed: 20_f32,
      turn: 180_f32,
      stats: HashMap::new(),
    }
  }
  pub fn move_mob(&mut self, handler: &mut Handler) -> &Self { // , world: &mut Box<World>
    let rate = handler.timer.delta;
    if rate > 0.07 { return self; }
    let (_mx, _my) = match handler.cursor_pos {
      Some(xy) => xy,
      None     => (0_f64, 0_f64),
    };
    // let mut marker = self.pos.borrow_mut();
    // marker.prep(world);
    // if handler.read_kb_multi_any_of(KCS::new(&[Up,    W])) { marker.move_forward( world, true ); }  // Move Forward
    // if handler.read_kb_multi_any_of(KCS::new(&[Down,  S])) { marker.move_forward( world, false ); } // Move Backward
    // if handler.read_kb_multi_any_of(KCS::new(&[Left,  A])) { marker.strafe_left( world ); }         // Strafe Left
    // if handler.read_kb_multi_any_of(KCS::new(&[Right, D])) { marker.strafe_right( world ); }        // Strafe Right
    // if handler.read_kb_single_any_of(KCS::new(&[Q]))       { marker.turn_left(); }                  // Turn Left
    // if handler.read_kb_single_any_of(KCS::new(&[E]))       { marker.turn_right(); }                 // Turn Right
    // if handler.read_kb_single(KC::new(Space))              { marker.jump() }                        // Jumping... is useless
    // // if handler.read_mouse_single(MB::Left)                 { println!("mouse x: {} y: {}", _mx, _my); } // Fire/Select
    // marker.calc_move_arc(world, rate); // move_to_new_pos(rate)
    self
  }
  pub fn pos_copy(&self, v: &mut Vector3f) {
    let marker = self.pos.borrow();
    let x = marker.pos.x;
    let y = marker.pos.y;
    let z = marker.pos.z;
    v.from_f32(x, y, z);
  }
}
