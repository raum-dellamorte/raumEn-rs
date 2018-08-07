

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use std::sync::{Arc, Mutex};
use CVOID;

// use entities::position::PosMarker;
use model::Model;
use shader::terrain::gen_terrain_shader;
use shader::Shader;
use terrain::{World, Chunk, ChunkColumn, Platform};
// use util::rvector::{Vector3f, }; // Vector2f, Vector4f, 
// use util::rvertex::{RVertex, RVertex2D};


pub struct RenderTerrain {
  pub shader: Shader,
}
impl RenderTerrain {
  pub fn new() -> Self {
    RenderTerrain {
      shader: gen_terrain_shader(),
    }
  }
  pub fn render(&mut self, world_arc: Arc<Mutex<World>>) {
    let mut world = world_arc.lock().unwrap();
    let vc = {
      let model = &world.model;
      self.bind_tex_model(model);
      model.raw().vertex_count
    };
    for chunk_arc in world.nearby() {
      let chunk = chunk_arc.lock().unwrap();
      for col in &chunk.columns {
        for platform in &col.platforms {
          self.prep_instance(&chunk, &col, platform);
          unsafe { DrawElements(TRIANGLES, vc, UNSIGNED_INT, CVOID); }
        }
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
  pub fn prep_instance(&mut self, chunk: &Chunk, col: &ChunkColumn, platform: &Platform) {
    let trans_mat = platform.transformation(chunk.base, chunk.height, chunk.x, chunk.z, col.x, col.z);
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