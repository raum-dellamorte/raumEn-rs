#![recursion_limit="128"]
#![allow(clippy::type_complexity, clippy::module_inception, clippy::too_many_arguments, clippy::len_without_is_empty)]
#![feature(proc_macro_hygiene)]
// #![feature(nightly)]
// #![allow(unused_imports,dead_code)]

extern crate gl;
extern crate glutin;
extern crate glsl;
extern crate glsl_quasiquote;
// extern crate cheddar; 
#[macro_use] extern crate nom;
extern crate image;
extern crate num;
extern crate nalgebra;
extern crate ncollide3d;
extern crate time;
extern crate rand;
extern crate noise;
extern crate specs;
#[macro_use] extern crate specs_derive;
// extern crate shred;
// #[macro_use]
// extern crate shred_derive;

// in project stuff
pub mod ecs;
pub mod engine;
pub mod entities;
pub mod importobj;
pub mod render;
pub mod shader;
pub mod text;
pub mod util;

use {
  gl::*,
  // glutin::{
  //   // dpi::*,
  //   ContextCurrentState,
  // },
  specs::{
    // Builder, Component, ReadStorage, WriteStorage, System, VecStorage, RunNow,
    DispatcherBuilder, 
    World, WorldExt, 
  },
  crate::{
    ecs::{
      c::{
        flags::*,
        components::*,
      },
      resource::*,
      helper::{
        particle::*,
        gen::{
          LandscapeGen,
          PlatformGen,
          PlayerGen,
        },
      },
      s::{
        camera::{
          CameraToActivePlayer,
        },
        particle::*,
        position::*,
        terrain::{
          DrawPlatform,
        },
        texmod::{
          DrawTexMods,
        },
      },
    },
    shader::{
      Shader,
      ParticleShader,
      TerrainShader,
      TexModShader,
    },
    util::Vector3f,
    // util::rgl::*,
    // entities::{
    //   EntityMgr,
    //   // Mob,
    // },
  },
};

pub use {
  crate::{
    ecs::resource::Model,
    engine::{
      Camera, Display, Fbo, HUD, GuiObj, Handler, Loader
    },
    text::TextMgr,
    render::{
      RenderMgr, RenderPostProc,
    },
  }
};

use engine::fbo::ColorType::{
  // ColorMultisampleRenderBuffer, 
  ColorMultisampleRenderBuffers2, 
  ColorTexture, 
  // NoColor, 
};
use engine::fbo::DepthType::{
  DepthRenderBuffer, 
  DepthTexture, 
  // NoDepth, 
};

fn gen_world() -> World {
  let mut world = World::new();
  world.insert(Camera::default());
  world.insert(Display::default());
  // world.insert(DrawModelsWithTextures::default());
  world.insert(Handler::default());
  world.insert(LandscapeGen::default());
  world.insert(Lightings::default());
  world.insert(Lights::default());
  world.insert(Loader::default());
  world.insert(Models::default());
  world.insert(ParticleShader::default());
  // world.insert(ParticleSystems::default());
  world.insert(PlayerGridLoc::default());
  world.insert(Rotators::default());
  world.insert(TerrainNodes::default());
  world.insert(TerrainShader::default());
  world.insert(TexModShader::default());
  world.insert(TextMgr::default());
  world.insert(Textures::default());
  world.insert(ViewMatrix::default());
  world.register::<ActivePlayer>();
  world.register::<CurrentNode>();
  world.register::<CamDistance>();
  world.register::<DeltaVelocity>();
  world.register::<Falling>();
  world.register::<GravPercent>();
  world.register::<JumpArc>();
  world.register::<InScene>();
  world.register::<IsPlatform>();
  world.register::<IsTexMod>();
  world.register::<LightingName>();
  world.register::<LocalToPlayer>();
  world.register::<ModelName>();
  world.register::<Moving>();
  world.register::<MultiTex>();
  world.register::<Particle>();
  world.register::<ParticleAlive>();
  world.register::<Platform>();
  world.register::<PosAdjust>();
  world.register::<Position>();
  world.register::<Rotation>();
  world.register::<Rotator<f32>>();
  world.register::<RowCount>();
  world.register::<ScaleFloat>();
  world.register::<StartMoving>();
  world.register::<TexAdditive>();
  world.register::<TexIndex>();
  world.register::<TexName>();
  world.register::<TexOffset>();
  world.register::<TexOffsets>();
  world.register::<TimedLife>();
  world.register::<TransformVelocity>();
  world.register::<Velocity>();
  {
    let mut lights = world.write_resource::<Lights>();
    lights.add_light();
    lights.lights[0].pos.copy_from_isize(0,500,-10);
  }
  ParticleVBO::default().init(&mut world);
  let quad = {
    let loader = world.write_resource::<Loader>();
    loader.quad_1_0
  };
  world.insert(HUD::new(quad));
  world
}

