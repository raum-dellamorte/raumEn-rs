#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod ModelRender {
  use gl::*;
  use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr};
  use gl::types::{GLboolean};
  use std::mem;
  use std::ptr;
  use std::str;
  use std::ffi::CString;
  use CVOID;
  use camera::Camera;
  use entities::Entity;
  use shader::lighting::Light;
  use shader::Shader;
  use util::rvector::{Vector2f, Vector3f, Vector4f};
  use util::rvertex::{RVertex, RVertex2D};
  
  pub fn prepare() { unsafe {
    Enable(DEPTH_TEST);
    Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }}
  pub fn render(shader: &Shader, camera: &mut Camera, light: &Light, entity: &mut Entity) { unsafe {
    camera.create_view_matrix();
    let view_mat = camera.view_mat.as_slice();
    let trans_mat = entity.marker.transformation();
    shader.start();
    BindVertexArray(entity.model.raw().vao_id);
    let mut count: GLuint = 0;
    while count < shader.vars.len() as GLuint {
      EnableVertexAttribArray(count);
      count += 1 as GLuint;
    }
    ActiveTexture(TEXTURE0);
    BindTexture(TEXTURE_2D, entity.model.texture);
    shader.load_matrix("u_Transform", &trans_mat);
    shader.load_matrix("playerLoc", &trans_mat); // mat4 playerLoc
    shader.load_matrix("u_View", &view_mat);
    light.load_to_shader(shader);
    entity.model.lighting().load_to_shader(shader);
    shader.load_float("numOfRows", 1_f32); // float numOfRows
    shader.load_vec_2f("offset", &Vector2f {x: 0_f32, y: 0_f32}); // vec2 offset;
    shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    shader.load_bool("useClipPlane", false); // float useClipPlane;
    DrawElements(TRIANGLES, entity.model.raw().vertex_count, UNSIGNED_INT, CVOID);
    while count > 0 as GLuint {
      count -= 1 as GLuint;
      DisableVertexAttribArray(count);
    }
    BindVertexArray(0);
    shader.stop();
  }}
}
