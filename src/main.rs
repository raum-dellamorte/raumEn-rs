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
#[macro_use] extern crate lazy_static;
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
pub mod constants {
  use {
    crate::{
      util::{
        Mutex, Vector3f, 
      },
    },
  };
  
  pub const XVEC: Vector3f<f32> = Vector3f {x: 1.0_f32, y: 0.0_f32, z: 0.0_f32};
  pub const YVEC: Vector3f<f32> = Vector3f {x: 0.0_f32, y: 1.0_f32, z: 0.0_f32};
  pub const ZVEC: Vector3f<f32> = Vector3f {x: 0.0_f32, y: 0.0_f32, z: 1.0_f32};
  pub const XVEC64: Vector3f<f64> = Vector3f {x: 1.0_f64, y: 0.0_f64, z: 0.0_f64};
  pub const YVEC64: Vector3f<f64> = Vector3f {x: 0.0_f64, y: 1.0_f64, z: 0.0_f64};
  pub const ZVEC64: Vector3f<f64> = Vector3f {x: 0.0_f64, y: 0.0_f64, z: 1.0_f64};
  pub const GRAVITY: f32 = 10.0;
  pub const TERMVEL: f32 = 15.0;
  pub const NEARBY: f32 = 50.0;
  pub const TOLERANCE64: f32 = 0.00001;

  lazy_static!{
    pub static ref DISPLAY: Mutex<crate::engine::Display> = Mutex::new(crate::engine::Display::default());
  }
}

