#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate nom;

pub mod util;
use util::rvector::Vector3f;

pub mod model;
use model::import::loadObj;
use model::import::test_nom;
use model::mesh::{Mesh, MeshBuffers};

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
  use glium::{DisplayBuild, Surface};
  let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
  
  test_nom();
  
  let vertex_shader_src = r#"
    #version 140
    in vec3 position;
    in vec3 normal;
    uniform mat4 matrix;
    void main() {
      gl_Position = matrix * vec4(position, 1.0);
    }
  "#;
  let fragment_shader_src = r#"
    #version 140
    
    out vec4 color;
    
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
  "#;
  
  let vertex1 = Vertex { position: [-0.5, -0.5] };
  let vertex2 = Vertex { position: [ 0.0,  0.5] };
  let vertex3 = Vertex { position: [ 0.5, -0.25] };
  let shape = vec![vertex1, vertex2, vertex3];
  
  let test_mesh = loadObj("lamp").unwrap().create_buffers(&display);
  
  let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
  
  let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
  
  loop {
    let matrix = [
      [0.05, 0.0, 0.0, 0.0],
      [0.0, 0.05, 0.0, 0.0],
      [0.0, 0.0, 0.05, 0.0],
      [0.0, 0.0, 0.0, 1.0f32]
    ];
    
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    
    // Draw stuff!
    target.draw(&test_mesh.verts,
                &test_mesh.indcs,
                &program,
                &uniform! { matrix: matrix },
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
