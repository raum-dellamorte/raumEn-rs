

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
use CVOID;

use entities::position::PosMarker;
use {GameMgr, Shader, Texture, }; // Camera, 
use model::Model;
use shader::gen_model_shader;
use util::{
  rgl::*,
  Vector3f, 
  Rc, RefCell, 
  // Vector2f, Vector4f, 
}; 
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
  pub fn render(&mut self, mgr: Box<GameMgr>) -> Box<GameMgr> {
    let mut mgr = mgr;
    let shader = &self.shader;
    shader.start();
    shader.load_matrix("u_View", &mgr.view_mat);
    mgr.lights_do(|lights| { lights.load_to_shader(shader); });
    // shader.load_vec_4f("plane", &Vector4f {x: 0_f32, y: 10000_f32, z: 0_f32, w: 1_f32, }); // vec4 plane;
    // shader.load_bool("use_clip_plane", false); // float useClipPlane;
    // shader.load_vec_3f("sky_color", &Vector3f::new(0.5, 0.6, 0.5));
    {
      let emgr = &mgr.entity_mgr;
      let entities = emgr.entities.borrow_mut();
      let instances = emgr.instances.borrow();
      for _entity in entities.values() {
        let entity = _entity.borrow_mut();
        let model = mgr.model(&entity.model);
        r_bind_vaa_3(&model);
        Self::use_material(&mgr, &self.shader, &entity.material);
        if let Some(_instances) = instances.get(&entity.name) {
          for ent_inst in _instances {
            let ent = ent_inst.borrow();
            {
              let mut marker = ent.marker.borrow_mut();
              let trans_mat = marker.transformation();
              shader.load_matrix("u_Transform", trans_mat);
            }
            shader.load_vec_3f("color_id", &ent.color_id.borrow()); // add color id to entities to use here.
            r_draw_triangles(&model);
          }
        }
        r_unbind_vaa_3();
      }
    }
    shader.stop();
    mgr
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
  fn use_material(mgr: &GameMgr, shader: &Shader, material: &str) {
    let (lighting, texture) = {
      let _mat = mgr.material(material);
      let material = _mat.borrow_mut();
      shader.load_float("row_count", material.row_count as f32); // float numOfRows
      shader.load_vec_2f("offset", &material.offset); // vec2 offset;
      (&material.lighting.clone(), &material.texture.clone())
    };
    {
      let lighting = mgr.lighting(lighting);
      lighting.borrow_mut().load_to_shader(shader);
    }
    {
      let texture = mgr.texture(texture);
      use util::rgl::r_bind_texture;
      r_bind_texture(&texture);
    }
  }
}
