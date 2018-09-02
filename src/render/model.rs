

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use std::sync::{Arc, Mutex};
use CVOID;

// use camera::Camera;
use entities::position::PosMarker;
use gamemgr::GameMgr;
use model::RawModel;
// use shader::lighting::Lights;
use shader::gen_model_shader;
use shader::Shader;
use texture::Texture;
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
  pub fn render(&mut self, mgr: &mut GameMgr) {
    self.shader.start();
    self.shader.load_matrix("u_View", &mgr.view_mat);
    mgr.lights_do(|lights| { lights.load_to_shader(&self.shader); });
    // self.shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    // self.shader.load_bool("use_clip_plane", false); // float useClipPlane;
    self.shader.load_vec_3f("sky_color", &Vector3f::new(0.5, 0.6, 0.5));
    let _arc = mgr.entities.clone();
    let entities = _arc.lock().unwrap();
    for entity in entities.values() {
      let model = mgr.model(&entity.model);
      Self::bind_model(&model);
      Self::use_material(mgr, &self.shader, &entity.material);
      for ent in &entity.instances {
        self.prep_instance(ent.marker.clone());
        unsafe { DrawElements(TRIANGLES, model.vertex_count, UNSIGNED_INT, CVOID); }
      }
      Self::unbind();
    }
    self.shader.stop();
  }
  pub fn prep_instance(&mut self, pos: Arc<Mutex<PosMarker>>) {
    let mut marker = pos.lock().unwrap();
    let trans_mat = marker.transformation();
    self.shader.load_matrix("u_Transform", trans_mat);
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
  fn use_material(mgr: &mut GameMgr, shader: &Shader, material: &str) {
    let (lighting, texture) = {
      let _arc = mgr.material(material);
      let material = _arc.lock().unwrap();
      shader.load_float("row_count", material.row_count as f32); // float numOfRows
      shader.load_vec_2f("offset", &material.offset); // vec2 offset;
      (&material.lighting.clone(), &material.texture.clone())
    };
    {
      let _arc = mgr.lighting(lighting);
      let lighting = _arc.lock().unwrap();
      lighting.load_to_shader(shader);
    }
    {
      let texture = mgr.texture(texture);
      Self::bind_texture(&texture);
    }
  }
  fn bind_model(model: &RawModel) { unsafe {
    BindVertexArray(model.vao_id);
    EnableVertexAttribArray(0);
    EnableVertexAttribArray(1);
    EnableVertexAttribArray(2);
  }}
  fn bind_texture(texture: &Texture) { unsafe {
    ActiveTexture(TEXTURE0);
    BindTexture(TEXTURE_2D, texture.tex_id);
  }}
  fn unbind() { unsafe {
    DisableVertexAttribArray(2);
    DisableVertexAttribArray(1);
    DisableVertexAttribArray(0);
  }}
}
