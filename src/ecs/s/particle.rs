
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
      RVec,
      Vector3f,
      Matrix4f, 
      rgl::*,
      specs::*,
    },
    Handler,
    Camera,
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
    r_bind_vaa_7(model);
    GlSettings::default()
        .disable_depth_mask()
        .enable_blend()
        .set();
    RBlend::AdditiveBlend.r_blend_func();
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
    GlSettings::default()
        .enable_depth_mask()
        .disable_blend()
        .set();
    r_unbind_vaa_7();
    shader.stop();
  }
}

#[derive(SystemData)]
pub struct DrawParticlesData<'a> {
  pub shader:      Read<'a, ParticleShader>, 
  pub view:        Read<'a, ViewMatrix>,
  pub models:      Read<'a, Models>, 
  pub textures:    Read<'a, Textures>, 
  pub lightings:   Read<'a, Lightings>, 
  pub ents:        Entities<'a>,
  pub pos:         ReadStorage<'a, Position>,
  pub rot:         ReadStorage<'a, Rotation>,
  pub model:       ReadStorage<'a, ModelName>,
  pub texture:     ReadStorage<'a, TexName>,
  pub lighting:    ReadStorage<'a, LightingName>,
  pub particle:    ReadStorage<'a, ParticleAlive>,
  pub cam_dist:    ReadStorage<'a, CamDistance>,
  pub tex_offsets: ReadStorage<'a, TexOffsets>,
}
impl<'a> DrawParticlesData<'a> {
  pub fn entities(&self) -> Vec<Entity> {
    let mut d = (
      &self.ents, &self.model, &self.texture, 
      &self.cam_dist, &self.pos, &self.rot, 
      &self.particle, ).join().collect::<Vec<_>>();
    d.sort_by(|&a,&b| {
      match a.1.cmp(&b.1) {
        Ordering::Equal => {
          match a.2 .0.cmp(&b.2 .0) {
            Ordering::Equal => {
              b.3 .0.partial_cmp(&a.3 .0).expect(
                    "A wild NAN has appearded in a CamDistance")
            }
            x => { x }
          }
        }
        x => { x }
      }
    });
    d.iter().map(|(e, _, _, _, _, _, _)| { *e }).collect()
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
  type SystemData = UpdateParticlesData<'a>;
  fn run(&mut self, data: Self::SystemData) {
    let mut data = data;
    let delta = data.delta();
    let cam_pos = data.cam_pos();
    let mut dead = Vec::new();
    for (e, pos, vel, cam, life, offsets, rows, grav_percent, _) in data.join() {
      life.inc_time(delta);
      if life.is_dead() { dead.push(e); continue; }
      vel.0.y -= 10.0 * grav_percent.0 * delta;
      pos.0 += vel.0 * delta;
      cam.0 = (cam_pos - pos.0).len_sqr();
      offsets.update(life, rows.0);
    }
    for e in dead {
      data.alive.remove(e);
    }
  }
}
#[derive(SystemData)]
pub struct UpdateParticlesData<'a> {
  pub handler:      Read<'a, Handler>, 
  pub camera:       Read<'a, Camera>, 
  pub ents:         Entities<'a>,
  pub position:     WriteStorage<'a, Position>,
  pub velocity:     WriteStorage<'a, Velocity>,
  pub cam_distance: WriteStorage<'a, CamDistance>,
  pub life:         WriteStorage<'a, TimedLife>,
  pub tex_offsets:  WriteStorage<'a, TexOffsets>,
  pub alive:        WriteStorage<'a, ParticleAlive>,
  pub grav_percent: ReadStorage<'a, GravPercent>,
  pub row_count:    ReadStorage<'a, RowCount>,
}
impl<'a> UpdateParticlesData<'a> {
  pub fn join(&mut self) -> specs::join::JoinIter<(
      &specs::Read<'a, specs::world::EntitiesRes>, 
      &mut SpecsWrStrg<'a, Position>, 
      &mut SpecsWrStrg<'a, Velocity>, 
      &mut SpecsWrStrg<'a, CamDistance>, 
      &mut SpecsWrStrg<'a, TimedLife>, 
      &mut SpecsWrStrg<'a, TexOffsets>, 
      &SpecsRdStrg<'a, RowCount>,
      &SpecsRdStrg<'a, GravPercent>,
      &mut SpecsWrStrg<'a, ParticleAlive>,
  )> {
    (
      &self.ents, 
      &mut self.position,
      &mut self.velocity,
      &mut self.cam_distance,
      &mut self.life,
      &mut self.tex_offsets,
      &self.row_count,
      &self.grav_percent,
      &mut self.alive,
    ).join()
  }
  pub fn delta(&self) -> f32 {
    self.handler.timer.delta
  }
  pub fn cam_pos(&self) -> Vector3f<f32> {
    self.camera.pos
  }
}

// specs::join::JoinIter<(
//   &specs::Read<'_, specs::world::EntitiesRes>, 
//   &mut specs::Storage<'_, Position, 
//     specs::shred::FetchMut<'_, 
//       specs::storage::MaskedStorage<Position>>>, 
//   &mut specs::Storage<'_, Velocity, 
//     specs::shred::FetchMut<'_, 
//       specs::storage::MaskedStorage<Velocity>>>, 
//   &specs::Storage<'_, ParticleAlive, 
//     specs::shred::Fetch<'_, 
//       specs::storage::MaskedStorage<ParticleAlive>>>
// )>

// specs::join::JoinIter<(
//   &specs::Read<'_, specs::world::EntitiesRes>, 
//   &mut specs::Storage<'a, Position,
//     specs::shred::FetchMut<'a, 
//       specs::storage::MaskedStorage<Position>>>, 
//   &mut specs::Storage<'a, Velocity, 
//     specs::shred::FetchMut<'a, 
//       specs::storage::MaskedStorage<Velocity>>>, 
//   &specs::Storage<'_, ParticleAlive, 
//     specs::shred::Fetch<'_, 
//       specs::storage::MaskedStorage<ParticleAlive>>>
// )>
