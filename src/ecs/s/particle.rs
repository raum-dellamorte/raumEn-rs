
use {
  std::cmp::Ordering,
  specs::{
    prelude::*, 
    System, Read, ReadStorage, WriteStorage, Entities, Join, 
  },
  crate::{
    ecs::{
      c::{
        flags::*,
        components::*,
      },
      resource::*,
    },
    shader::ParticleShader,
    util::{
      Vector3f,
      Matrix4f, 
      rgl::*,
    },
    Handler,
    ViewMatrix,
  },
};

pub struct DrawParticles;
impl<'a> System<'a> for DrawParticles {
  type SystemData = DrawParticlesData<'a>;
  fn run(&mut self, data: Self::SystemData) {
    let shader = &data.shader.shader;
    let mut transform = Matrix4f::new();
    let d = data.entities();
    if d.is_empty() { return }
    let mut last_model: &str = &data.model_name(d[0]);
    let mut last_texture: &str = &data.texture_name(d[0]);
    let mut model: &Model = &data.models.0.get(last_model)
        .unwrap_or_else(|| panic!("DrawTexMods: No such Model :{}", last_model));
    let mut texture: &Texture = &data.textures.0.get(last_texture)
        .unwrap_or_else(|| panic!("DrawTexMods: No such Texture :{}", last_texture));
    shader.start();
    shader.load_matrix("u_View", &(*data.view).view);
    // shader.load_vec_3f("light_pos", &(*light).pos); // Unimplemented
    // shader.load_vec_3f("light_color", &(*light).color);
    r_bind_vaa_3(model);
    r_bind_texture(texture);
    for e in d {
      let mdl = data.model_name(e);
      if *mdl != *last_model {
        model = &data.models.0.get(mdl).unwrap();
        last_model = mdl;
        r_bind_vaa_3(model);
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
      let pos = data.position(e);
      transform.set_identity();
      transform.translate_v3f(pos);
      let rot = data.rotation(e);
      transform.rotate(rot.y.to_radians(), crate::util::YVEC);
      // transform.scale(&p.scale(200.0));
      shader.load_matrix("u_Transform", &transform);
      r_draw_triangles(model);
    }
    r_unbind_vaa_3();
    shader.stop();
  }
}

#[derive(SystemData)]
pub struct DrawParticlesData<'a> {
  pub shader:    Read<'a, ParticleShader>, 
  pub view:      Read<'a, ViewMatrix>,
  pub models:    Read<'a, Models>, 
  pub textures:  Read<'a, Textures>, 
  pub lightings: Read<'a, Lightings>, 
  pub ents:      Entities<'a>,
  pub pos:       ReadStorage<'a, Position>,
  pub rot:       ReadStorage<'a, Rotation>,
  pub model:     ReadStorage<'a, ModelName>,
  pub texture:   ReadStorage<'a, TexName>,
  pub lighting:  ReadStorage<'a, LightingName>,
  pub particle:  ReadStorage<'a, ParticleAlive>,
}
impl<'a> DrawParticlesData<'a> {
  pub fn entities(&self) -> Vec<Entity> {
    let mut d = (&self.ents, &self.particle).join().collect::<Vec<_>>();
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
      self.pos.get(*e).is_some() &&
      self.rot.get(*e).is_some() &&
      self.model.get(*e).is_some() &&
      self.texture.get(*e).is_some()
    });
    out
  }
  pub fn model_name(&self, e: Entity) -> &str {
    &self.model.get(e).expect("DrawParticlesData: No ModelName for Entity").0
  }
  pub fn texture_name(&self, e: Entity) -> &str {
    &self.texture.get(e).expect("DrawParticlesData: No ModelName for Entity").0
  }
  pub fn lighting_name(&self, e: Entity) -> &str {
    &self.lighting.get(e).expect("DrawParticlesData: No LightingName for Entity").0
  }
  pub fn position(&self, e: Entity) -> Vector3f<f32> {
    self.pos.get(e).expect("DrawParticlesData: No Position for Entity").0
  }
  pub fn rotation(&self, e: Entity) -> Vector3f<f32> {
    self.rot.get(e).expect("DrawParticlesData: No Rotation for Entity").0
  }
}

pub struct UpdateParticles;
impl<'a> System<'a> for UpdateParticles {
  type SystemData = (
    (
      Read<'a, Handler>,
    ), (
      Entities<'a>,
      WriteStorage<'a, Velocity>,
      ReadStorage<'a, GravPercent>,
    ),
  );
  fn run(&mut self, data: Self::SystemData) {
    let delta = {
      let handler = data.0 .0;
      handler.timer.delta
    };
    // velocity.y -= gravity * gravEffect * delta * delta
    // val change = Vector3f(velocity).apply { scale(delta) }
    // Vector3f.add(change, pos, pos)
    // distance = Vector3f.sub(camera.pos, pos, null).lengthSquared()
    // updateTexCoords()
    // elapsedTime += delta
    // return elapsedTime > life
    let mut data = data.1;
    for (_, vel, grav_percent) in (&data.0, &mut data.1, &data.2).join() {
      vel.0.y -= 10.0 * grav_percent.0 * delta;
    }
  }
}