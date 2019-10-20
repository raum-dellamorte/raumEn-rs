
use {
  gl::{
    *,
    types::{
      GLfloat, GLint, GLuint, GLsizeiptr, // GLenum, GLchar, GLboolean, 
    },
  },
  std::{
    mem,
    ptr,
    path::Path,
  },
  image,
  crate::{
    ecs::resource::{ Model, Texture, },
    importobj::{
      load_obj,
      Mesh,
    },
    util::{
      HashMap, 
      rgl::*,
      rvertex::{
        RVertex, RVertex2D
      },
    },
  }
};

pub struct Loader {
  vaos: Vec<GLuint>,
  vbos: Vec<GLuint>,
  meshes: HashMap<String, Mesh>,
  textures: Vec<GLuint>,
  pub quad_1_0: Model,
  pub quad_0_5: Model,
}
impl Default for Loader {
  fn default() -> Self {
    let mut out = Loader {
      vaos: Vec::new(),
      vbos: Vec::new(),
      meshes: HashMap::new(),
      textures: Vec::new(),
      quad_1_0: Model::default(),
      quad_0_5: Model::default(),
    };
    let quad_vec = vec!(-1.0,1.0, -1.0,-1.0, 1.0,1.0, 1.0,-1.0);
    out.quad_1_0 = out.load_to_vao_gui(&quad_vec);
    let quad_vec = vec!(-0.5,0.5, -0.5,-0.5, 0.5,0.5, 0.5,-0.5);
    out.quad_0_5 = out.load_to_vao_gui(&quad_vec);
    out
  }
}
impl Loader {
  pub fn load_to_vao(&mut self, mesh_name: &str) -> Model {
    let (indcs, verts) = match self.load_mesh(mesh_name) {
      Some(mesh) => { (mesh.indcs.clone(), mesh.verts.clone()) }
      _ => panic!("Can't load Mesh: {}", mesh_name)
    };
    let vao_id = self.create_vao();
    self.bind_indices(&indcs);
    let vdata = verts_pos_to_glfloats(&verts); self.bind_attrib(0, 3_i32, &vdata);
    let tdata = verts_tex_coords_to_glfloats(&verts) ; self.bind_attrib(1, 2_i32, &tdata);
    let ndata = verts_norms_to_glfloats(&verts); self.bind_attrib(2, 3_i32, &ndata);
    self.unbind_vao();
    Model::new(vao_id, indcs.len() as i32)
  }
  pub fn create_empty_vbo(&mut self, count: usize) -> VboID { unsafe {
    let vbo_id: GLuint = r_gen_buffers();
    assert!(vbo_id != 0);
    self.vbos.push(vbo_id);
    BindBuffer(ARRAY_BUFFER, vbo_id);
    BufferData(ARRAY_BUFFER,
      (count * mem::size_of::<GLfloat>()) as GLsizeiptr,
      ptr::null(),
      STREAM_DRAW);
    BindBuffer(ARRAY_BUFFER, 0_u32);
    VboID(vbo_id)
  }}
  pub fn load_mesh(&mut self, name: &str) -> Option<&Mesh> {
    if self.meshes.get(name).is_none() {
      let mesh = match load_obj(name) {
        Ok(mesh) => { mesh }
        _ => panic!("Mesh {} failed to load.", name)
      };
      self.meshes.insert(name.to_string(), mesh);
    }
    self.meshes.get(name)
  }
  pub fn create_vao(&mut self) -> GLuint { unsafe {
    let vao_id: GLuint = r_gen_vertex_arrays();
    assert!(vao_id != 0);
    self.vaos.push(vao_id);
    BindVertexArray(vao_id);
    vao_id
  }}
  pub fn bind_attrib(&mut self, attrib: u32, step: GLint, data: &[GLfloat]) { unsafe {
    let vbo_id: GLuint = r_gen_buffers();
    assert!(vbo_id != 0);
    self.vbos.push(vbo_id);
    BindBuffer(ARRAY_BUFFER, vbo_id);
    BufferData(ARRAY_BUFFER,
      (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
      &data[0] as *const f32 as *const std::ffi::c_void,
      STATIC_DRAW);
    VertexAttribPointer(attrib, step, FLOAT, FALSE, 0, ptr::null());
    BindBuffer(ARRAY_BUFFER, 0_u32);
  }}
  pub fn bind_indices(&mut self, idxs: &[u16]) { unsafe {
    let vbo_id = r_gen_buffers();
    self.vbos.push(vbo_id);
    BindBuffer(ELEMENT_ARRAY_BUFFER, vbo_id);
    let _idxs = indices_to_gluints(idxs);
    BufferData(ELEMENT_ARRAY_BUFFER,
      (_idxs.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
      &_idxs[0] as *const u32 as *const std::ffi::c_void,
      STATIC_DRAW);
  }}
  pub fn unbind_vao(&self) { unsafe {
    BindVertexArray(0_u32);
  }}
  pub fn load_texture(&mut self, tex_name: &str) -> Texture {
    let path: &str = &format!("res/img/{}.png", tex_name);
    let img = match image::open(&Path::new(path)) {
      Ok(image) => {
        // println!("Image loaded");
        image.to_rgba()
      },
      _ => panic!("Failed to load image")
    };
    let (width, height) = img.dimensions();
    let img_raw = img.into_raw();
    let tex_id: GLuint = r_gen_textures();
    // println!("texture: image<{}> tex_id<{}>", tex_name, tex_id);
    assert!(tex_id != 0, "tex_id should not be 0");
    unsafe {
      BindTexture(TEXTURE_2D, tex_id);
      TexImage2D(
        TEXTURE_2D, 0, RGBA as i32, width as i32, height as i32, 0, RGBA, UNSIGNED_BYTE, 
        &img_raw[0] as *const u8 as *const std::ffi::c_void
      );
      TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
      TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
      GenerateMipmap(TEXTURE_2D);
      TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
      TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
      TexParameterf(TEXTURE_2D, TEXTURE_LOD_BIAS, 0.0);
      //BindTexture(TEXTURE_2D, 0);
    }
    self.textures.push(tex_id);
    Texture::new(tex_name, tex_id)
  }
  pub fn load_to_vao_gui(&mut self, verts: &[f32]) -> Model {
    let vao_id = self.create_vao();
    self.bind_attrib(0, 2, &verts);
    self.unbind_vao();
    Model::new(vao_id, (verts.len() / 2) as i32)
  }
  pub fn load_to_vao_2d(&mut self, verts: &[f32], tex_coords: &[f32]) -> u32 {
    let vao_id = self.create_vao();
    self.bind_attrib(0, 2, &verts);
    self.bind_attrib(1, 2, &tex_coords);
    self.unbind_vao();
    vao_id
  }
  pub fn rm_vao(&mut self, id: u32) {
    for i in 0..self.vaos.len() {
      if self.vaos[i] == id {
        self.vaos.remove(i);
        break; } }
    unsafe { DeleteVertexArrays(1_i32, &id); }
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
pub fn indices_to_gluints(idxs: &[u16]) -> Vec<GLuint> {
  let mut out = Vec::new();
  for idx in idxs {
    out.push(u32::from(*idx));
  }
  out
}
