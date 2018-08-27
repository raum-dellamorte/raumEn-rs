#![recursion_limit="128"]
#[allow(unused_imports)]
#[allow(dead_code)]

extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;
extern crate cgmath;
extern crate time;
extern crate noise;

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
pub mod loader;
pub mod material;
pub mod model;
pub mod render;
pub mod shader;
pub mod terrain;
pub mod text;
pub mod texture;
pub mod timer;
pub mod util;

pub use camera::Camera;
pub use entities::Entity;
pub use entities::mobs::Mob;
pub use input::Handler;
pub use loader::Loader;
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
  let mut game_mgr = render_mgr.mgr.clone();
  
  let mut spaceship = {
    game_mgr.new_model("spaceship");
    game_mgr.new_material("spaceship", "spaceship", "metal");
    game_mgr.new_entity("spaceship", "spaceship", "spaceship");
    game_mgr.mod_entity("spaceship", |ships| {
      ships.new_instance().set_pos(0.0,10.0,0.0);
      ships.new_instance().set_pos(10.0,0.0,-10.0);
      ships.new_instance().set_pos(-12.0,5.0,-15.0);
    });
    game_mgr.new_material("dirt", "dirt", "flat");
    game_mgr.new_model("platform");
    println!("entities loaded");
    let _arc = game_mgr.entities.clone();
    let hm = _arc.lock().unwrap();
    hm.get("spaceship").unwrap().first().create_mob("player")
  };
  
  println!("Starting game loop.");
  let mut running = true;
  {
    let dpi = gl_window.get_hidpi_factor();
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    render_mgr.load_proj_mat(size);
  }
  while running {
    game_mgr.handler_do(|handler| {
      handler.timer.tick();
      handler.reset_delta();
    });
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
          _ => { game_mgr.handler_do(|handler| { handler.window_event(&event); }); }
        },
        glutin::Event::DeviceEvent{ event, ..} => {
          game_mgr.handler_do(|handler| { handler.device_event(&event); });
        }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    spaceship.move_mob(game_mgr.handler.clone(), game_mgr.world.clone());
    game_mgr.camera_do(|camera| { camera.calc_pos(spaceship.pos.clone()); });
    render_mgr.render();
    
    gl_window.swap_buffers().unwrap();
  }
  render_mgr.clean_up();
}

pub const EOF: &str = "\04";

pub fn eof(string: &str) -> String {
  let mut out = string.to_owned();
  out.push_str(EOF);
  out
}
