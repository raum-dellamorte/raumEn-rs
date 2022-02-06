
use {
  std::cmp::Ordering,
  
  specs::{
    System, Read, ReadStorage, Entities, Join, 
  },
  crate::{
    DISPLAY,
    ecs::{
      c::{
        flags::*,
        components::*,
      },
      resource::*,
    },
    shader::TexModShader,
    util::{
      Matrix4f, 
      // Vector3f,
      rgl::*,
    },
  },
};

pub struct DrawTexMods;
impl<'a> System<'a> for DrawTexMods {
  type SystemData = (
    (
      Read<'a, TexModShader>, 
      Read<'a, Models>, 
      Read<'a, Textures>, 
      Read<'a, Lightings>, 
    ),
    (
      Entities<'a>,
      ReadStorage<'a, Position>,
      ReadStorage<'a, Rotation>,
      ReadStorage<'a, ModelName>,
      ReadStorage<'a, TexName>,
      ReadStorage<'a, LightingName>,
      ReadStorage<'a, IsTexMod>,
    ),
  );
  fn run(&mut self, data: Self::SystemData) {
    let (shader, models, textures, lightings) = data.0;
    let shader = &shader.shader;
    let mut transform = Matrix4f::new();
    let (e, pos, rot, mc, tc, lc, is_tex_mod) = data.1;
    let _data = (&e, &pos, &rot, &mc, &tc, &lc, &is_tex_mod);
    let mut d = _data.join().collect::<Vec<_>>();
    if d.is_empty() { return }
    d.sort_by(|&a,&b| {
      match a.3 .0 .cmp(&b.3 .0) { // .3 is ModelName; .0 is the internal String
        Ordering::Equal => {
          a.4 .0 .cmp(&b.4 .0) // .4 is TexName; .0 is the internal String
        }
        x => { x }
      }
    });
    let mut last_model = &d[0] .3 .0;
    let mut last_texture = &d[0] .4 .0;
    let mut model: &Model = &models.0.get(last_model)
        .unwrap_or_else(|| panic!("DrawTexMods: No such Model :{}", last_model));
    let mut texture: &Texture = &textures.0.get(last_texture)
        .unwrap_or_else(|| panic!("DrawTexMods: No such Texture :{}", last_texture));
    shader.start();
    {
      let view = DISPLAY.lock().unwrap().camera.view_mat;
      shader.load_matrix("u_View", &view);
    }
    // shader.load_vec_3f("light_pos", &(*light).pos); // Unimplemented
    // shader.load_vec_3f("light_color", &(*light).color);
    r_bind_vaa_3(model.vao_id);
    r_bind_texture(texture);
    for (_, p, r, m, t, l, _) in d {
      if m.0 != *last_model {
        model = &models.0.get(&m.0).unwrap();
        last_model = &m.0;
        r_bind_vaa_3(model.vao_id);
      }
      if t.0 != *last_texture {
        texture = &textures.0.get(&t.0).unwrap();
        last_texture = &t.0;
        r_bind_texture(texture);
      }
      if let Some(ref lighting) = lightings.0.get(&l.0) {
        lighting.load_to_shader(shader);
      }
      transform.set_identity();
      transform.translate_v3f(p.0);
      transform.rotate(r.0.y.to_radians(), crate::constants::YVEC);
      // transform.scale(&p.scale(200.0));
      shader.load_matrix("u_Transform", &transform);
      r_draw_triangles(model.vertex_count);
    }
    r_unbind_vaa_3();
    shader.stop();
  }
}

