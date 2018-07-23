#[allow(unused_imports)]

extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;
extern crate cgmath;


use gl::*;
use std::os::raw::c_void;
use glutin::dpi::*;
use glutin::GlContext;
// use cgmath::{Matrix4, Point3, Vector3}; // Deg, 

const CVOID: *const c_void = 0 as *const c_void;

// in project stuff
pub mod camera;
pub mod entities;
pub mod input;
pub mod model;
pub mod render;
pub mod shader;
pub mod util;

pub use camera::Camera;
pub use entities::Entity;
pub use entities::mobs::Mob;
pub use input::Handler;
pub use model::loader::Loader;
pub use model::Model;
pub use shader::Shader;
pub use render::ModelRender;

fn main() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_title("RaumEn")
    .with_dimensions(LogicalSize::new(1024.0, 768.0));
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
  let mut camera = Camera::new();
  let mut handler = Handler::new();
  println!("Creating loader");
  let mut loader = Loader::new();
  println!("loader ready. getting model.");
  let mut spaceship = Mob::new("spaceship");
  spaceship.init(&mut loader);
  println!("loading shader program.");
  let mut shader = shader::model::gen_model_shader();
  let mut running = true;
  let mut proj_mat = [0_f32; 16];
  while running {
    events_loop.poll_events(|event| {
    match event {
      glutin::Event::WindowEvent{ event, .. } => match event {
        glutin::WindowEvent::CloseRequested => running = false,
        glutin::WindowEvent::Resized(logical_size) => {
          let dpi_factor = gl_window.get_hidpi_factor();
          let size = logical_size.to_physical(dpi_factor);
          gl_window.resize(size);
          camera.update_size(size.into());
          proj_mat = camera.projection();
          shader.start();
          shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
          shader.stop();
        },
        _ => { handler.window_event(&event) }
      },
      glutin::Event::DeviceEvent{ event, ..} => { handler.device_event(&event); }
      e => println!("Other Event:\n{:?}", e)
    }
    });
    ModelRender::prepare(); // Clear color
    spaceship.move_mob(&mut handler, 0.01);
    ModelRender::render(&shader, &mut camera, &mut spaceship.entity);
    
    gl_window.swap_buffers().unwrap();
  }
  shader.clean_up();
  loader.clean_up();
}

// fn default_view() -> Matrix4<f32> {
//   Matrix4::look_at(
//     Point3::new(0f32, 0.0, 0.0),
//     Point3::new(0.0f32, -5.0, 1.0),
//     Vector3::unit_z(),
//   )
// }
