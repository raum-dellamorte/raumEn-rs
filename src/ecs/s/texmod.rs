
use {
  std::cmp::Ordering,
  
  util::{
    Matrix4f, 
  },
  specs::{
    System, Read, ReadStorage, Entities, Join, 
  },
  ecs::{
    c::{
      lighting::Lightings,
      model::{
        Models, Model,
      },
      texture::{
        Textures,
        Texture,
      },
      material::{
        LightingComponent, ModelComponent, TextureComponent,
      },
      position::Position,
    },
  },
  flags::*,
  util::{
    // Vector3f,
    rgl::*,
  },
  
  ViewMatrix,
  shader::TexModShader,
};

pub struct DrawTexMods;
impl<'a> System<'a> for DrawTexMods {
  type SystemData = (
    (
      Read<'a, TexModShader>, 
      Read<'a, ViewMatrix>,
      Read<'a, Models>, 
      Read<'a, Textures>, 
      Read<'a, Lightings>, 
    ),
    (
      Entities<'a>,
      ReadStorage<'a, Position>,
      ReadStorage<'a, ModelComponent>,
      ReadStorage<'a, TextureComponent>,
      ReadStorage<'a, LightingComponent>,
      ReadStorage<'a, IsTexMod>,
    ),
  );
  fn run(&mut self, data: Self::SystemData) {
    let (shader, view, models, textures, lightings) = data.0;
    let shader = &shader.shader;
    let mut transform = Matrix4f::new();
    let _data = (&(data.1).0, &(data.1).1, &(data.1).2, &(data.1).3, &(data.1).4, &(data.1).5);
    let mut d = _data.join().collect::<Vec<_>>();
    if d.is_empty() { return }
    d.sort_by(|&a,&b| {
      match a.2 .0 .cmp(&b.2 .0) { // .2 is ModelComponent; .0 is the internal String
        Ordering::Equal => {
          a.3 .0 .cmp(&b.3 .0) // .3 is TextureComponent; .0 is the internal String
        }
        x => { x }
      }
    });
    let mut last_model = &d[0] .2 .0;
    let mut last_texture = &d[0] .3 .0;
    let mut model: &Model = &models.0.get(last_model)
        .unwrap_or_else(|| panic!("DrawTexMods: No such Model :{}", last_model));
    let mut texture: &Texture = &textures.0.get(last_texture)
        .unwrap_or_else(|| panic!("DrawTexMods: No such Texture :{}", last_texture));
    shader.start();
    shader.load_matrix("u_View", &(*view).view);
    // shader.load_vec_3f("light_pos", &(*light).pos); // Unimplemented
    // shader.load_vec_3f("light_color", &(*light).color);
    r_bind_vaa_3(model);
    r_bind_texture(texture);
    for (_, p, m, t, l, _) in &d {
      if m.0 != *last_model {
        model = &models.0.get(&m.0).unwrap();
        last_model = &m.0;
        r_bind_vaa_3(model);
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
      transform.translate_v3f(&p.pos);
      // transform.scale(&p.scale(200.0));
      shader.load_matrix("u_Transform", &transform);
      r_draw_triangles(model);
    }
    r_unbind_vaa_3();
    shader.stop();
  }
}

