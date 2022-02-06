
use {
  specs::{
    // , Read
    System, Write, ReadStorage, Entities, Join, 
  },
  crate::{
    constants::DISPLAY,
    Handler,
    ecs::{
      c::{
        Position,
        Rotation,
        ActivePlayer,
      },
    },
  },
};

pub struct CameraToActivePlayer;
impl<'a> System<'a> for CameraToActivePlayer {
  type SystemData = (
    Write<'a, Handler>,
    Entities<'a>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Rotation>,
    ReadStorage<'a, ActivePlayer>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (mut handler, ent, pos, rot, player) = data;
    for (_, p, r, _) in (&ent, &pos, &rot, &player).join() {
      DISPLAY.lock().unwrap().camera.calc_pos(&mut (*handler), p, r);
    }
  }
}
