#[allow(non_snake_case)]
#[allow(unused_imports)]

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use glutin::dpi::PhysicalSize;
// use std::collections::HashMap;
// use std::ffi::CString;
// use std::mem;
// use std::ptr;
// use std::str;
use std::sync::{Arc, Mutex};
use CVOID;

// use camera::Camera;
use entities::Entities;
use entities::position::PosMarker;
use gamemgr::GameMgr;
use model::Model;
// use shader::lighting::Lights;
use shader::model::gen_model_shader;
use shader::Shader;
use util::rvector::{Vector3f, }; // Vector2f, Vector4f, 
// use util::rvertex::{RVertex, RVertex2D};

pub fn prepare() { unsafe {
  Enable(CULL_FACE);
  CullFace(BACK);
  Enable(DEPTH_TEST);
  Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
  ClearColor(0.5, 0.8, 0.3, 1.0);
}}

pub struct RenderTexModel {
  pub shader: Shader
}

impl RenderTexModel {
  pub fn new() -> Self {
    RenderTexModel {
      shader: gen_model_shader(),
    }
  }
  pub fn render(&mut self, entities_arc: Arc<Mutex<Entities>>) {
    let mut entities = entities_arc.lock().unwrap();
    for key in entities.keys() {
      entities.set_key(&key);
      let model_arc = entities.model();
      let model = model_arc.lock().unwrap();
      self.bind_tex_model(&model);
      let ents = entities.entities();
      for entity_arc in ents.lock().unwrap().iter() {
        let mut entity = entity_arc.lock().unwrap();
        self.prep_instance(entity.marker.clone());
        unsafe { DrawElements(TRIANGLES, model.raw().vertex_count, UNSIGNED_INT, CVOID); }
      }
      self.unbind_tex_model();
    }
    
  }
  pub fn bind_tex_model(&mut self, model: &Model) { unsafe {
    BindVertexArray(model.raw().vao_id);
    EnableVertexAttribArray(0);
    EnableVertexAttribArray(1);
    EnableVertexAttribArray(2);
    model.lighting().load_to_shader(&self.shader);
    ActiveTexture(TEXTURE0);
    BindTexture(TEXTURE_2D, model.texture);
  }}
  pub fn prep_instance(&mut self, pos: Arc<Mutex<PosMarker>>) {
    let mut marker = pos.lock().unwrap();
    let trans_mat = marker.transformation();
    self.shader.load_matrix("u_Transform", &trans_mat);
    // self.shader.load_float("row_count", 1_f32); // float numOfRows
    // self.shader.load_vec_2f("offset", &Vector2f {x: 0_f32, y: 0_f32}); // vec2 offset;
  }
  pub fn unbind_tex_model(&mut self) { unsafe {
    DisableVertexAttribArray(2);
    DisableVertexAttribArray(1);
    DisableVertexAttribArray(0);
  }}
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}

pub struct RenderMgr {
  pub mgr: GameMgr,
  pub renderer: RenderTexModel,
  pub entities: Arc<Mutex<Entities>>,
}

impl RenderMgr {
  pub fn new() -> Self {
    let mgr = GameMgr::new();
    let loader = mgr.loader.clone();
    let ents = Entities::new(loader);
    RenderMgr {
      mgr: mgr,
      renderer: RenderTexModel::new(),
      entities: Arc::new(Mutex::new(ents)),
    }
  }
  pub fn render(&mut self) { unsafe {
    prepare();
    self.renderer.shader.start();
    let view_mat = {
      let mut camera = self.mgr.camera.lock().unwrap();
      camera.create_view_matrix()
    };
    self.renderer.shader.load_matrix("u_View", &view_mat);
    let lights = self.mgr.lights.lock().unwrap();
    lights.load_to_shader(&self.renderer.shader);
    // self.renderer.shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    // self.renderer.shader.load_bool("use_clip_plane", false); // float useClipPlane;
    self.renderer.shader.load_vec_3f("sky_color", &Vector3f::new(0.5, 0.6, 0.5));
    
    self.renderer.render(self.entities.clone());
    
    BindVertexArray(0);
    self.renderer.shader.stop();
  }}
  pub fn load_proj_mat(&mut self, size: PhysicalSize) {
    let mut camera = self.mgr.camera.lock().unwrap();
    let shader = &self.renderer.shader;
    camera.update_size(size.into());
    let proj_mat = camera.projection();
    shader.start();
    shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
    shader.stop();
  }
  pub fn clean_up(&mut self) {
    self.mgr.clean_up();
    self.renderer.clean_up();
  }
}