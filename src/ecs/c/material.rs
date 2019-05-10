#![allow(dead_code)]

use {
  std::cmp::Ordering,
  // ecs::c::{
  //   Textures, Lightings, 
  // },
  util::{
    // rgl::*, 
    Vector2f, 
    // HashMap, 
  },
  specs::*,
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TextureComponent(pub String);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct LightingComponent(pub String);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TexIndexComponent(pub u32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RowCountComponent(pub u32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct OffsetComponent(pub Vector2f);

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct MultiTexComponent;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelComponent(pub String);
impl PartialEq for ModelComponent {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
impl Eq for ModelComponent {}
impl Ord for ModelComponent {
  fn cmp(&self, other: &Self) -> Ordering {
    if self == other { return Ordering::Equal }
    let mut a: Vec<char> = self.0.chars().collect();
    let mut b: Vec<char> = other.0.chars().collect();
    while a.len() > 0 && b.len() > 0 {
      match (a.pop(), b.pop()) {
        (ac,bc) if bc < ac => { return Ordering::Less } 
        (ac,bc) if bc > ac => { return Ordering::Greater } 
        _ => {}
      };
    };
    return if a.len() == 0 { Ordering::Greater } else { Ordering::Less }
  }
}
impl PartialOrd for ModelComponent {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other { return Some(Ordering::Equal) }
    return if self < other { Some(Ordering::Less) } else { Some(Ordering::Greater) }
  }
}

// !ECS

pub struct Material {
  pub name: String,
  pub texture: String,
  pub lighting: String,
  pub tex_index: u32,
  pub row_count: u32,
  pub offset: Vector2f,
  pub multi_tex: bool,
}
impl Material {
  pub fn new(name: &str, texture: &str, lighting: &str) -> Self {
    Self::new_with_tex_atlas(name, texture, lighting, 1, 0)
  }
  pub fn new_with_tex_atlas(name: &str, texture: &str, lighting: &str, row_count: u32, tex_index: u32) -> Self {
    let mut out = Material {
      name: name.to_string(),
      texture: texture.to_string(),
      lighting: lighting.to_string(),
      tex_index: tex_index,
      row_count: row_count,
      offset: Vector2f::blank(),
      multi_tex: false
    };
    out.calc_offset();
    out
  }
  pub fn x_offset(&self) -> f32 {
    let col: u32 = self.tex_index % self.row_count;
    col as f32 / self.row_count as f32
  }
  pub fn y_offset(&self) -> f32 {
    let row: u32 = self.tex_index / self.row_count;
    row as f32 / self.row_count as f32
  }
  pub fn calc_offset(&mut self) {
    self.offset.x = self.x_offset();
    self.offset.y = self.y_offset();
  }
  pub fn set_tex_index(&mut self, index: u32) {
    self.tex_index = index;
    self.calc_offset();
  }
}
