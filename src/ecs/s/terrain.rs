
use {
  std::cmp::Ordering,
  
  util::{
    Matrix4f, 
  },
  specs::{
    prelude::*,
    System, Read, ReadStorage, WriteStorage, Entities, Join, 
  },
  ecs::{
    c::{
      flags::*,
      components::*,
    },
    resource::*,
  },
  crate::{
    util::{
      rgl::*, 
      // Vector3f, 
    },
    DISPLAY,
    shader::TerrainShader,
  },
};

pub struct DrawPlatform;
impl<'a> System<'a> for DrawPlatform {
  type SystemData = DrawPlatformData<'a>;
  fn run(&mut self, data: Self::SystemData) {
    let shader = &data.shader.shader;
    let mut transform = Matrix4f::new();
    let d = data.entities();
    if d.is_empty() { return }
    let mut last_model: &str = &data.model_name(d[0]);
    let mut last_texture: &str = &data.texture_name(d[0]);
    let mut model: &Model = &data.models.0.get(last_model)
        .unwrap_or_else(|| panic!("DrawPlatform: No such Model :{}", last_model));
    let mut texture: &Texture = &data.textures.0.get(last_texture)
        .unwrap_or_else(|| panic!("DrawPlatform: No such Texture :{}", last_texture));
    shader.start();
    {
      let view = DISPLAY.lock().unwrap().camera.view_mat;
      shader.load_matrix("u_View", &view);
    }
    // shader.load_vec_3f("light_pos", &(*light).pos); // Unimplemented
    // shader.load_vec_3f("light_color", &(*light).color);
    r_bind_vaa_3(model.vao_id);
    r_bind_texture(texture);
    for e in d {
      let mdl = data.model_name(e);
      if *mdl != *last_model {
        model = &data.models.0.get(mdl).unwrap();
        last_model = mdl;
        r_bind_vaa_3(model.vao_id);
      }
      let tex = data.texture_name(e);
      if *tex != *last_texture {
        texture = &data.textures.0.get(tex).unwrap();
        last_texture = tex;
        r_bind_texture(texture);
      }
      let ltg = data.lighting_name(e);
      if let Some(ref lighting) = data.lightings.0.get(ltg) {
        lighting.load_to_shader(shader);
      }
      let p = data.get_platform(e);
      transform.set_identity();
      transform.translate_v3f(p.pos);
      transform.scale(p.scale);
      shader.load_matrix("u_Transform", &transform);
      r_draw_triangles(model.vertex_count);
    }
    r_unbind_vaa_3();
    shader.stop();
  }
}

#[derive(SystemData)]
pub struct DrawPlatformData<'a> {
  pub shader:    Read<'a, TerrainShader>, 
  pub models:    Read<'a, Models>, 
  pub textures:  Read<'a, Textures>, 
  pub lightings: Read<'a, Lightings>, 
  pub ents:      Entities<'a>,
  pub platform:  ReadStorage<'a, Platform>,
  pub model:     ReadStorage<'a, ModelName>,
  pub texture:   ReadStorage<'a, TexName>,
  pub lighting:  ReadStorage<'a, LightingName>,
  pub particle:  ReadStorage<'a, ParticleAlive>,
}
impl<'a> DrawPlatformData<'a> {
  pub fn entities(&self) -> Vec<Entity> {
    let mut d = (&self.ents, &self.platform).join().collect::<Vec<_>>();
    d.sort_by(|&a,&b| {
      match (self.model.get(a.0), self.model.get(b.0)) {
        (Some(_ma), Some(_mb)) => {
          match _ma.cmp(&_mb) {
            Ordering::Equal => {
              match (self.texture.get(a.0), self.texture.get(b.0)) {
                (Some(_ta), Some(_tb)) => {
                  _ta.0.cmp(&_tb.0)
                }
                (Some(_ta), None) => { Ordering::Less }
                (None, Some(_tb)) => { Ordering::Greater }
                _ => { Ordering::Equal }
              }
            }
            x => { x }
          }
        }
        (Some(_), None) => { Ordering::Less }
        (None, Some(_)) => { Ordering::Greater }
        _ => { Ordering::Equal }
      }
    });
    let mut out: Vec<Entity> = d.iter().map(|(e, _)| { *e }).collect();
    out.retain(|e| {
      self.platform.get(*e).is_some() &&
      self.model.get(*e).is_some() &&
      self.texture.get(*e).is_some()
    });
    out
  }
  pub fn model_name(&self, e: Entity) -> &str {
    &self.model.get(e).expect("DrawPlatformData: No ModelName for Entity").0
  }
  pub fn texture_name(&self, e: Entity) -> &str {
    &self.texture.get(e).expect("DrawPlatformData: No ModelName for Entity").0
  }
  pub fn lighting_name(&self, e: Entity) -> &str {
    &self.lighting.get(e).expect("DrawPlatformData: No LightingName for Entity").0
  }
  pub fn get_platform(&self, e: Entity) -> &Platform {
    self.platform.get(e).expect("DrawPlatformData: No Platform for Entity")
  }
}

/// We want to mark platforms within a certain distance as local to the player
/// so we know which should be processed when dealing with movement and other 
/// things.  We don't want to have to process every platform in existence every
/// time we want to know where a player can jump.
pub struct MarkLocalToPlayer;
impl<'a> System<'a> for MarkLocalToPlayer {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Platform>,
    WriteStorage<'a, LocalToPlayer>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (_ents, _pforms, mut _local) = data;
    
  }
}
