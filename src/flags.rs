
use {
  specs::{
    Component,
    NullStorage,
  },
};

#[derive(Default)]
pub struct InScene;
impl Component for InScene {
  type Storage = NullStorage<Self>;
}
