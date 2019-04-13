#![allow(dead_code)]

use {
  std::cmp::Ordering,
  specs::{
    *,
    // prelude::*,
  },
  TerrainShader,
  flags::InScene,
  material::{
    Model,
    Models, 
    ModelComponent,
    lighting::Lightings, 
    // material::MaterialData, 
    Texture,
    texture::Textures, 
    TextureComponent, 
    LightingComponent, 
    // TexIndexComponent, 
    // RowCountComponent, 
    // OffsetComponent, 
    // MultiTexComponent,
  },
  util::{
    rgl::*, 
    // HashSet,
    // HashMap, 
    Vector3f,
    Matrix4f,
  },
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Platform {
  pub x: i32,
  pub z: i32,
  pub h: f32,
  pub d: f32,
}
impl Platform {
  pub fn pos(&self, _wh: f32, _base: f32) -> Vector3f {
    // let y = ((wh * self.h) - (wh * self.d)) + base;
    Vector3f::new(self.x as f32, 0.0, self.z as f32)
  }
  pub fn scale(&self, wh: f32) -> Vector3f {
    Vector3f::new(1.0, wh * self.d, 1.0)
  }
}


pub struct DrawPlatform;
impl<'a> System<'a> for DrawPlatform {
  type SystemData = (
    (
      Read<'a, TerrainShader>, 
      Read<'a, Models>, 
      Read<'a, Textures>, 
      Read<'a, Lightings>, 
    ),
    (
      Entities<'a>,
      ReadStorage<'a, Platform>,
      ReadStorage<'a, ModelComponent>,
      ReadStorage<'a, TextureComponent>,
      ReadStorage<'a, LightingComponent>
      // ReadStorage<'a, InScene>,
    ),
  );
  fn run(&mut self, data: Self::SystemData) {
    let (shader, models, textures, lightings) = data.0;
    let shader = &shader.shader;
    let mut transform = Matrix4f::new();
    let _data = (&(data.1).0, &(data.1).1, &(data.1).2, &(data.1).3, &(data.1).4);
    let mut d = _data.join().collect::<Vec<_>>();
    d.sort_by(|&a,&b| {
      match a.2 .0 .cmp(&b.2 .0) {
        Ordering::Equal => {
          a.3 .0 .cmp(&b.3 .0) // When I want to sort by draw distance and whatnot
        }
        x => { x }
      }
    });
    let mut last_model = "platform";
    let mut last_texture = "dirt";
    let mut model: &Model = &models.0.get("platform").unwrap();
    let mut texture: &Texture = &textures.0.get("dirt").unwrap();
    println!("platform {}, dirt {}", model.vao_id.0, texture.tex_id.0);
    shader.start();
    r_bind_vaa_3(model);
    r_bind_texture(texture);
    for (_, p, m, t, l) in &d {
      if m.0 != last_model {
        model = &models.0.get(&m.0).unwrap();
        last_model = &m.0;
        r_bind_vaa_3(model);
      }
      if t.0 != last_texture {
        texture = &textures.0.get(&t.0).unwrap();
        last_texture = &t.0;
        r_bind_texture(texture);
      }
      if let Some(ref lighting) = lightings.0.get(&l.0) {
        lighting.load_to_shader(shader);
      }
      transform.set_identity();
      transform.translate_v3f(&p.pos(200.0, 100.0));
      transform.scale(&p.scale(200.0));
      if p.x.abs() < 3 && p.z.abs() < 3 {
        println!("{:?}", transform);
      }
      shader.load_matrix("u_Transform", &transform);
      r_draw_triangles(model);
    }
    r_unbind_vaa_3();
    shader.stop();
    println!("Platforms Drawn");
  }
}
