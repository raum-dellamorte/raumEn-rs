
use {
  std::cmp::Ordering,
  specs::{
    prelude::*, 
    System, Read, ReadStorage, WriteStorage, Entities, Join, 
  },
  crate::{
    Handler,
    constants::DISPLAY,
    ecs::{
      c::{
        flags::*,
        components::*,
      },
      resource::*,
    },
    shader::{ ParticleShader, ShaderWrapper, },
    util::{
      RVec,
      Vector3f,
      Matrix4f, 
      HashMap, 
      rgl::*,
      specs::*,
    },
  },
};

pub struct DrawParticles;
impl<'a> System<'a> for DrawParticles {
  type SystemData = DrawParticlesData<'a>;
  fn run(&mut self, data: Self::SystemData) {
    let shader = &data.shader;
    let quad = data.pvbo.quad;
    let vbo_id = data.pvbo.vbo_id;
    let max_inst = data.pvbo.max_instances;
    let inst_data_len = data.pvbo.instance_data_length;
    // println!("{:?} {:?}", quad, vbo_id);
    
    // let mut transform: Matrix4f<f32> = Matrix4f::new();
    let d = data.entities();
    if d.is_empty() { return }
    let view = DISPLAY.lock().unwrap().camera.view_mat.clone();
    // println!("{:?}", view);
    shader.start();
    r_bind_vaa_7(quad.vao_id);
    GlSettings::default()
        .disable_depth_mask()
        .enable_blend()
        .set();
    RBlend::AdditiveBlend.exec();
    for tex in d.keys() {
      if let Some(batch) = d.get(tex) {
        let texture: &Texture = &data.textures.0.get(tex)
            .unwrap_or_else(|| panic!("DrawParticles: No such Texture :{}", tex));
        r_bind_texture(texture);
        shader.load_float("rowCount", 4.0);
        let mut vbo_i = 0;
        let parts_count = batch.len();
        let parts_size = parts_count * inst_data_len;
        let mut vbo_data = vec!(0_f32; parts_size);
        // ( Entity, &TexName, &CamDistance, 
        //   &Position, &Rotation, &ScaleFloat, &TexOffsets, 
        //   &ParticleAlive,)
        for (_, _, _, pos, rot, scale, toff, _) in batch {
          update_tex_coord_info(&mut vbo_data, &mut vbo_i, **toff);
          update_mv_mat(&mut vbo_data, &mut vbo_i, &view, **pos, **rot, **scale);
        }
        // println!("VBO Data {:?}", vbo_data);
        // shader.stop();
        r_update_vbo(vbo_id, vbo_data, max_inst * inst_data_len);
        // shader.start();
        // std::thread::sleep(std::time::Duration::from_millis(20));
        r_update_instanced_attrib(&data.pvbo);
        r_draw_instanced(quad.vertex_count, parts_count as u32);
        // r_draw_triangle_strip(quad.vertex_count);
      };
    }
    GlSettings::default()
        .enable_depth_mask()
        .disable_blend()
        .set();
    r_unbind_vaa_7();
    shader.stop();
  }
}

pub struct DrawParticlesNotInstanced;
impl<'a> System<'a> for DrawParticlesNotInstanced {
  type SystemData = DrawParticlesData<'a>;
  fn run(&mut self, data: Self::SystemData) {
    let shader = &data.shader;
    let quad = data.pvbo.quad;
    let vbo_id = data.pvbo.vbo_id;
    let max_inst = data.pvbo.max_instances;
    let inst_data_len = data.pvbo.instance_data_length;
    // println!("{:?} {:?}", quad, vbo_id);
    
    // todo: rewrite not instanced and try to work out instancing later.
    let mut _transform: Matrix4f<f32> = Matrix4f::new();
    let d = data.entities();
    if d.is_empty() { return }
    let view = DISPLAY.lock().unwrap().camera.view_mat.clone();
    // println!("{:?}", view);
    shader.start();
    r_bind_vaa_7(quad.vao_id);
    GlSettings::default()
        .disable_depth_mask()
        .enable_blend()
        .set();
    RBlend::AdditiveBlend.exec();
    for tex in d.keys() {
      if let Some(batch) = d.get(tex) {
        let texture: &Texture = &data.textures.0.get(tex)
            .unwrap_or_else(|| panic!("DrawParticles: No such Texture :{}", tex));
        r_bind_texture(texture);
        shader.load_float("rowCount", 4.0);
        let mut vbo_i = 0;
        let parts_count = batch.len();
        let parts_size = parts_count * inst_data_len;
        let mut vbo_data = vec!(0_f32; parts_size);
        // ( Entity, &TexName, &CamDistance, 
        //   &Position, &Rotation, &ScaleFloat, &TexOffsets, 
        //   &ParticleAlive,)
        for (_, _, _, pos, rot, scale, toff, _) in batch {
          update_tex_coord_info(&mut vbo_data, &mut vbo_i, **toff);
          update_mv_mat(&mut vbo_data, &mut vbo_i, &view, **pos, **rot, **scale);
        }
        // println!("VBO Data {:?}", vbo_data);
        // shader.stop();
        r_update_vbo(vbo_id, vbo_data, max_inst * inst_data_len);
        // shader.start();
        // std::thread::sleep(std::time::Duration::from_millis(20));
        r_update_instanced_attrib(&data.pvbo);
        r_draw_instanced(quad.vertex_count, parts_count as u32);
        // r_draw_triangle_strip(quad.vertex_count);
      };
    }
    GlSettings::default()
        .enable_depth_mask()
        .disable_blend()
        .set();
    r_unbind_vaa_7();
    shader.stop();
  }
}

