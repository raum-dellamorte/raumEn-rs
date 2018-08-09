#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod model;
pub mod terrain;

use render::model::RenderTexModel;
use render::terrain::RenderTerrain;

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
}

impl RenderMgr {
  pub fn new() -> Self {
    RenderMgr {
      mgr: GameMgr::new(),
      ren_tex_model: RenderTexModel::new(),
      ren_terrain: RenderTerrain::new(),
    }
  }
  pub fn render(&mut self) { 
    prepare();
    self.mgr.create_view_matrix();
    self.ren_tex_model.render(&mut self.mgr.clone());
    
    // self.ren_terrain.shader.start();
    // self.ren_terrain.shader.load_matrix("u_View", &view_mat);
    // self.mgr.clone().lights_do(|lights| { lights.load_to_shader(&self.ren_terrain.shader); });
    // // self.ren_terrain.shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    // // self.ren_terrain.shader.load_bool("use_clip_plane", false); // float useClipPlane;
    // self.ren_terrain.shader.load_vec_3f("sky_color", &Vector3f::new(0.5, 0.6, 0.5));
    // self.ren_terrain.render(self.world.clone());
    
    // self.ren_terrain.shader.stop();
    unsafe { BindVertexArray(0); }
  }
  pub fn load_proj_mat(&mut self, size: PhysicalSize) {
    let mut camera = self.mgr.camera.lock().unwrap();
    let shader = &self.ren_tex_model.shader;
    camera.update_size(size.into());
    let proj_mat = camera.projection();
    shader.start();
    shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
    shader.stop();
  }
  pub fn clean_up(&mut self) {
    self.mgr.clean_up();
    self.ren_tex_model.clean_up();
  }
}