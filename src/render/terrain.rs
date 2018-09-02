

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
// use std::sync::{Arc, Mutex};
use CVOID;

// use entities::position::PosMarker;
use gamemgr::GameMgr;
use model::RawModel;
use shader::gen_terrain_shader;
use shader::Shader;
use terrain::Platform; // World, Chunk, ChunkColumn, 
use texture::Texture;
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
    let _arc = mgr.world.clone();
    let mut world = _arc.lock().unwrap();
    let vc = {
      let model = mgr.model(&world.model);
      Self::bind_model(&model);
      model.vertex_count
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
          Self::use_material(mgr, &self.shader, &platform.material);
          self.prep_instance(platform);
          unsafe { DrawElements(TRIANGLES, vc, UNSIGNED_INT, CVOID); }
        }
      }
    }
    Self::unbind();
    self.shader.stop();
  }
  pub fn prep_instance(&mut self, platform: &Platform) {
    platform.transformation(&mut self.trans_mat);
    self.shader.load_matrix("u_Transform", &self.trans_mat);
    // self.shader.load_float("row_count", 1_f32); // float numOfRows
    // self.shader.load_vec_2f("offset", &Vector2f {x: 0_f32, y: 0_f32}); // vec2 offset;
    // println!("{:?}", trans_mat)
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
  fn use_material(mgr: &mut GameMgr, shader: &Shader, material: &str) {
    let (lighting, texture) = {
      let _arc = mgr.material(material);
      let material = _arc.lock().unwrap();
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