#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![feature(attr_literals)]
#![feature(custom_derive)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;
extern crate time;

//use glutin::Event;

pub mod camera;
pub mod entities;
pub mod fbo;
pub mod input;
pub mod model;
//pub mod shaders;
pub mod timer;
pub mod util;

use camera::Camera;
use entities::entity::Entity;
use entities::mobs::Mob;
use fbo::FboWithDepth;
use input::Handler;
//use model::import::load_obj;
//use model::mesh::{Mesh, MeshBuffers};
//use util::rmatrix::Matrix4f;
//use util::rvector::Vector3f;
use timer::Timer;

use glium::DisplayBuild;

fn main() {
  let mut cam = Camera::create(
    glium::glutin::WindowBuilder::new()
      .with_title(format!("RaumEn Test"))
      .with_dimensions(1024, 760)
      .with_depth_buffer(24)
      .build_glium().unwrap()
  );
  
  //use model::import::test_nom;
  //test_nom();
  
  let mut timer = Timer::new();
  
  let mut entity = Entity::new("spaceship");
  entity.load_model_defaults(&cam.display);
  let mut focus = Mob::new("spaceship");
  focus.entity.load_model_defaults(&cam.display).marker.pos.from_isize(0, 0, -20);
  let mut handler = Handler::new();
  
  let fbuffer = FboWithDepth::new_default(&cam);
  
  loop {
    use glium::Surface;
    
    timer.tick();
    cam.load_target();
    
    // Calc Movement
    entity.marker.inc_yrot(0.01_f32);
    //focus.entity.marker.inc_yrot(-0.01_f32);
    focus.move_mob(&mut handler, timer.delta);
    cam.calc_pos(&focus.entity.marker);
    
    // Draw!
    let mut fbo = fbuffer.fb(&cam);
    cam.draw_entity_surface(&mut entity, &mut fbo);
    cam.draw_entity_surface(&mut focus.entity, &mut fbo);
    {cam.target.as_mut().unwrap().clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);}
    cam.draw_entity(&mut entity);
    cam.draw_entity(&mut focus.entity);
    
    
    // Finish!
    cam.finish();
    
    // listing the events produced by the window and waiting to be received
    for event in cam.display.poll_events() {
      match event {
        glium::glutin::Event::Closed => return,   // the window has been closed by the user
        ev => handler.event(&ev)
      }
    }
  }
}