#[allow(unused_imports)]
use {
  gl::*,
  glutin::{
    event::{Event, WindowEvent, Event::DeviceEvent, },
    event_loop::{ControlFlow, EventLoop, },
  },
  specs::{
    // Builder, Component, ReadStorage, WriteStorage, System, VecStorage, RunNow,
    DispatcherBuilder, 
    World, WorldExt, 
  },
  crate::{
    constants::DISPLAY,
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
    render::{
      RenderPostProc, RenderFont, RenderHUD,
    },
    shader::{
      Shader,
      ShaderWrapper,
      ParticleShader,
      TerrainShader,
      TexModShader,
    },
    util::{
      Vector3f, Mutex, 
    },
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
      Fbo, HUD, GuiObj, Handler, Loader
    },
    text::TextMgr,
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
  // world.insert(Camera::default());
  // world.insert(Display::default());
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
  world.register::<IsNearby>();
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
  
  // Specify OpenGL version
  let gl_request = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));
  let gl_profile = glutin::GlProfile::Core;
  // Create a window
  let mut el = EventLoop::new();
  let wb = glutin::window::WindowBuilder::new()
    .with_title("RaumEn")
    .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0))
    .with_maximized(false);
  let windowed_context = glutin::ContextBuilder::new()
    .with_gl(gl_request)
    .with_gl_profile(gl_profile)
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
  let mut render_hud = RenderHUD::default();
  let mut render_fnt = RenderFont::new();
  
  // time keeping
  let mut fps: f32 = 60.0;
  let mut once_per_sec = false;
  let mut clean_up_time = false;
  
  // Set up the world, which holds all the things,
  // and will be passed around...
  //     ...like your mom at a frat house.
  // I'm sorry, that was uncalled for.
  let mut world = gen_world();
  
  { // Here, we're getting the size of the window in pixels
    // and passing it to the update_size() method. It in turn
    // updates the Projection Matrix and passes that to 
    // ALL THE SHADERS, so if you add a SHADER, you need
    // to REMEMBER to add that shader to the update_size()
    // method near the bottom of this file.
    // let dpi = windowed_context.window().get_hidpi_factor();
    let size: glutin::dpi::PhysicalSize<u32> = windowed_context.window().inner_size();
    update_size(&world, size.into());
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
  let mut _fbo = Fbo::new(0, 0, ColorMultisampleRenderBuffers2, DepthRenderBuffer);
  let mut _fbo_final = Fbo::new(0, 0, ColorTexture, DepthTexture);
  let render_post = RenderPostProc::new("fog", world.read_resource::<HUD>().quad.vao_id.0, 
      vec![
        Texture::new("fbo color", _fbo_final.color_tex_id).assign_tex_unit(0_i32),
        Texture::new("fbo depth", _fbo_final.depth_tex_id).assign_tex_unit(1_i32),
      ]);
  { // Right now, the HUD is displaying a small version of the whole
    // screen so you can see the color and depth buffers for debugging
    // purposes.
    let mut _hud = world.write_resource::<HUD>();
    _hud.elements.push(GuiObj::new_one()); // bad practice for testing 
    _hud.elements.push(GuiObj::new_two()); // maybe load them from a file?
    let _gui = _hud.elements.get_mut(0).unwrap();
    _gui.tex_id = world.read_resource::<Textures>().0["cosmic"].tex_id.0;
    let _gui = _hud.elements.get_mut(1).unwrap();
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
      .with(NearbyThings, "NearbyThings", &[])
      .with(PlayerInput, "PlayerInput", &[])
      .with(ApplyGravity, "ApplyGravity", &[])
      .with(ApplyRotation, "ApplyRotation", &[])
      .with(UpdateDeltaVelocity, "UpdateDeltaVelocity", &["ApplyRotation", "PlayerInput"])
      .with(Collision, "Collision", &["UpdateDeltaVelocity", "ApplyGravity"])
      .with(UpdatePos, "UpdatePos", &["Collision"])
      .build();
  move_player.setup(&mut world);
  // move_player.dispatch(&world);
  
  let mut terrain_draw = DispatcherBuilder::new()
      .with_thread_local(DrawPlatform)
      .build();
  terrain_draw.setup(&mut world);
  // terrain_draw.dispatch(&world);
  
  let mut texmod_draw = DispatcherBuilder::new()
      .with_thread_local(DrawTexMods)
      .build();
  texmod_draw.setup(&mut world);
  
  // texmod_draw.dispatch(&world);
  // world.maintain();
  
  // let mut particle_update = DispatcherBuilder::new()
  //     .with(UpdateParticles, "UpdateParticles", &[])
  //     .build();
  // particle_update.setup(&mut world);
  
  // let mut particle_draw = DispatcherBuilder::new()
  //     .with_thread_local(DrawParticles)
  //     .build();
  // particle_draw.setup(&mut world);
  
  // particle_draw.dispatch(&world);
  // world.maintain();
  
  // let particle_rule = ParticleRules::default()
  //   .set_texture("cosmic")
  //   .set_tex_row_count(4)
  //   .set_position(Vector3f::new(0.0,10.0,0.0))
  //   .set_direction(crate::util::YVEC, 0.5)
  //   .set_life_params(3.5, 0.5)
  //   .set_speed_params(1.0, 0.1)
  //   .set_scale_params(2.0, 0.5)
  //   .set_parts_per_sec(20.0)
  // ;
  
  // Game loop!
  println!("Starting game loop.");
  el.run(move |event, _, control_flow| {
    match event {
      Event::LoopDestroyed => { println!("Event::LoopDestroyed"); return; }
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          println!("Close Requested");
          println!("Cleaning Up...");
          _fbo.clean_up();
          _fbo_final.clean_up();
          clean_up(&world);
          render_hud.clean_up();
          render_fnt.clean_up();
          render_post.clean_up();
          clean_up_time = true;
          *control_flow = ControlFlow::Exit;
        },
        WindowEvent::Resized(size) => {
          windowed_context.resize(size);
          update_size(&world, size.into());
        },
        _ => { world.write_resource::<Handler>().window_event(&event); }
      },
      DeviceEvent{ event, ..} => { world.write_resource::<Handler>().device_event(&event); }
      Event::NewEvents( _time ) => {
        // Emitted when new events arrive from the OS to be processed.
        // 
        // This event type is useful as a place to put code that should be done before you start processing events, such 
        // as updating frame timing information for benchmarking or checking the StartCause][crate::event::StartCause] to 
        // see if a timer set by [ControlFlow::WaitUntil has elapse
        // println!("Event::NewEvents");
      }
      Event::MainEventsCleared => {
        // Emitted when all of the event loop's input events have been processed and redraw processing is about to begin.
        
        // This event is useful as a place to put your code that should be run after all state-changing events have been 
        // handled and you want to do stuff (updating state, performing calculations, etc) that happens as the "main body" 
        // of your event loop. 
        // If your program draws graphics, it's usually better to do it in response to Event::RedrawRequested, which gets 
        // emitted immediately after this event.
        // println!("Event::MainEventsCleared");
        
        {
          let mut handler = world.write_resource::<Handler>();
          handler.timer.tick();
          handler.reset_delta();
          if handler.timer.once_per_sec() {
            fps = handler.timer.fps;
            once_per_sec = true;
          }
        }
        if once_per_sec {
          once_per_sec = false;
          println!("Once per second FPS: {}", &format!("FPS: {:.3}", (fps * 1000.0).round() / 1000.0 ) );
          let mut textmgr = world.write_resource::<TextMgr>();
          textmgr.update_text(&world, "FPS", &format!("FPS: {:.3}", (fps * 1000.0).round() / 1000.0 ) );
        }
        
        // *** Do per frame calculations such as movement
        
        // gen_particles(&mut world, &particle_rule);
        // particle_update.dispatch(&world);
        move_player.dispatch(&world);
        follow_player.dispatch(&world);
        // world.maintain();
        windowed_context.window().request_redraw();
      }
      Event::RedrawRequested(_) => {
        // Emitted after MainEventsCleared when a window should be redrawn.
        
        // This gets triggered in two scenarios:
        
        // - The OS has performed an operation that's invalidated the window's contents (such as resizing the window).
        // - The application has explicitly requested a redraw via Window::request_redraw.
        
        // During each iteration of the event loop, Winit will aggregate duplicate redraw requests into a single event, 
        // to help avoid duplicating rendering work.
        
        
        if clean_up_time { return; }
        // *** Drawing phase
        _fbo.bind();
        prepare(&world);
        terrain_draw.dispatch(&world);
        texmod_draw.dispatch(&world);
        // particle_draw.dispatch(&world);
        world.maintain();
        _fbo.unbind();
        _fbo.blit_to_fbo(0, &_fbo_final);
        
        // _fbo_final.blit_to_screen(&world);
        render_post.render();
        render_hud.render(&world);
        render_fnt.render(&world);
        // Write the new frame to the screen!
        windowed_context.swap_buffers().unwrap();
      }
      Event::RedrawEventsCleared => {
        // Emitted after all RedrawRequested events have been processed and control flow is about to be taken away from 
        // the program. If there are no RedrawRequested events, it is emitted immediately after MainEventsCleared.
        
        // This event is useful for doing any cleanup or bookkeeping work after all the rendering tasks have been completed.
        // println!("Event::RedrawEventsCleared");
      }
      e => println!("Other Event:\n{:?}", e)
    }
  });
}

