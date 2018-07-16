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
  use model::model::RawModel;
  use util::rvertex::{RVertex, RVertex2D};
  
  pub fn prepare() { unsafe {
    Clear(COLOR_BUFFER_BIT);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }}
  pub fn render(model: RawModel) { unsafe {
    BindVertexArray(model.vao_id);
    EnableVertexAttribArray(0);
    DrawArrays(TRIANGLES, 0, model.vertex_count);
    DisableVertexAttribArray(0);
    BindVertexArray(0);
  }}
}
