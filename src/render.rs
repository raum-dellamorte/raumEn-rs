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
  use model::model::RawModel;
  use shader::Shader;
  use util::rvertex::{RVertex, RVertex2D};
  
  pub fn prepare() { unsafe {
    Clear(COLOR_BUFFER_BIT);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }}
  pub fn render(shader: &Shader, model: &RawModel) { unsafe {
    shader.start();
    BindVertexArray(model.vao_id);
    let mut count: GLuint = 0;
    while count < shader.vars.len() as GLuint {
      EnableVertexAttribArray(count);
      count += 1 as GLuint;
    }
    DrawElements(TRIANGLES, model.vertex_count, UNSIGNED_INT, CVOID);
    while count > 0 as GLuint {
      count -= 1 as GLuint;
      DisableVertexAttribArray(count);
    }
    BindVertexArray(0);
    shader.stop()
  }}
}
