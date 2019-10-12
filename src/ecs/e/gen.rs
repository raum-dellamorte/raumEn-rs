
use {
  specs::{
    Entities, System,
    // SystemData, 
    // Read, Write, ReadStorage, WriteStorage, 
  },
  ecs::c::{
    material::{
      LightingComponent, 
      ModelComponent,
      TextureComponent, 
      // TexIndexComponent, 
      // RowCountComponent, 
      // OffsetComponent, 
      // MultiTexComponent,
    },
    position::*,
    terrain::{
      Platform,
      TerrainNode,
      TerrainNodes,
    },
  },
  flags::*,
  util::{
    // rgl::*, 
    // HashSet,
    // HashMap, 
    Vector3f,
    // Matrix4f,
  }
};

use noise::NoiseFn;
use noise::Fbm;
use noise::Point2;
use noise::Seedable;
use specs::*;

const NODE_SIZE: i32 = 50;

pub struct LandscapeGen {
  pub landscape: noise::Fbm,
  pub l_weight: f32,
  pub l_mult: i32,
  pub holes: noise::Fbm,
}
impl Default for LandscapeGen {
  fn default() -> Self {
    LandscapeGen {
      landscape: Fbm::new().set_seed(186_074_u32),
      l_weight: 0_f32,
      l_mult: 0_i32,
      holes: Fbm::new().set_seed(441_952_u32),
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
    
    let (px,pz) = (node_group(player.0),node_group(player.1));
    for ix in &[-1, 0, 1] { for iz in &[-1, 0, 1] {
      let node = TerrainNode(px + ix, pz + iz);
      if !nodes.0.contains(&node) {
        let (nx,nz) = (node.0 * NODE_SIZE, node.1 * NODE_SIZE);
        for x_loc in 0..NODE_SIZE { for z_loc in 0..NODE_SIZE { 
          let (x, z) = ((nx + x_loc) * 2, (nz + z_loc) * 2);
          let ent = ents.create();
          let hpt = point(x, z, 6);
          let hole: bool = landgen.holes.get(hpt) < -0.2_f64;
          if !hole {
            let pt = point(x, z, 8);
            let mut top: f64 = landgen.landscape.get(pt);
            top = (top + 1.0) / 2.0;
            // top = (top + (landgen.l_weight * landgen.l_mult as f32) as f64) / (landgen.l_mult + 1) as f64;
            let pf = Platform::new(200.0, -100.0, x, z, top as f32, 0.05);
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

fn node_group(x: i32) -> i32 { (if x < 0 { -(NODE_SIZE - 1) + x } else { x }) / NODE_SIZE }

fn point(x: i32, z: i32, precision: u32) -> Point2<f64> {
  let p = 2_isize.pow(precision) as f64;
  let x = f64::from(x) / p;
  let z = f64::from(z) / p;
  let pt: Point2<f64> = [x, z];
  pt
}

pub struct PlayerGen;
impl<'a> System<'a> for PlayerGen {
  type SystemData = (
    // Read<'a, >,
    // Write<'a, >,
    Read<'a, PlayerLoc>,
    Entities<'a>,
    WriteStorage<'a, Position>,
    WriteStorage<'a, Rotation>,
    WriteStorage<'a, Velocity>,
    WriteStorage<'a, TransformVelocity>,
    WriteStorage<'a, DeltaVelocity>,
    WriteStorage<'a, PosAdjust>,
    WriteStorage<'a, ModelComponent>,
    WriteStorage<'a, TextureComponent>,
    WriteStorage<'a, LightingComponent>,
    WriteStorage<'a, JumpArc>,
    WriteStorage<'a, ActivePlayer>,
    WriteStorage<'a, Falling>,
    WriteStorage<'a, IsTexMod>,
  );
  fn run(&mut self, _data: Self::SystemData) {
    let ( ploc, ents, mut pos, mut rot, 
          mut vel, mut transvel, mut dvel, mut tvel, 
          mut mod_c, mut tex_c, mut ltg_c, mut jump_arcs,
          mut player, mut fall, mut texmod) = _data;
    let ent = ents.create();
    
    pos.insert(ent, Position {0: Vector3f {x: ploc.0 as f32, y: 20.0, z: ploc.1 as f32}}).expect("Failed to insert new Position");
    rot.insert(ent, Rotation {0: Vector3f::blank()}).expect("Failed to insert new Rotation");
    transvel.insert(ent, TransformVelocity {0: Vector3f::blank()}).expect("Failed to insert new TransformVelocity");
    vel.insert(ent, Velocity {0: Vector3f::blank()}).expect("Failed to insert new Velocity");
    dvel.insert(ent, DeltaVelocity {0: Vector3f::blank()}).expect("Failed to insert new DeltaVelocity");
    tvel.insert(ent, PosAdjust {0: Vector3f::blank()}).expect("Failed to insert new PosAdjust");
    mod_c.insert(ent, ModelComponent("player".to_owned())).expect("Failed to insert new ModelComponent");
    tex_c.insert(ent, TextureComponent("dirt".to_owned())).expect("Failed to insert new TextureComponent");
    ltg_c.insert(ent, LightingComponent("flat".to_owned())).expect("Failed to insert new LightingComponent");
    jump_arcs.insert(ent, JumpArc::new()).expect("Failed to insert new JumpArc");
    player.insert(ent, ActivePlayer).expect("Failed to insert flag ActivePlayer");
    fall.insert(ent, Falling).expect("Failed to insert flag Falling");
    texmod.insert(ent, IsTexMod).expect("Failed to insert flag IsTexMod");
    
    println!("Mobs made");
  }
}

pub struct ParticleGen;
impl<'a> System<'a> for ParticleGen {
  type SystemData = (
    // Read<'a, >,
    // Write<'a, >,
    Entities<'a>,
    // WriteStorage<'a, Position>,
    // WriteStorage<'a, Velocity>,
    // WriteStorage<'a, TransformVelocity>,
    // WriteStorage<'a, DeltaVelocity>,
    // WriteStorage<'a, PosAdjust>,
    // WriteStorage<'a, ModelComponent>,
    // WriteStorage<'a, TextureComponent>,
    // WriteStorage<'a, LightingComponent>,
    // WriteStorage<'a, JumpArc>,
    // WriteStorage<'a, ActivePlayer>,
    // WriteStorage<'a, Falling>,
    // WriteStorage<'a, IsTexMod>,
  );
  fn run(&mut self, _data: Self::SystemData) {
    // let ( ents, mut pos, 
    //       mut vel, mut transvel, mut dvel, mut tvel, 
    //       mut mod_c, mut tex_c, mut ltg_c, mut jump_arcs,
    //       mut player, mut fall, mut texmod) = _data;
    // let ent = ents.create();
    
    // pos.insert(ent, Position {0: Vector3f {x: ploc.0 as f32, y: 20.0, z: ploc.1 as f32}}).expect("Failed to insert new Position");
    // transvel.insert(ent, TransformVelocity {0: Vector3f::blank()}).expect("Failed to insert new TransformVelocity");
    // vel.insert(ent, Velocity {0: Vector3f::blank()}).expect("Failed to insert new Velocity");
    // dvel.insert(ent, DeltaVelocity {0: Vector3f::blank()}).expect("Failed to insert new DeltaVelocity");
    // tvel.insert(ent, PosAdjust {0: Vector3f::blank()}).expect("Failed to insert new PosAdjust");
    // mod_c.insert(ent, ModelComponent("player".to_owned())).expect("Failed to insert new ModelComponent");
    // tex_c.insert(ent, TextureComponent("dirt".to_owned())).expect("Failed to insert new TextureComponent");
    // ltg_c.insert(ent, LightingComponent("flat".to_owned())).expect("Failed to insert new LightingComponent");
    // jump_arcs.insert(ent, JumpArc::new()).expect("Failed to insert new JumpArc");
    // player.insert(ent, ActivePlayer).expect("Failed to insert flag ActivePlayer");
    // fall.insert(ent, Falling).expect("Failed to insert flag Falling");
    // texmod.insert(ent, IsTexMod).expect("Failed to insert flag IsTexMod");
    
    // println!("Mobs made");
  }
}

pub struct EntityGen;
impl<'a> System<'a> for EntityGen {
  type SystemData = (
    // Read<'a, >,
    // Write<'a, >,
    // Read<'a, PlayerLoc>,
    Entities<'a>,
    // WriteStorage<'a, Position>,
    // WriteStorage<'a, ModelComponent>,
    // WriteStorage<'a, TextureComponent>,
    // WriteStorage<'a, LightingComponent>,
  );
  fn run(&mut self, _data: Self::SystemData) {
    
    println!("Mobs made");
  }
}
