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
//extern crate image;

//use glutin::Event;

pub mod camera;
pub mod entities;
pub mod input;
pub mod model;
pub mod util;

fn main() {
  use camera::Camera;
  use input::Handler;
  use model::import::load_obj;
  use entities::entity::PosMarker;
  use entities::mobs::Mob;
  //use util::rmatrix::Matrix4f;
  //use util::rvector::Vector3f;
  //use model::mesh::{Mesh, MeshBuffers};
  
  use std::default::Default;
  use glium::{DisplayBuild, Program};
  
  let display = glium::glutin::WindowBuilder::new()
    .with_title(format!("RaumEn Test"))
    .with_dimensions(1024, 760)
    .with_depth_buffer(24)
    .build_glium().unwrap();
  
  use model::import::test_nom;
  test_nom();
  
  let vertex_shader_src = r#"
#version 400
in vec3 position;
in vec3 normal;

out vec3 surface_normal;
out vec3 v_position;
out vec3 toLightVector;
out vec3 toCameraVector;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 u_light;

void main() {
    v_position = gl_Position.xyz / gl_Position.w;
    vec4 worldPos = transform * vec4(position, 1.0);
    vec4 posRelToCam = view * worldPos;
    gl_Position = projection * posRelToCam;
    
    surface_normal = (transform * vec4(normal, 0.0)).xyz;
    
    toLightVector = u_light - worldPos.xyz;
    toCameraVector = (inverse(view) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
}
"#;
  let fragment_shader_src = r#"
#version 400
in vec3 surface_normal;
in vec3 v_position;
in vec3 toLightVector;
in vec3 toCameraVector;

out vec4 color;

const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
  vec3 lightColour = vec3(1.0);
  vec3 unitNormal = normalize(surface_normal);
  vec3 unitCameraVector = normalize(toCameraVector);
  vec3 unitLightVector = normalize(toLightVector);
  
  float diffuse = max(dot(unitNormal, unitLightVector), 0.0);
  float specular = max(dot(reflect(unitLightVector, unitNormal), unitCameraVector), 0.0);
  
  color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}
"#;
  
  let mesh = load_obj("dragon").unwrap().create_buffers(&display);
  let vb = mesh.verts;
  let ib = mesh.indcs;
  let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
  
  let mut cam = Camera::create();
  let mut focus = Mob::new();
  let mut pmrkr = PosMarker::new();
  let mut handler = Handler::new();
  
  loop {
    use glium::Surface;
    
    let mut target = display.draw();
    target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);
    
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
    pmrkr.inc_yrot(0.01_f32);
    focus.move_mob(&mut handler);
    
    // Uniforms
    let transform = pmrkr.transformation();
    let view = { cam.calc_pos(&focus.marker); cam.view_matrix() }; //view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);
    let projection = gen_projection(&target);
    let light = [0.0, 1000.0, -7000.0_f32];
    
    // Draw stuff!
    target.draw(&vb,
                &ib,
                &program,
                &uniform! { transform: transform, view: view, projection: projection, u_light: light },
                &params).unwrap();
    
    target.finish().unwrap();
    // listing the events produced by the window and waiting to be received
    for event in display.poll_events() {
      match event {
        glium::glutin::Event::Closed => return,   // the window has been closed by the user
        ev => handler.event(&ev)
      }
    }
  }
}

fn gen_projection(target: &glium::Frame) -> [[f32; 4]; 4] {
  use glium::Surface;
  use util::rmatrix::Matrix4f;
  let (width, height) = target.get_dimensions();
  let aspect_ratio = height as f32 / width as f32;
  
  let fov: f32 = 3.141592 / 3.0;
  let zfar = 1024.0;
  let znear = 0.1;
  let mut pMatrix = Matrix4f::new();
  let yScale = 1_f32 / (fov / 2_f32).tan();
  let frustumLength = zfar - znear;
  pMatrix.m00 = yScale / aspect_ratio;
  pMatrix.m11 = yScale;
  pMatrix.m22 = -((zfar + znear) / frustumLength);
  pMatrix.m23 = -1_f32;
  pMatrix.m32 = -(2_f32 * znear * zfar) / frustumLength;
  pMatrix.m33 = 0_f32;
  pMatrix.as_slice()
}
