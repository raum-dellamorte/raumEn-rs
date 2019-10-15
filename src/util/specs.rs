

use {
  specs::{
    World, WorldExt, Component, Entity, 
  },
};


pub fn ins_comp<T: Component>(
  world: &mut World, e: Entity, thing: T, text: &str, f: &dyn Fn(&mut T) -> ()
) {
  let mut things = world.write_component::<T>();
  let err_msg = format!("Failed Component Insertion: {}", text);
  let mut thing = thing;
  f(&mut thing);
  things.insert(e, thing).expect(&err_msg);
}

pub fn mod_comp<T: Component + Default>(
  world: &mut World, e: Entity, msg: &str, f: &dyn Fn(&mut T) -> ()
) {
  {
    let mut things = world.write_component::<T>();
    let thing = things.get_mut(e);
    if let Some(thing) = thing {
      f(thing);
      return;
    }
  }
  ins_comp::<T>(world, e, T::default(), msg, f);
}

pub fn ins_flag<T: Component + Default>(
  world: &mut World, e: Entity, text: &str
) {
  let mut things = world.write_component::<T>();
  let err_msg = format!("Failed Component Insertion: {}", text);
  things.insert(e, T::default()).expect(&err_msg);
}

pub fn del_flag<T: Component + Default>(
  world: &mut World, e: Entity, text: &str
) {
  let mut things = world.write_component::<T>();
  let err_msg = format!("Failed Component Insertion: {}", text);
  things.insert(e, T::default()).expect(&err_msg);
}