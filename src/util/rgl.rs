
use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use CVOID;

pub fn r_gen_textures() -> u32 { unsafe {
  let mut id = 0;
  GenTextures(1, &mut id);
  id
}}
pub fn r_gen_buffers() -> u32 { unsafe {
  let mut id = 0;
  GenBuffers(1, &mut id);
  id
}}
pub fn r_gen_framebuffers() -> u32 { unsafe {
  let mut id = 0;
  GenFramebuffers(1, &mut id);
  id
}}
pub fn r_gen_renderbuffers() -> u32 { unsafe {
  let mut id = 0;
  GenRenderbuffers(1, &mut id);
  id
}}
pub fn r_gen_vertex_arrays() -> u32 { unsafe {
  let mut id = 0;
  GenVertexArrays(1, &mut id);
  id
}}
