
use {
  material::{
    // lighting::Lightings, 
    // material::MaterialData, 
    // texture::Textures, 
    LightingComponent, 
    ModelComponent,
    TextureComponent, 
    // TexIndexComponent, 
    // RowCountComponent, 
    // OffsetComponent, 
    // MultiTexComponent,
  },
  terrain::*,
  // util::{
  //   rgl::*, 
  //   HashSet,
  //   HashMap, 
  //   Vector3f,
  //   Matrix4f,
  // }
};

use noise::NoiseFn;
use noise::Fbm;
use noise::Point2;
use noise::Seedable;
use specs::*;

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
    Write<'a, TerrainNodes>,
    Read<'a, PlayerLoc>,
    Entities<'a>,
    WriteStorage<'a, Platform>,
    WriteStorage<'a, ModelComponent>,
    WriteStorage<'a, TextureComponent>,
    WriteStorage<'a, LightingComponent>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (landgen, mut nodes, player, ents, mut platforms, mut models, mut textures, mut lightings) = data;
    // Need to know where I am and what "Node" I am nearest, what "Node"s haven't been generated near me
    
    let (px,pz) = (thousands_group(player.0),thousands_group(player.1));
    for ix in vec![-1, 0, 1] { for iz in vec![-1, 0, 1] {
      let node = TerrainNode(px + ix, pz + iz);
      if !nodes.0.contains(&node) {
        let (nx,nz) = (node.0 * 1000, node.1 * 1000);
        for x_loc in 0..1000_i32 { for z_loc in 0..1000_i32 { 
          let (x, z) = ((nx + x_loc) * 2, (nz + z_loc) * 2);
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
        }}
        nodes.0.insert(node);
      }
    }}
    println!("Platforms made");
  }
}

fn thousands_group(x: i32) -> i32 { (if x < 0 { -999 + x } else { x }) / 1000 }

fn point(x: i32, z: i32, precision: u32) -> Point2<f64> {
  let p = 2_isize.pow(precision) as f64;
  let x = x as f64 / p;
  let z = z as f64 / p;
  let pt: Point2<f64> = [x, z];
  pt
}
