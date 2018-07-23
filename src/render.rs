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
  use entities::Entity;
  use shader::Shader;
  use util::rvertex::{RVertex, RVertex2D};
  
  pub fn prepare() { unsafe {
    Clear(COLOR_BUFFER_BIT);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }}
  pub fn render(shader: &Shader, entity: &mut Entity) { unsafe {
    shader.start();
    BindVertexArray(entity.model.raw().vao_id);
    let mut count: GLuint = 0;
    while count < shader.vars.len() as GLuint {
      EnableVertexAttribArray(count);
      count += 1 as GLuint;
    }
    ActiveTexture(TEXTURE0);
    BindTexture(TEXTURE_2D, entity.model.texture);
    let tmp_mat = entity.marker.transformation();
    shader.load_matrix("u_Transform", &tmp_mat);
    DrawElements(TRIANGLES, entity.model.raw().vertex_count, UNSIGNED_INT, CVOID);
    while count > 0 as GLuint {
      count -= 1 as GLuint;
      DisableVertexAttribArray(count);
    }
    BindVertexArray(0);
    shader.stop();
  }}
}
