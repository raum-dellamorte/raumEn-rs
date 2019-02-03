#![recursion_limit="128"]
#![allow(unused_imports,dead_code)]

extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;
extern crate num;
extern crate cgmath;
extern crate time;
extern crate noise;

use gl::*;
use std::os::raw::c_void;
use glutin::dpi::*;
use glutin::GlContext;
// use cgmath::{Matrix4, Point3, Vector3}; // Deg, 

const CVOID: *const c_void = 0 as *const c_void;

// in project stuff
pub mod engine;
pub mod entities;
pub mod material;
pub mod model;
pub mod render;
pub mod shader;
pub mod terrain;
pub mod text;
pub mod util;

pub use engine::{Camera, Display, Fbo, GameMgr, HUD, GuiObj, Handler, Loader, Timer};
pub use entities::Entity;
pub use entities::Mob;
pub use material::{Material, Texture, Lights, Lighting};
pub use render::{RenderMgr, RenderPostProc, };
pub use shader::Shader;
pub use terrain::{World, WorldBuilder};

use engine::fbo::ColorType::{ColorMultisampleRenderBuffer, ColorMultisampleRenderBuffers2, ColorTexture, NoColor};
use engine::fbo::DepthType::{DepthRenderBuffer, DepthTexture, NoDepth};

