#![allow(dead_code)]

use {
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
  pub fn pos(&self, wh: f32, base: f32) -> Vector3f {
    let y = ((wh * self.h) - (wh * self.d)) + base;
    Vector3f::new(self.x as f32, y, self.z as f32)
  }
  pub fn scale(&self, wh: f32) -> Vector3f {
    Vector3f::new(1.0, wh * self.d, 1.0)
  }
}

pub struct DrawPlatformPrep;
impl<'a> System<'a> for DrawPlatformPrep {
  type SystemData = (
    Write<'a, DrawModelsWithTextures>, 
    ReadStorage<'a, Platform>,
    ReadStorage<'a, ModelComponent>,
    ReadStorage<'a, TextureComponent>,
    ReadStorage<'a, LightingComponent>,
  );
  fn run(&mut self, data: Self::SystemData) {
    use specs::Join;
    let (mut draw, platform, model, texture, lighting) = data;
    draw.clear();
    let mut count = 0;
    for (platform, model, texture, lighting) in (&platform, &model, &texture, &lighting).join() {
      count += 1;
      if count > 9000000 {println!("It's over 9000000!");}
      let mut tex = match draw.index_of(&model.0) {
        _n if _n < 0 => { draw.push(&texture.0) }
        n => { &mut draw.0[n as usize] }
      };
      let mut attribs = match tex.index_of(&texture.0, &lighting.0) {
        _n if _n < 0 => { tex.push(&texture.0, &lighting.0) }
        n => { &mut tex.1[n as usize] }
      };
      let mut transform = Matrix4f::new();
      transform.set_identity();
      transform.translate_v3f(&platform.pos(200.0, 100.0));
      transform.scale(&platform.scale(200.0));
      let attrib = ModelTextureAttribs {
        transform: transform,
        tex_index: None,
        row_count: None,
        offset: None,
        multi_tex: None,
      };
      attribs.push(attrib);
    }
    println!("Platforms Prepped");
  }
}
pub struct DrawPlatform;
impl<'a> System<'a> for DrawPlatform {
  type SystemData = (
    Read<'a, TerrainShader>, 
    Read<'a, Models>, 
    Read<'a, Textures>, 
    Read<'a, Lightings>, 
    Read<'a, DrawModelsWithTextures>
  );
  fn run(&mut self, (shader, models, textures, lightings, data): Self::SystemData) {
    let shader = &shader.shader;
    
    shader.start();
    for draw_model in &data.0 {
      if let Some(ref model) = models.0.get(&draw_model.0) {
        r_bind_vaa_3(model);
        for draw_material in &draw_model.1 {
          if let (Some(ref texture), Some(ref lighting)) = (textures.0.get(&draw_material.0), lightings.0.get(&draw_material.1)) {
            r_bind_texture(texture);
            lighting.load_to_shader(shader);
            for attrib in &draw_material.2 {
              shader.load_matrix("u_Transform", &attrib.transform);
              r_draw_triangles(model);
            }
          }
        }
      }
    }
    r_unbind_vaa_3();
    shader.stop();
    println!("Platforms Drawn");
  }
}
