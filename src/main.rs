#![recursion_limit="128"]
#![allow(unused_imports)]
#![allow(dead_code)]

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
pub mod display;
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
pub use display::Display;
pub use entities::Entity;
pub use entities::mobs::Mob;
pub use gamemgr::GameMgr;
pub use input::Handler;
pub use loader::Loader;
pub use material::Material;
pub use render::{RenderMgr, };
pub use shader::lighting::Lights;
pub use shader::Shader;
pub use terrain::World;
pub use timer::Timer;

fn main() {
  // Test code for parsing fnt files
  // use text::metafile::test_noms;
  // test_noms();
  
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
  let mut mgr = render_mgr.mgr.clone();
  let mut spaceship = {
    mgr.new_model("spaceship");
    mgr.new_material("spaceship", "spaceship", "metal");
    mgr.new_entity("spaceship", "spaceship", "spaceship");
    mgr.mod_entity("spaceship", |ships| {
      ships.new_instance().set_pos(0.0,10.0,0.0);
      ships.new_instance().set_pos(10.0,0.0,-10.0);
      ships.new_instance().set_pos(-12.0,5.0,-15.0);
    });
    mgr.new_material("dirt", "dirt", "flat");
    mgr.new_model("platform");
    println!("entities loaded");
    let _arc = mgr.entities.clone();
    let hm = _arc.lock().unwrap();
    hm.get("spaceship").unwrap().first().create_mob("player")
  };
  
  let mut fps: f32;
  let mut sec = 0.0;
  
  {
    let dpi = gl_window.get_hidpi_factor();
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    render_mgr.update_size(size.into());
  }
  {
    let _textmgr = mgr.clone().textmgr.take().unwrap();
    let mut textmgr = _textmgr.lock().unwrap();
    textmgr.add_font(mgr.clone(), "pirate");
    textmgr.add_font(mgr.clone(), "sans");
    textmgr.new_text(mgr.clone(), "Title", "The Never", "pirate", 4.0, 0.0, 0.0, 1.0, true, true);
    textmgr.new_text(mgr.clone(), "FPS", "FPS: 0.0", "sans", 1.5, 0.0, 0.0, 0.3, false, true);
  }
  println!("Starting game loop.");
  let mut running = true;
  while running {
    mgr.handler_do(|handler| {
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
            render_mgr.update_size(size.into());
          },
          _ => { mgr.handler_do(|handler| { handler.window_event(&event); }); }
        },
        glutin::Event::DeviceEvent{ event, ..} => {
          mgr.handler_do(|handler| { handler.device_event(&event); });
        }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    {
      let handler = mgr.handler.lock().unwrap();
      fps = handler.timer.fps;
      sec += handler.timer.delta;
    }
    if sec >= 1.0 {
      sec -= 1.0;
      let _textmgr = mgr.clone().textmgr.take().unwrap();
      let mut textmgr = _textmgr.lock().unwrap();
      textmgr.update_text(mgr.clone(), "FPS", &format!("FPS: {:.3}", (fps * 1000.0).round() / 1000.0 ) );
    }
    
    spaceship.move_mob(mgr.handler.clone(), mgr.world.clone());
    mgr.camera_do(|camera| { camera.calc_pos(spaceship.pos.clone()); });
    render_mgr.render();
    
    gl_window.swap_buffers().unwrap();
  }
  render_mgr.clean_up();
}

pub const EOF: &str = "\04";

pub fn eof(string: &str) -> String {
  [string, EOF].join("")
}
