#[allow(non_snake_case)]
#[allow(unused_imports)]

use gl::*;
use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr, GLboolean};
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use model::model::RawModel;
use model::mesh::Mesh;
use util::rvertex::{RVertex, RVertex2D};

pub struct Loader {
  vaos: Vec<GLuint>,
  vbos: Vec<GLuint>,
}

impl Loader {
  pub fn new() -> Self {
    Loader {
      vaos: Vec::new(),
      vbos: Vec::new(),
    }
  }
  pub fn create_vao(&mut self) -> GLuint { unsafe {
    let mut vao_id = 0 as GLuint;
    GenVertexArrays(1, &mut vao_id);
    self.vaos.push(vao_id);
    BindVertexArray(vao_id);
    vao_id
  }}
  pub fn bind_vertices_2d(&mut self, attrib: u32, verts: &[RVertex2D]) { unsafe {
    let mut vbo_id = 0 as GLuint;
    GenBuffers(1, &mut vbo_id);
    self.vbos.push(vbo_id);
    BindBuffer(ARRAY_BUFFER, vbo_id);
    let _verts = verts_pos_to_glfloats_2d(verts);
    BufferData(ARRAY_BUFFER,
      (_verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
      mem::transmute(&_verts[0]),
      STATIC_DRAW);
    VertexAttribPointer(attrib, 2, FLOAT, FALSE, 0, ptr::null());
    BindBuffer(ARRAY_BUFFER, 0_u32);
  }}
  pub fn bind_vertices(&mut self, attrib: u32, verts: &[RVertex]) { unsafe {
    let mut vbo_id = 0_u32;
    GenBuffers(1, &mut vbo_id);
    self.vbos.push(vbo_id);
    BindBuffer(ARRAY_BUFFER, vbo_id);
    use std::mem;
    let _verts = verts_pos_to_glfloats(verts);
    BufferData(ARRAY_BUFFER,
      (_verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
      mem::transmute(&_verts[0]),
      STATIC_DRAW);
    VertexAttribPointer(attrib, 3, FLOAT, FALSE, 0, ptr::null());
    BindBuffer(ARRAY_BUFFER, 0_u32);
  }}
  pub fn bind_indices(&mut self, idxs: &[u16]) { unsafe {
    let mut vbo_id = 0_u32;
    GenBuffers(1, &mut vbo_id);
    self.vbos.push(vbo_id);
    BindBuffer(ELEMENT_ARRAY_BUFFER, vbo_id);
    use std::mem;
    let _idxs = indices_to_gluints(idxs);
    BufferData(ELEMENT_ARRAY_BUFFER,
      (_idxs.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
      mem::transmute(&_idxs[0]),
      STATIC_DRAW);
    BindBuffer(ELEMENT_ARRAY_BUFFER, 0_u32);
  }}
  pub fn unbind_vao(&self) { unsafe {
    BindVertexArray(0_u32);
  }}
  pub fn load_to_vao_2d(&mut self, verts: &[RVertex2D]) -> RawModel {
    let vao_id = self.create_vao();
    self.bind_vertices_2d(0, verts);
    self.unbind_vao();
    RawModel::new(vao_id, verts.len() as i32)
  }
  pub fn load_to_vao(&mut self, mesh: &Mesh) -> RawModel {
    let vao_id = self.create_vao();
    self.bind_indices(&mesh.indcs);
    self.bind_vertices(0, &mesh.verts);
    self.unbind_vao();
    RawModel::new(vao_id, mesh.verts.len() as i32)
  }
  pub fn clean_up(&mut self) { unsafe {
    for vao in &self.vaos {
      DeleteVertexArrays(1_i32, vao);
    }
    for vbo in &self.vbos {
      DeleteVertexArrays(1_i32, vbo);
    }
  }}
}

pub fn verts_pos_to_glfloats_2d(verts: &[RVertex2D]) -> Vec<GLfloat> {
  let mut out = Vec::new();
  for vert in verts {
    out.push(vert.position[0] as GLfloat);
    out.push(vert.position[1] as GLfloat);
  }
  out
}
pub fn verts_pos_to_glfloats(verts: &[RVertex]) -> Vec<GLfloat> {
  let mut out = Vec::new();
  for vert in verts {
    out.push(vert.position[0] as GLfloat);
    out.push(vert.position[1] as GLfloat);
    out.push(vert.position[2] as GLfloat);
  }
  out
}
pub fn verts_norms_to_glfloats(verts: &[RVertex]) -> Vec<GLfloat> {
  let mut out = Vec::new();
  for vert in verts {
    out.push(vert.normal[0] as GLfloat);
    out.push(vert.normal[1] as GLfloat);
    out.push(vert.normal[2] as GLfloat);
  }
  out
}
pub fn verts_tex_coords_to_glfloats(verts: &[RVertex]) -> Vec<GLfloat> {
  let mut out = Vec::new();
  for vert in verts {
    out.push(vert.tex_coords[0] as GLfloat);
    out.push(vert.tex_coords[1] as GLfloat);
  }
  out
}
pub fn verts_tex_coords_to_glfloats_2d(verts: &[RVertex2D]) -> Vec<GLfloat> {
  let mut out = Vec::new();
  for vert in verts {
    out.push(vert.tex_coords[0] as GLfloat);
    out.push(vert.tex_coords[1] as GLfloat);
  }
  out
}
pub fn indices_to_gluints(idxs: &[u16]) -> Vec<GLuint> {
  let mut out = Vec::new();
  for idx in idxs {
    out.push(*idx as GLuint);
  }
  out
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
  let shader;
  unsafe {
    shader = CreateShader(ty);
    // Attempt to compile the shader
    let c_str = CString::new(src).unwrap();
    ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    CompileShader(shader);
    
    // Get the compile status
    let mut status = FALSE as GLint;
    GetShaderiv(shader, COMPILE_STATUS, &mut status);
    
    // Fail on error
    if status != (TRUE as GLint) {
      let mut len = 0;
      GetShaderiv(shader, INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::new();
      buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
      GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
      panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
    }
  }
  shader
}
pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
  let program = CreateProgram();
  AttachShader(program, vs);
  AttachShader(program, fs);
  LinkProgram(program);
  // Get the link status
  let mut status = FALSE as GLint;
  GetProgramiv(program, LINK_STATUS, &mut status);
  
  // Fail on error
  if status != (TRUE as GLint) {
    let mut len: GLint = 0;
    GetProgramiv(program, INFO_LOG_LENGTH, &mut len);
    let mut buf = Vec::new();
    buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
    GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
    panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8"));
  }
  program
}}
