
use {
  specs::{
    // , Read
    System, Write, ReadStorage, Entities, Join, 
  },
  Camera,
  Handler,
  ecs::{
    c::{
      Position,
    },
  },
  flags::{
    ActivePlayer,
  },
};

pub struct CameraToActivePlayer;
impl<'a> System<'a> for CameraToActivePlayer {
  type SystemData = (
    Write<'a, Handler>,
    Write<'a, Camera>,
    Entities<'a>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, ActivePlayer>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (mut handler, mut camera, ent, pos, player) = data;
    for (_, p, _) in (&ent, &pos, &player).join() {
      camera.calc_pos(&mut (*handler), p);
    }
  }
}
