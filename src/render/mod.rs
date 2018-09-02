#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod font;
pub mod model;
pub mod terrain;

use render::model::RenderTexModel;
use render::terrain::RenderTerrain;
use render::font::RenderFont;

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use glutin::dpi::PhysicalSize;
// use std::collections::HashMap;
// use std::ffi::CString;
// use std::mem;
// use std::ptr;
// use std::str;
// use std::sync::{Arc, Mutex};
// use CVOID;

// use camera::Camera;
// use entities::position::PosMarker;
use gamemgr::GameMgr;
// use model::Model;
// use shader::lighting::Lights;
// use shader::model::gen_model_shader;
// use shader::Shader;
// use util::rvector::{Vector3f, }; // Vector2f, Vector4f, 
// use util::rvertex::{RVertex, RVertex2D};

pub fn prepare() { unsafe {
  Enable(CULL_FACE);
  CullFace(BACK);
  Enable(DEPTH_TEST);
  Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
  ClearColor(0.2, 0.2, 0.3, 1.0);
}}

pub struct RenderMgr {
  pub mgr: GameMgr,
  pub ren_tex_model: RenderTexModel,
  pub ren_terrain: RenderTerrain,
  pub ren_font: RenderFont,
}

impl RenderMgr {
  pub fn new() -> Self {
    RenderMgr {
      mgr: GameMgr::new(),
      ren_tex_model: RenderTexModel::new(),
      ren_terrain: RenderTerrain::new(),
      ren_font: RenderFont::new(),
    }
  }
  pub fn render(&mut self) { 
    prepare();
    self.mgr.create_view_matrix();
    self.ren_tex_model.render(&mut self.mgr.clone());
    self.ren_terrain.render(&mut self.mgr.clone());
    self.ren_font.render(self.mgr.clone());
    unsafe { BindVertexArray(0); }
  }
  pub fn load_proj_mat(&mut self, size: PhysicalSize) {
    let mut camera = self.mgr.camera.lock().unwrap();
    camera.update_size(size.into());
    let proj_mat = camera.projection();
    {
      let shader = &self.ren_tex_model.shader;
      shader.start();
      shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
      shader.stop();
    }
    {
      let shader = &self.ren_terrain.shader;
      shader.start();
      shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
      shader.stop();
    }
  }
  pub fn clean_up(&mut self) {
    self.mgr.clean_up();
    self.ren_tex_model.clean_up();
  }
}