fn main() {
  // // Test code for parsing fnt files
  // use text::metafile::test_noms;
  // test_noms();
  
  // Create a window
  let mut el = glutin::EventsLoop::new();
  let wb = glutin::WindowBuilder::new()
    .with_title("RaumEn")
    .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0))
    .with_maximized(false);
  let windowed_context = glutin::ContextBuilder::new()
    .build_windowed(wb, &el)
    .unwrap();
  
  let windowed_context = unsafe { windowed_context.make_current().unwrap() };
  // Set up OpenGL
  unsafe {
    load_with(|symbol| windowed_context.context().get_proc_address(symbol) as *const _);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  
  // shader::terrain::test_terrain_cheddar();
  
  // Create the RenderMgr which will eventually be depricated
  // in favor of specs Systems, provided this whole specs expirement
  // ends with me figuring out how not to have a huge slowdown when
  // thousands of things are on screen and whatnot.
  let mut render_mgr = RenderMgr::new();
  
  // time keeping
  let mut fps: (f32, f32);
  let mut sec = 0.0;
  
  // Set up the world, which holds all the things,
  // and will be passed around...
  //     ...like your mom at a frat house.
  // I'm sorry, that was uncalled for.
  let mut world = gen_world();
  
  { // Here, we're getting the size of the window in pixels
    // and passing it to the render manager. It in turn
    // updates the Projection Matrix and passes that to 
    // ALL THE SHADERS, so if you add a SHADER, you need
    // to REMEMBER to add that shader to the update_size()
    // method in the RenderMgr.  There needs to be a better
    // way to do that.
    let dpi = windowed_context.window().get_hidpi_factor();
    let size = windowed_context.window().get_inner_size().unwrap().to_physical(dpi);
    render_mgr.update_size(&world, size.into());
  }
  { // loading models and textures
    let loader = &mut world.write_resource::<Loader>();
    let mut models = world.write_resource::<Models>();
    let mut textures = world.write_resource::<Textures>();
    let mut lightings = world.write_resource::<Lightings>();
    models.load_models(loader, &["platform", "player", "spaceship"]);
    textures.load_textures(loader, &["dirt", "spaceship", "cosmic"]);
    lightings.new_lighting_default("flat");
  }
  { // loading fonts and text
    let mut textmgr = world.write_resource::<TextMgr>();
    textmgr.add_font(&world, "pirate");
    textmgr.add_font(&world, "sans");
    textmgr.new_text(&world, "Title", "The Never", "pirate", 4.0, 0.0, 0.0, 1.0, true, true);
    textmgr.new_text(&world, "FPS", "FPS: 0.0", "sans", 1.5, 0.0, 0.0, 0.3, false, true);
  }
  
  // creating framebuffer objects 
  // the first one we render all the things to and pass
  // it to Post Production so we can make everything 
  // disappear into a black fog!  No need to write fog code into
  // every shader!  Just pass the first framebuffer, which has 
  // everything that's been drawn so far along with its depth 
  // information, to a single shader that blackens er'thing
  // based on its distance from the camera!
  // Couldn't get it to work in the Kotlin version.
  // I may go back to it and try to replicate how it's done here.
  let mut _fbo = Fbo::new(&world, 0, 0, ColorMultisampleRenderBuffers2, DepthRenderBuffer);
  let mut _fbo_final = Fbo::new(&world, 0, 0, ColorTexture, DepthTexture);
  let render_post = RenderPostProc::new("fog", world.read_resource::<HUD>().quad.vao_id.0, 
      vec![
        Texture::new("fbo color", _fbo_final.color_tex_id).assign_tex_unit(0_i32),
        Texture::new("fbo depth", _fbo_final.depth_tex_id).assign_tex_unit(1_i32),
      ]);
  { // Right now, the HUD is displaying a small version of the whole
    // screen so you can see the color and depth buffers for debugging
    // purposes.
    let mut _hud = world.write_resource::<HUD>();
    _hud.elements.push(GuiObj::new());
    let _gui = _hud.elements.get_mut(0).unwrap();
    _gui.tex_id = _fbo_final.color_tex_id;
    _gui.depth_tex_id = _fbo_final.depth_tex_id;
  }
  
  // Here we're getting back to specs stuff
  // Creating all the System dispatchers needed to control 
  // all the goings on and whatnot.
  let mut terrain_gen = DispatcherBuilder::new()
      .with_thread_local(PlatformGen)
      .build();
  terrain_gen.setup(&mut world);
  terrain_gen.dispatch(&world);
  
  world.maintain();
  
  let mut player_gen = DispatcherBuilder::new()
      .with_thread_local(PlayerGen)
      .build();
  player_gen.setup(&mut world);
  player_gen.dispatch(&world);
  
  let mut follow_player = DispatcherBuilder::new()
      .with_thread_local(CameraToActivePlayer)
      .build();
  follow_player.setup(&mut world);
  follow_player.dispatch(&world);
  
  let mut move_player = DispatcherBuilder::new()
      .with(PlayerInput, "PlayerInput", &[])
      .with(ApplyGravity, "ApplyGravity", &[])
      .with(ApplyRotation, "ApplyRotation", &[])
      .with(UpdateDeltaVelocity, "UpdateDeltaVelocity", &["ApplyRotation", "PlayerInput"])
      .with(Collision, "Collision", &["UpdateDeltaVelocity", "ApplyGravity"])
      .with(UpdatePos, "UpdatePos", &["Collision"])
      .build();
  
  // let mut move_player = DispatcherBuilder::new()
  //     .with_thread_local(PlayerInput)
  //     .with_thread_local(ApplyGravity)
  //     .with_thread_local(ApplyRotation)
  //     .with_thread_local(UpdateDeltaVelocity)
  //     .with_thread_local(Collision)
  //     .with_thread_local(UpdatePos)
  //     .build();
  move_player.setup(&mut world);
  move_player.dispatch(&world);
  
  // world.create_entity()
  //     .with()
  
  let mut terrain_draw = DispatcherBuilder::new()
      .with_thread_local(DrawPlatform)
      .build();
  terrain_draw.setup(&mut world);
  
  terrain_draw.dispatch(&world);
  
  let mut texmod_draw = DispatcherBuilder::new()
      .with_thread_local(DrawTexMods)
      .build();
  texmod_draw.setup(&mut world);
  
  texmod_draw.dispatch(&world);
  world.maintain();
  
  let mut particle_update = DispatcherBuilder::new()
      .with(UpdateParticles, "UpdateParticles", &[])
      .build();
  particle_update.setup(&mut world);
  
  let mut particle_draw = DispatcherBuilder::new()
      .with_thread_local(DrawParticles)
      .build();
  particle_draw.setup(&mut world);
  
  particle_draw.dispatch(&world);
  world.maintain();
  
  let particle_rule = ParticleRules::default()
    .set_texture("cosmic")
    .set_tex_row_count(4)
    .set_position(Vector3f::new(0.0,5.0,10.0))
    .set_direction(crate::util::YVEC, 0.5)
    .set_life_params(3.5, 0.5)
    .set_speed_params(1.0, 0.1)
    .set_scale_params(2.0, 0.5)
    .set_parts_per_sec(0.5)
  ;
  
  // Game loop!
  println!("Starting game loop.");
  let mut running = true;
  while running {
    {
      let mut handler = world.write_resource::<Handler>();
      handler.timer.tick();
      handler.reset_delta();
    }
    el.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
          glutin::WindowEvent::CloseRequested => running = false,
          glutin::WindowEvent::Resized(logical_size) => {
            let dpi = windowed_context.window().get_hidpi_factor();
            let size = logical_size.to_physical(dpi);
            windowed_context.resize(size);
            render_mgr.update_size(&world, size.into());
          },
          _ => { world.write_resource::<Handler>().window_event(&event); }
        },
        glutin::Event::DeviceEvent{ event, ..} => { world.write_resource::<Handler>().device_event(&event); }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    {
      {
        fps = world.read_resource::<Handler>().fps_and_delta();
        sec += fps.1;
      }
      if sec >= 1.0 {
        sec -= 1.0;
        let mut textmgr = world.write_resource::<TextMgr>();
        textmgr.update_text(&world, "FPS", &format!("FPS: {:.3}", (fps.0 * 1000.0).round() / 1000.0 ) );
      }
    }
    // *** Do per frame calculations such as movement
    
    gen_particles(&mut world, &particle_rule);
    particle_update.dispatch(&world);
    move_player.dispatch(&world);
    follow_player.dispatch(&world);
    world.maintain();
    
    // *** Drawing phase
    _fbo.bind();
    render_mgr.render(&world);
    // terrain_draw.dispatch(&world);
    texmod_draw.dispatch(&world);
    particle_draw.dispatch(&world);
    world.maintain();
    _fbo.unbind(&world);
    _fbo.blit_to_fbo(&world, 0, &_fbo_final);
    
    // render_mgr.render();
    // _fbo_final.blit_to_screen();
    render_post.render();
    render_mgr.render_gui(&world);
    // Write the new frame to the screen!
    windowed_context.swap_buffers().unwrap();
  }
  _fbo.clean_up();
  _fbo_final.clean_up();
  render_mgr.clean_up(&world);
  render_post.clean_up();
}

pub const EOF: &str = "\04";

pub fn eof(string: &str) -> String {
  [string, EOF].join("")
}

pub struct ViewMatrix { pub view: util::Matrix4f<f32> }
impl Default for ViewMatrix {
  fn default() -> Self {
    Self { view: util::Matrix4f::new() }
  }
}

