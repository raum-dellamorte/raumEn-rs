
extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;

use glutin::dpi::*;
use glutin::GlContext;

// in project stuff
pub mod model;
pub mod util;
pub mod render;

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
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  
  use model::loader::Loader;
  let mut loader = Loader::new();
  
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

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    gl_window.swap_buffers().unwrap();
  }
}
