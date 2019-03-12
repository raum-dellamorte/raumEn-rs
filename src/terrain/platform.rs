
use specs::*;
use specs::prelude::*;

use TerrainShader;
use model::{
  Models, 
  ModelComponent,
};
use material::{
  lighting::Lightings, 
  material::MaterialData, 
  texture::Textures, 
  TextureComponent, 
  LightingComponent, 
  TexIndexComponent, 
  RowCountComponent, 
  OffsetComponent, 
  MultiTexComponent,
};
use util::{
  rgl::*, 
  HashMap, 
  Vector3f,
  Matrix4f,
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Platform {
  x: i32,
  z: i32,
  h: f32,
  d: f32,
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

#[derive(SystemData)]
pub struct PlatformData<'a> {
  platform: ReadStorage<'a, Platform>,
  model: ReadStorage<'a, ModelComponent>,
  texture: ReadStorage<'a, TextureComponent>,
  lighting: ReadStorage<'a, LightingComponent>,
  tex_index: ReadStorage<'a, TexIndexComponent>,
  row_count: ReadStorage<'a, RowCountComponent>,
  offset: ReadStorage<'a, OffsetComponent>,
  multi_tex: ReadStorage<'a, MultiTexComponent>,
}

use noise::NoiseFn;
use noise::Fbm;
use noise::Point2;
use noise::Seedable;
pub struct LandscapeGen {
  pub landscape: noise::Fbm,
  pub l_weight: f32,
  pub l_mult: i32,
  pub holes: noise::Fbm,
}
impl Default for LandscapeGen {
  fn default() -> Self {
    LandscapeGen {
      landscape: Fbm::new().set_seed(186074_u32),
      l_weight: 0_f32,
      l_mult: 0_i32,
      holes: Fbm::new().set_seed(441952_u32),
    }
  }
}
pub struct PlatformGen;
impl<'a> System<'a> for PlatformGen {
  type SystemData = (
    Read<'a, LandscapeGen>,
    Entities<'a>,
    WriteStorage<'a, Platform>,
    WriteStorage<'a, ModelComponent>,
    WriteStorage<'a, TextureComponent>,
    WriteStorage<'a, LightingComponent>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (landgen, ents, mut platforms, mut models, mut textures, mut lightings) = data;
    // Need to know where I am and what "Node" I am nearest, what "Node"s haven't been generated near me
    {
      for x_loc in 0..1000_i32 { for z_loc in 0..1000_i32 { 
        let (x, z) = ((x_loc - 500) * 2, (z_loc - 500) * 2);
        let ent = ents.create();
        let hpt = point(x, z, 6);
        let hole: bool = landgen.holes.get(hpt) < -0.2_f64;
        if !hole {
          let pt = point(x, z, 8);
          let mut top: f64 = landgen.landscape.get(pt);
          top = (top + 1.0) / 2.0;
          // top = (top + (landgen.l_weight * landgen.l_mult as f32) as f64) / (landgen.l_mult + 1) as f64;
          let pf = Platform { x: x, z: z, h: top as f32, d: 0.05 };
          platforms.insert(ent, pf).expect("Failed to insert new Platform");
          models.insert(ent, ModelComponent("platform".to_owned())).expect("Failed to insert new ModelComponent");
          textures.insert(ent, TextureComponent("dirt".to_owned())).expect("Failed to insert new TextureComponent");
          lightings.insert(ent, LightingComponent("flat".to_owned())).expect("Failed to insert new LightingComponent");
        }
      }
    }}
  }
}

fn point(x: i32, z: i32, precision: u32) -> Point2<f64> {
  let p = 2_isize.pow(precision) as f64;
  let x = x as f64 / p;
  let z = z as f64 / p;
  let pt: Point2<f64> = [x, z];
  pt
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
    for (platform, model, texture, lighting) in (&platform, &model, &texture, &lighting).join() {
      let mut tex = match draw.index_of(&model.0) {
        _n if _n < 0 => {
          draw.push(&texture.0)
        }
        n => {
          &mut draw.0[n as usize]
        }
      };
      let mut attribs = match tex.index_of(&texture.0, &lighting.0) {
        _n if _n < 0 => {
          tex.push(&texture.0, &lighting.0)
        }
        n => {
          &mut tex.1[n as usize]
        }
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
  }
}
