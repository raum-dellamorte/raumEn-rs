

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
// use std::sync::{Arc, Mutex};
use CVOID;

// use entities::position::PosMarker;
use gamemgr::GameMgr;
use model::Model;
use shader::terrain::gen_terrain_shader;
use shader::Shader;
use terrain::{Chunk, ChunkColumn, Platform}; // World, 
use util::rmatrix::Matrix4f;
use util::rvector::{Vector3f, }; // Vector2f, Vector4f, 
// use util::rvertex::{RVertex, RVertex2D};


pub struct RenderTerrain {
  pub shader: Shader,
  pub trans_mat: Matrix4f,
}
impl RenderTerrain {
  pub fn new() -> Self {
    RenderTerrain {
      shader: gen_terrain_shader(),
      trans_mat: Matrix4f::new(),
    }
  }
  pub fn render(&mut self, mgr: &mut GameMgr) {
    self.shader.start();
    let world_arc = mgr.world.clone();
    let mut world = world_arc.lock().unwrap();
    let vc = {
      let model = &world.model;
      self.bind_tex_model(model);
      model.raw().vertex_count
    };
    self.shader.load_matrix("u_View", &mgr.view_mat);
    mgr.lights_do(|lights| { lights.load_to_shader(&self.shader); });
    // self.shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    // self.shader.load_bool("use_clip_plane", false); // float useClipPlane;
    self.shader.load_vec_3f("sky_color", &Vector3f::new(0.5, 0.6, 0.5));
    for chunk_arc in world.nearby() {
      let chunk = chunk_arc.lock().unwrap();
      for col in &chunk.columns {
        for platform in &col.platforms {
          self.prep_instance(&chunk, &col, platform);
          unsafe { DrawElements(TRIANGLES, vc, UNSIGNED_INT, CVOID); }
        }
      }
      self.unbind_tex_model();
      self.shader.stop();
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
  pub fn prep_instance(&mut self, chunk: &Chunk, col: &ChunkColumn, platform: &Platform) {
    platform.transformation(&mut self.trans_mat, chunk.base, chunk.height, chunk.x, chunk.z, col.x, col.z);
    self.shader.load_matrix("u_Transform", &self.trans_mat);
    // self.shader.load_float("row_count", 1_f32); // float numOfRows
    // self.shader.load_vec_2f("offset", &Vector2f {x: 0_f32, y: 0_f32}); // vec2 offset;
    // println!("{:?}", trans_mat)
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