fn update_tex_coord_info(vbo_data: &mut Vec<f32>, vbo_i: &mut usize, tex_offsets: TexOffsets) {
  vbo_data[*vbo_i] = tex_offsets.blend; *vbo_i += 1;
  vbo_data[*vbo_i] = tex_offsets.a.x; *vbo_i += 1;
  vbo_data[*vbo_i] = tex_offsets.a.y; *vbo_i += 1;
  vbo_data[*vbo_i] = tex_offsets.b.x; *vbo_i += 1;
  vbo_data[*vbo_i] = tex_offsets.b.y; *vbo_i += 1;
}
fn update_mv_mat(
  vbo_data: &mut Vec<f32>, 
  vbo_i: &mut usize, 
  view: &Matrix4f<f32>, 
  pos: Position, 
  rot: Rotation, 
  scale: ScaleFloat, 
) {
  let mut model: Matrix4f<f32> = Matrix4f::new();
  model.translate_v3f(pos.0);
  model.transpose3x3(&view);
  model.rotate(rot.0.z.to_radians(), crate::constants::ZVEC);
  model.scale(Vector3f::new(scale.0, scale.0, scale.0));
  let mv_mat = *view * model;
  // mv_mat.transpose();
  for n in mv_mat.matrix.iter() {
    vbo_data[*vbo_i] = *n;
    *vbo_i += 1;
  }
}

#[derive(SystemData)]
pub struct DrawParticlesData<'a> {
  pub shader:      Read<'a, ParticleShader>, 
  pub pvbo:        Read<'a, ParticleVBO>,
  pub models:      Read<'a, Models>, 
  pub textures:    Read<'a, Textures>, 
  pub lightings:   Read<'a, Lightings>, 
  pub ents:        Entities<'a>,
  pub pos:         ReadStorage<'a, Position>,
  pub rot:         ReadStorage<'a, Rotation>,
  pub scale:       ReadStorage<'a, ScaleFloat>,
  pub model:       ReadStorage<'a, ModelName>,
  pub texture:     ReadStorage<'a, TexName>,
  pub lighting:    ReadStorage<'a, LightingName>,
  pub particle:    ReadStorage<'a, ParticleAlive>,
  pub cam_dist:    ReadStorage<'a, CamDistance>,
  pub tex_offsets: ReadStorage<'a, TexOffsets>,
}
impl<'a> DrawParticlesData<'a> {
  pub fn entities(&self) -> HashMap<String, Vec<(
      Entity, &TexName, &CamDistance, 
      &Position, &Rotation, &ScaleFloat, &TexOffsets, 
      &ParticleAlive, 
  )>> {
    let mut d = (
      &self.ents, &self.texture, &self.cam_dist,
      &self.pos, &self.rot, &self.scale, &self.tex_offsets, 
      &self.particle, ).join().collect::<Vec<_>>();
    d.sort_by(|&a,&b| {
      match a.1 .0.cmp(&b.1 .0) {
        Ordering::Equal => {
          b.2 .0.partial_cmp(&a.2 .0).expect(
              "A wild NAN has appearded in a CamDistance")
        }
        x => { x }
      }
    });
    let mut out: HashMap<String, Vec<_>> = HashMap::new();
    for e in d {
      if let Some(x) = out.get_mut(&e.1 .0) {
        x.push(e);
      } else {
        out.insert(e.1 .0.to_owned(), vec!(e));
      }
    }
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
      offsets.update(*life, rows.0);
    }
    for e in dead {
      data.alive.remove(e);
    }
  }
}
#[derive(SystemData)]
pub struct UpdateParticlesData<'a> {
  pub handler:      Read<'a, Handler>, 
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
    DISPLAY.lock().unwrap().camera.pos
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
