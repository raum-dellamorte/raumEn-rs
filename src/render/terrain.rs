

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
// use std::sync::{Arc, Mutex};
use CVOID;

// use entities::position::PosMarker;
use GameMgr;
use model::RawModel;
use shader::gen_terrain_shader;
use shader::Shader;
use terrain::{World, Platform, from_world_to_chunk_space}; // Chunk, ChunkColumn, 
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
  pub fn render(&mut self, mgr: Box<GameMgr>) -> Box<GameMgr> {
    let mut mgr = mgr;
    self.shader.start();
    let mut world = mgr.take_world();
    let vc = {
      let model = mgr.model(&world.model);
      Self::bind_model(&model);
      model.vertex_count
    };
    self.shader.load_matrix("u_View", &mgr.view_mat);
    mgr.lights_do(|lights| { lights.load_to_shader(&self.shader); });
    // self.shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    // self.shader.load_bool("use_clip_plane", false); // float useClipPlane;
    // self.shader.load_vec_3f("sky_color", &Vector3f::new(0.5, 0.6, 0.5));
    let chunks = world.take_nearby(mgr.player_loc.x, mgr.player_loc.z); // need player location
    for chunk in &chunks {
      for (_c_loc, col) in &chunk.columns {
        for platform in &col.platforms {
          Self::use_material(&mut mgr, &self.shader, &platform.material);
          self.prep_instance(&world, platform);
          unsafe { DrawElements(TRIANGLES, vc, UNSIGNED_INT, CVOID); }
        }
      }
    }
    Self::unbind();
    self.shader.stop();
    world.return_chunks(chunks);
    mgr.return_world(world);
    mgr
  }
  pub fn prep_instance(&mut self, world: &Box<World>, platform: &Platform) {
    platform.transformation(world, &mut self.trans_mat);
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
      let _mat = mgr.material(material);
      let material = _mat.borrow_mut();
      (&material.lighting.clone(), &material.texture.clone())
    };
    {
      let lighting = mgr.lighting(lighting);
      lighting.borrow_mut().load_to_shader(shader);
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
