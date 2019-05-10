
use {
  specs::{
    Component,
    NullStorage,
  },
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct InScene;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct ActivePlayer;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct IsPlatform;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct IsTexMod;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Falling;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct CurrentNode;

