

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
  pub mgr: Option<Box<GameMgr>>,
  pub ren_tex_model: RenderTexModel,
  pub ren_terrain: RenderTerrain,
  pub ren_font: RenderFont,
}

impl RenderMgr {
  pub fn new() -> Self {
    RenderMgr {
      mgr: Some(Box::new(GameMgr::new())),
      ren_tex_model: RenderTexModel::new(),
      ren_terrain: RenderTerrain::new(),
      ren_font: RenderFont::new(),
    }
  }
  pub fn take_mgr(&mut self) -> Box<GameMgr> {
    let out = self.mgr.take();
    Box::new(*out.unwrap())
  }
  pub fn return_mgr(&mut self, mgr: Box<GameMgr>) {
    self.mgr = Some(mgr);
  }
  pub fn render(&mut self) { 
    prepare();
    let mut mgr = self.take_mgr();
    mgr.create_view_matrix();
    mgr = self.ren_tex_model.render(mgr);
    mgr = self.ren_terrain.render(mgr);
    mgr = self.ren_font.render(mgr);
    self.return_mgr(mgr);
    unsafe { BindVertexArray(0); }
  }
  pub fn update_size(&mut self, dimensions: (u32, u32)) {
    let mut mgr = self.take_mgr();
    {
      mgr = mgr.update_size(dimensions);
      let mut d = mgr.display.lock().unwrap();
      let proj_mat = d.projection();
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
    self.return_mgr(mgr);
  }
  pub fn clean_up(&mut self) {
    self.mgr.as_mut().unwrap().clean_up();
    self.ren_tex_model.clean_up();
  }
}