
use {
  specs::{
    Component,
    NullStorage,
  },
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct ActivePlayer;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct CurrentNode;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Falling;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct InScene;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct IsPlatform;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct IsTexMod;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct LocalToPlayer;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Moving;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Particle;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct ParticleAlive;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct StartMoving;

