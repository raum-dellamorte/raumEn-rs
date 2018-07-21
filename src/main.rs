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
use cgmath::{Deg, Matrix4, Point3, Vector3};

const CVOID: *const c_void = 0 as *const c_void;

// in project stuff
pub mod model;
pub mod render;
pub mod shader;
pub mod util;

pub use shader::Shader;

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
  println!("Creating loader");
  use model::loader::Loader;
  let mut loader = Loader::new();
  println!("loader ready. getting model.");
  use model::model::Model;
  let mut spaceship_model = Model::new("spaceship"); // fixme: can't chain this bc lifetimes
  println!("loading the mesh for the model.");
  spaceship_model.load_default_mesh();
  println!("loading mesh to opengl with loader.");
  let spaceship = loader.load_to_vao(&spaceship_model.mesh.unwrap());
  println!("loading shader program.");
  let mut shader = shader::model::gen_model_shader();
  let mut running = true;
  while running {
    events_loop.poll_events(|event| {
    match event {
      glutin::Event::WindowEvent{ event, .. } => match event {
        glutin::WindowEvent::CloseRequested => running = false,
        glutin::WindowEvent::Resized(logical_size) => {
          let dpi_factor = gl_window.get_hidpi_factor();
          gl_window.resize(logical_size.to_physical(dpi_factor));
        },
        _ => ()
      },
      _ => ()
    }
    });
    render::ModelRender::prepare(); // Clear color
    
    render::ModelRender::render(&shader, &spaceship);
    
    gl_window.swap_buffers().unwrap();
  }
  shader.clean_up();
  loader.clean_up();
}

fn default_view() -> Matrix4<f32> {
  Matrix4::look_at(
    Point3::new(1.5f32, -5.0, 3.0),
    Point3::new(0f32, 0.0, 0.0),
    Vector3::unit_z(),
  )
}
