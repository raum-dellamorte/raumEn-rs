

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
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

pub struct RenderTexModel {
  pub shader: Shader,
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