fn main() {
  // // Test code for parsing fnt files
  // use text::metafile::test_noms;
  // test_noms();
  
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_title("RaumEn")
    .with_dimensions(LogicalSize::new(640.0, 360.0))
    .with_maximized(false);
  let context = glutin::ContextBuilder::new()
    .with_vsync(true);
  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
  gl_window.set_maximized(false);
  
  unsafe {
    gl_window.make_current().unwrap();
  }
  
  unsafe {
    load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  
  let mut render_mgr = RenderMgr::new();
  let mut mgr = render_mgr.take_mgr();
  
  let mut spaceship = {
    mgr.new_model("spaceship");
    mgr.new_material("spaceship", "spaceship", "metal");
    mgr.new_entity("spaceship", "spaceship", "spaceship");
    mgr.new_model("player");
    mgr.new_material("player", "dirt", "metal");
    mgr.new_entity("player", "player", "player");
    mgr.mod_entity("spaceship", |ships| {
      ships.new_instance().set_pos(10.0,10.0,10.0);
      ships.new_instance().set_pos(10.0,0.0,-10.0);
      ships.new_instance().set_pos(-12.0,5.0,-15.0);
    });
    mgr.mod_entity("player", |player| {
      player.new_instance().set_pos(0.0,10.0,0.0);
    });
    
    mgr.new_material("dirt", "dirt", "flat");
    mgr.new_model("platform");
    // println!("entities loaded");
    let hm = mgr.entities.borrow_mut();
    hm.get("player").unwrap().first().create_mob("player")
  };
  render_mgr.return_mgr(mgr);
  
  let mut fps: (f32, f32);
  let mut sec = 0.0;
  
  {
    let dpi = gl_window.get_hidpi_factor();
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    render_mgr.update_size(size.into());
  }
  let mut mgr = render_mgr.take_mgr();
  {
    let _textmgr = mgr.textmgr.take().unwrap();
    {
      let mut textmgr = _textmgr.borrow_mut();
      mgr = textmgr.add_font(mgr, "pirate");
      mgr = textmgr.add_font(mgr, "sans");
      mgr = textmgr.new_text(mgr, "Title", "The Never", "pirate", 4.0, 0.0, 0.0, 1.0, true, true);
      mgr = textmgr.new_text(mgr, "FPS", "FPS: 0.0", "sans", 1.5, 0.0, 0.0, 0.3, false, true);
    }
    mgr.textmgr = Some(_textmgr);
  }
  
  
  let mut _fbo = Fbo::new(mgr.display_clone(), 0, 0, ColorMultisampleRenderBuffers2, DepthRenderBuffer);
  let mut _fbo_final = Fbo::new(mgr.display_clone(), 0, 0, ColorTexture, DepthTexture);
  let render_post = RenderPostProc::new("fog", mgr.quad_id, 
      vec![
        Texture::new("fbo color", _fbo_final.color_tex_id).assign_tex_unit(0_i32),
        Texture::new("fbo depth", _fbo_final.depth_tex_id).assign_tex_unit(1_i32),
      ]);
  {
    let mut _hud = mgr.hud.borrow_mut();
    _hud.elements.push(GuiObj::new());
    let _gui = _hud.elements.get_mut(0).unwrap();
    _gui.tex_id = _fbo_final.color_tex_id;
    _gui.depth_tex_id = _fbo_final.depth_tex_id;
  }
  
  // Return the GameMgr to the RenderMgr
  render_mgr.return_mgr(mgr);
  
  // Game loop!
  println!("Starting game loop.");
  let mut running = true;
  while running {
    {
      let mut handler = render_mgr.mgr.as_mut().unwrap().take_handler();
      handler.timer.tick();
      handler.reset_delta();
      render_mgr.mgr.as_mut().unwrap().return_handler(handler);
    }
    events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
          glutin::WindowEvent::CloseRequested => running = false,
          glutin::WindowEvent::Resized(logical_size) => {
            let dpi = gl_window.get_hidpi_factor();
            let size = logical_size.to_physical(dpi);
            gl_window.resize(size);
            render_mgr.update_size(size.into());
          },
          _ => {
            let mut handler = render_mgr.mgr.as_mut().unwrap().take_handler();
            handler.window_event(&event);
            render_mgr.mgr.as_mut().unwrap().return_handler(handler);
          }
        },
        glutin::Event::DeviceEvent{ event, ..} => {
          let mut handler = render_mgr.mgr.as_mut().unwrap().take_handler();
          handler.device_event(&event);
          render_mgr.mgr.as_mut().unwrap().return_handler(handler);
        }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    let mut mgr = render_mgr.take_mgr();
    {
      {
        fps = mgr.fps_and_delta();
        sec += fps.1;
      }
      if sec >= 1.0 {
        sec -= 1.0;
        let _textmgr = mgr.textmgr.take().unwrap();
        {
          let mut textmgr = _textmgr.borrow_mut();
          mgr = textmgr.update_text(mgr, "FPS", &format!("FPS: {:.3}", (fps.0 * 1000.0).round() / 1000.0 ) );
        }
        mgr.textmgr = Some(_textmgr);
        
      }
      
      // Borrowing things from mgr
      let mut handler = mgr.take_handler();
      let mut camera = mgr.take_camera();
      let mut world = mgr.take_world();
      { // Do per frame calculations such as movement
        
        spaceship.move_mob(&mut handler, &mut world);
        camera.calc_pos(&mut handler, &spaceship.pos.borrow());
        spaceship.pos_copy(&mut mgr.player_loc);
        
      }
      // Returning borrowed things to mgr
      mgr.return_camera(camera);
      mgr.return_handler(handler);
      mgr.return_world(world);
      mgr.gen_chunks();
    }
    // Returning mgr to render_mgr
    render_mgr.return_mgr(mgr);
    // Draw the stuff we keep in the mgr we just returned
    _fbo.bind();
    render_mgr.render();
    _fbo.unbind();
    _fbo.blit_to_fbo(0, &_fbo_final);
    
    // render_mgr.render();
    // _fbo_final.blit_to_screen();
    render_post.render();
    render_mgr.render_gui();
    // Write the new frame to the screen!
    gl_window.swap_buffers().unwrap();
  }
  _fbo.clean_up();
  _fbo_final.clean_up();
  render_mgr.clean_up();
  render_post.clean_up();
}

pub const EOF: &str = "\04";

pub fn eof(string: &str) -> String {
  [string, EOF].join("")
}