pub fn update_size(world: &World, dimensions: (u32, u32)) {
  DISPLAY.lock().unwrap().update_size(dimensions);
  {
    let mut textmgr = world.write_resource::<TextMgr>();
    textmgr.update_size(world);
  }
  let proj_mat = DISPLAY.lock().unwrap().proj_mat.clone();
  world.read_resource::<TexModShader>().update_projection(&proj_mat);
  world.read_resource::<TerrainShader>().update_projection(&proj_mat);
  world.read_resource::<ParticleShader>().update_projection(&proj_mat);
}
pub fn prepare(world: &World) {
  unsafe {
    // Enable(CULL_FACE);
    // CullFace(BACK);
    Enable(DEPTH_TEST);
    Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
    ClearColor(0.2, 0.2, 0.3, 1.0);
  }
  { // Prep the view matrix
    // let view = &mut (*world.write_resource::<ViewMatrix>()).view;
    DISPLAY.lock().unwrap().camera.create_view_matrix();
  }
  
  let lights = world.read_resource::<Lights>();
  {
    let shader = &(*world.read_resource::<TerrainShader>()).shader;
    shader.start();
    lights.load_to_shader(shader);
    shader.stop();
  }
  {
    let shader = &(*world.read_resource::<TexModShader>()).shader;
    shader.start();
    lights.load_to_shader(shader);
    shader.stop();
  }
  unsafe { BindVertexArray(0); }
}
pub fn clean_up(world: &World) {
  let mut loader = world.write_resource::<Loader>();
  loader.clean_up();
  world.read_resource::<TerrainShader>().shader.clean_up();
  world.read_resource::<TexModShader>().shader.clean_up();
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

