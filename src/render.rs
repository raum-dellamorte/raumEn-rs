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
  use shader::Shader;
  use util::rvertex::{RVertex, RVertex2D};
  
  pub fn prepare() { unsafe {
    Enable(DEPTH_TEST);
    Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }}
  pub fn render(shader: &Shader, camera: &mut Camera, entity: &mut Entity) { unsafe {
    camera.create_view_matrix();
    let view_mat = camera.view_mat.as_slice();
    shader.start();
    BindVertexArray(entity.model.raw().vao_id);
    let mut count: GLuint = 0;
    while count < shader.vars.len() as GLuint {
      EnableVertexAttribArray(count);
      count += 1 as GLuint;
    }
    ActiveTexture(TEXTURE0);
    BindTexture(TEXTURE_2D, entity.model.texture);
    let trans_mat = entity.marker.transformation();
    shader.load_matrix("u_Transform", &trans_mat);
    shader.load_matrix("u_View", &view_mat);
    DrawElements(TRIANGLES, entity.model.raw().vertex_count, UNSIGNED_INT, CVOID);
    while count > 0 as GLuint {
      count -= 1 as GLuint;
      DisableVertexAttribArray(count);
    }
    BindVertexArray(0);
    shader.stop();
  }}
}
