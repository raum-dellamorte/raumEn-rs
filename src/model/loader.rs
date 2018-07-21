#[allow(non_snake_case)]
#[allow(unused_imports)]

use gl::*;
use gl::types::{GLfloat, GLuint, GLsizeiptr, }; // GLenum, GLint, GLchar, GLboolean, 
use std::mem;
use std::ptr;
use model::model::RawModel;
use model::mesh::Mesh;
use util::rvertex::{RVertex, RVertex2D};

pub struct Loader {
  vaos: Vec<GLuint>,
  vbos: Vec<GLuint>,
  textures: Vec<GLuint>,
}

impl Loader {
  pub fn new() -> Self {
    Loader {
      vaos: Vec::new(),
      vbos: Vec::new(),
      textures: Vec::new(),
    }
  }
  pub fn create_vao(&mut self) -> GLuint { unsafe {
    let mut vao_id: GLuint = 0;
    GenVertexArrays(1, &mut vao_id);
    assert!(vao_id != 0);
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
    let mut vbo_id: GLuint = 0;
    GenBuffers(1, &mut vbo_id);
    assert!(vbo_id != 0);
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
  }}
  pub fn bind_tex_coords(&mut self, attrib: u32, verts: &[RVertex]) { unsafe {
    let mut vbo_id: GLuint = 0;
    GenBuffers(1, &mut vbo_id);
    assert!(vbo_id != 0);
    self.vbos.push(vbo_id);
    BindBuffer(ARRAY_BUFFER, vbo_id);
    use std::mem;
    let _verts = verts_tex_coords_to_glfloats(verts);
    BufferData(ARRAY_BUFFER,
      (_verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
      mem::transmute(&_verts[0]),
      STATIC_DRAW);
    VertexAttribPointer(attrib, 2, FLOAT, FALSE, 0, ptr::null());
    BindBuffer(ARRAY_BUFFER, 0_u32);
  }}
  pub fn unbind_vao(&self) { unsafe {
    BindVertexArray(0_u32);
  }}
  pub fn load_texture(&mut self, tex_name: &str) -> GLuint {
    use image;
    use std::path::Path;
    let path: &str = &format!("res/img/{}.png", tex_name);
    let img = match image::open(&Path::new(path)) {
      Ok(image) => {
        println!("Image loaded");
        image.to_rgba()
      },
      _ => panic!("Failed to load image")
    };
    let (width, height) = img.dimensions();
    let img_raw = img.into_raw();
    let mut tex_id: GLuint = 0;
    unsafe {
      GenTextures(1, &mut tex_id);
      BindTexture(TEXTURE_2D, tex_id);
      TexImage2D(
        TEXTURE_2D, 0, RGBA as i32, width as i32, height as i32, 0, RGBA, UNSIGNED_BYTE, 
        mem::transmute(&img_raw[0])
      );
      TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
      TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
      BindTexture(TEXTURE_2D, 0);
    }
    self.textures.push(tex_id);
    tex_id
  }
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
    self.bind_tex_coords(1, &mesh.verts);
    self.unbind_vao();
    RawModel::new(vao_id, mesh.indcs.len() as i32)
  }
  pub fn clean_up(&mut self) { unsafe {
    for vao in &self.vaos {
      DeleteVertexArrays(1_i32, vao);
    }
    for vbo in &self.vbos {
      DeleteVertexArrays(1_i32, vbo);
    }
    for tex in &self.textures {
      DeleteTextures(1_i32, tex);
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
