#![allow(non_snake_case)]
#![allow(dead_code)]
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
pub mod input;
pub mod model;
pub mod timer;
pub mod util;

fn main() {
  use camera::Camera;
  use entities::entity::Entity;
  use entities::mobs::Mob;
  use input::Handler;
  //use model::import::load_obj;
  //use model::mesh::{Mesh, MeshBuffers};
  //use util::rmatrix::Matrix4f;
  //use util::rvector::Vector3f;
  use timer::Timer;
  
  use std::default::Default;
  use glium::DisplayBuild;
  
  let mut camera = Camera::create(
    glium::glutin::WindowBuilder::new()
      .with_title(format!("RaumEn Test"))
      .with_dimensions(1024, 760)
      .with_depth_buffer(24)
      .build_glium().unwrap()
  );
  let mut cam = &mut camera;
  
  //use model::import::test_nom;
  //test_nom();
  
  let mut timer = Timer::new();
  
  let mut entity = Entity::new("spaceship");
  entity.load_model_defaults(&cam.display);
  let mut focus = Mob::new("spaceship");
  focus.entity.load_model_defaults(&cam.display).marker.pos.from_isize(0, 0, -20);
  let mut handler = Handler::new();
  
  loop {
    use glium::Surface;
    
    timer.tick();
    
    cam.update();
    {cam.target.as_mut().unwrap().clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);}
    
    let params = glium::DrawParameters {
      depth: glium::Depth {
        test: glium::DepthTest::IfLess,
        write: true,
        .. Default::default()
      },
      backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
      .. Default::default()
    };
    
    // Calc Movement
    entity.marker.inc_yrot(0.01_f32);
    //focus.entity.marker.inc_yrot(-0.01_f32);
    focus.move_mob(&mut handler, timer.delta);
    cam.calc_pos(&focus.entity.marker);
    
    // Draw!
    cam.draw_entity(&mut entity, &params);
    cam.draw_entity(&mut focus.entity, &params);
    
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
