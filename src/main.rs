#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate nom;

pub mod util;
//use util::rvector::Vector3f;

pub mod model;
use model::import::load_obj;
//use model::import::test_nom;
//use model::mesh::{Mesh, MeshBuffers};

#[macro_use]
extern crate glium;

fn main() {
  use std::default::Default;
  use glium::{DisplayBuild, Program};
  let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    //.with_dimensions(1024, 760)
    //.with_title(format!("RaumEn Test"))
    //.with_depth_buffer(32)
    //.build_glium()
    //.unwrap();
  //.build_glium().unwrap();
  
  //test_nom();
  
  let vertex_shader_src = r#"
    #version 140
    in vec3 position;
    in vec3 normal;
    uniform mat4 matrix;
    out vec3 v_normal;
    void main() {
      v_normal = transpose(inverse(mat3(matrix))) * normal;
      gl_Position = matrix * vec4(position, 1.0);
    }
  "#;
  let fragment_shader_src = r#"
    #version 140
    
    in vec3 v_normal;
    uniform vec3 u_light;
    out vec4 color;
    
    void main() {
      float brightness = dot(normalize(v_normal), normalize(u_light));
      vec3 dark_color = vec3(0.6, 0.0, 0.0);
      vec3 regular_color = vec3(1.0, 0.0, 0.0);
      color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
  "#;
  
  let test_mesh = load_obj("dragon").unwrap().create_buffers(&display);
  let vb = test_mesh.verts;
  let ib = test_mesh.indcs;
  
  let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
  
  loop {
    use glium::Surface;
    let matrix = [
      [0.05, 0.0, 0.0, 0.0],
      [0.0, 0.05, 0.0, 0.0],
      [0.0, 0.0, 0.05, 0.0],
      [0.0, 0.0, 0.0, 1.0f32]
    ];
    let light = [-1.0, 0.4, 0.9f32];
    
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    
    // Draw stuff!
    target.draw(&vb,
                &ib,
                &program,
                &uniform! { matrix: matrix, u_light: light },
                &Default::default()).unwrap();
    
    target.finish().unwrap();
    // listing the events produced by the window and waiting to be received
    for ev in display.poll_events() {
      match ev {
        glium::glutin::Event::Closed => return,   // the window has been closed by the user
        _ => ()
      }
    }
  }
}
