#[allow(unused_imports)]

extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;
extern crate cgmath;
extern crate time;

use gl::*;
use std::os::raw::c_void;
use glutin::dpi::*;
use glutin::GlContext;
// use cgmath::{Matrix4, Point3, Vector3}; // Deg, 

const CVOID: *const c_void = 0 as *const c_void;

// in project stuff
pub mod camera;
pub mod entities;
pub mod gamemgr;
pub mod input;
pub mod material;
pub mod model;
pub mod render;
pub mod shader;
pub mod terrain;
pub mod texture;
pub mod timer;
pub mod util;

pub use camera::Camera;
pub use entities::Entity;
pub use entities::mobs::Mob;
pub use input::Handler;
pub use model::loader::Loader;
pub use model::Model;
pub use render::{RenderMgr, };
pub use shader::lighting::Lights;
pub use shader::Shader;
pub use terrain::World;
pub use timer::Timer;

fn main() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_title("RaumEn")
    .with_dimensions(LogicalSize::new(640.0, 480.0));
  let context = glutin::ContextBuilder::new()
    .with_vsync(true);
  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
  
  unsafe {
    gl_window.make_current().unwrap();
  }
  
  unsafe {
    load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  
  let mut render_mgr = RenderMgr::new();
  // let mut game_mgr = render_mgr.mgr.clone();
  let ents_arc = render_mgr.mgr.entities.clone();
  
  let mut spaceship = {
    let mut ents = ents_arc.lock().unwrap();
    ents.new_model("spaceship", "").new_entities(&vec!["01", "02", "03"]);
    ents.new_model("platform", "dirt").new_entities(&vec!["01", "02", "03"]);
    println!("entities loaded");
    ents.mod_entity("spaceship", "02", |ent| { ent.set_pos(10.0,0.0,-10.0); });
    ents.mod_entity("spaceship", "03", |ent| { ent.set_pos(-12.0,5.0,-15.0); });
    ents.mod_entity("platform", "01", |ent| { ent.set_pos(0.0,0.0,0.0); });
    ents.mod_entity("platform", "02", |ent| { ent.set_pos(2.0,0.0,0.0); });
    ents.mod_entity("platform", "03", |ent| { ent.set_pos(0.0,0.0,2.0); });
    let spaceship_arc = ents.get_entity("spaceship", "01");
    let mut spaceship = spaceship_arc.lock().unwrap();
    spaceship.create_mob()
  };
  
  println!("Starting game loop.");
  let mut running = true;
  {
    let dpi = gl_window.get_hidpi_factor();
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    render_mgr.load_proj_mat(size);
  }
  while running {
    { let mut handler = render_mgr.mgr.handler.lock().unwrap();
      handler.timer.tick();
      handler.reset_delta(); }
    events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
          glutin::WindowEvent::CloseRequested => running = false,
          glutin::WindowEvent::Resized(logical_size) => {
            let dpi = gl_window.get_hidpi_factor();
            let size = logical_size.to_physical(dpi);
            gl_window.resize(size);
            render_mgr.load_proj_mat(size);
          },
          _ => {
            let mut handler = render_mgr.mgr.handler.lock().unwrap();
            handler.window_event(&event);
          }
        },
        glutin::Event::DeviceEvent{ event, ..} => {
          let mut handler = render_mgr.mgr.handler.lock().unwrap();
          handler.device_event(&event);
        }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    { let mut handler = render_mgr.mgr.handler.lock().unwrap();
      spaceship.move_mob(&mut handler); }
    { let mut camera = render_mgr.mgr.camera.lock().unwrap();
      camera.calc_pos(spaceship.pos.clone()); }
    render_mgr.render();
    
    gl_window.swap_buffers().unwrap();
  }
  render_mgr.clean_up();
}

// fn default_view() -> Matrix4<f32> {
//   Matrix4::look_at(
//     Point3::new(0f32, 0.0, 0.0),
//     Point3::new(0.0f32, -5.0, 1.0),
//     Vector3::unit_z(),
//   )
// }
