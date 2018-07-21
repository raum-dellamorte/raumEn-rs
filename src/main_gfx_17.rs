

extern crate cgmath;
#[macro_use] extern crate gfx;
extern crate gfx_app;
#[macro_use] extern crate nom;
extern crate image;

use cgmath::{Deg, Matrix4, Point3, Vector3};
use gfx::{Bundle, }; // , texture

pub mod model;
pub mod pipeline;
pub mod util;

pub use pipeline::*;

struct App<R: gfx::Resources>{
  bundle: Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
  fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend, window_targets: gfx_app::WindowTargets<R>) -> Self {
    use gfx::traits::FactoryExt;

    let vs = gfx_app::shade::Source {
      glsl_150: include_bytes!("res/glsl/model.glslv"),
      .. gfx_app::shade::Source::empty()
    };
    let ps = gfx_app::shade::Source {
      glsl_150: include_bytes!("res/glsl/model.glslf"),
      .. gfx_app::shade::Source::empty()
    };
    
    use model::import::load_obj;
    let spaceship_mesh = load_obj("spaceship").unwrap();
    let spaceship_verts = pipeline::convertices(&spaceship_mesh.verts);
    
    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&spaceship_verts, &spaceship_mesh.indcs[0..]);
    
    let spaceship_texture = match load_texture(factory, "spaceship") {
      Ok(tex) => {
        println!("Texture loaded.");
        tex
      }
      _ => panic!("Failed to load texture")
    };
    
    println!("Creating Pipeline State.");
    let _vs = match vs.select(backend) {
      Ok(_vs) => { println!("vs.select(backend) succeeded."); _vs }
      _ => panic!("vs.select(backend) failed.")
    };
    let _ps = match ps.select(backend) {
      Ok(_ps) => { println!("ps.select(backend) succeeded."); _ps }
      _ => panic!("ps.select(backend) failed.")
    };
    let pso = match factory.create_pipeline_simple(
      &_vs,
      &_ps,
      pipe::new()
    ) {
      Ok(pso) => {
        println!("Pipeline State created.");
        pso
      }
      _ => panic!("Failed to create Pipeline State.")
    };
    
    let proj = cgmath::perspective(Deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);
    
    let data = pipe::Data {
      vbuf: vbuf,
      tex: (spaceship_texture, factory.create_sampler_linear()),
      transform: (proj * default_view()).into(),
      locals: factory.create_constant_buffer(1),
      out_color: window_targets.color,
      out_depth: window_targets.depth,
    };
    
    App {
      bundle: Bundle::new(slice, pso, data),
    }
  }

  fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
    println!("Render Pass");
    let locals = Locals { transform: self.bundle.data.transform };
    encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
    encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
    encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
    self.bundle.encode(encoder);
  }

  fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
    self.bundle.data.out_color = window_targets.color;
    self.bundle.data.out_depth = window_targets.depth;
    
    // In this example the transform is static except for window resizes.
    let proj = cgmath::perspective(Deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);
    self.bundle.data.transform = (proj * default_view()).into();
  }
}

pub fn main() {
  use gfx_app::Application;
  App::launch_simple("RaumEn");
}

fn default_view() -> Matrix4<f32> {
  Matrix4::look_at(
    Point3::new(1.5f32, -5.0, 3.0),
    Point3::new(0f32, 0.0, 0.0),
    Vector3::unit_z(),
  )
}
