
use {
  gl::{
    *,
    // types::{ // GLuint, GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
    // },
  },
  specs::World,
  Camera, Display, Loader, TextMgr,
  ecs::{
    c::{
      Lights,
    },
  },
  render::{
    // RenderTexModel, 
    // RenderTerrain, 
    RenderFont, 
    RenderHUD 
  },
  shader::{
    TerrainShader,
    TexModShader,
  },
  // util::{
  //   Rc, RefCell,
  // },
  // glutin::dpi::PhysicalSize,
};

pub fn prepare() { unsafe {
  Enable(CULL_FACE);
  CullFace(BACK);
  Enable(DEPTH_TEST);
  Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
  ClearColor(0.2, 0.2, 0.3, 1.0);
}}

#[derive(Default)]
pub struct RenderMgr {
  pub ren_font: RenderFont,
  pub ren_hud: RenderHUD,
}
impl RenderMgr {
  pub fn new() -> Self {
    RenderMgr {
      ren_font: RenderFont::new(),
      ren_hud: RenderHUD::new(),
    }
  }
  pub fn render(&mut self, world: &World) { 
    prepare();
    {
      use ViewMatrix;
      let mut cam = world.write_resource::<Camera>();
      let view = &mut (*world.write_resource::<ViewMatrix>()).view;
      cam.create_view_matrix(view);
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
    // self.return_mgr(mgr);
    unsafe { BindVertexArray(0); }
  }
  pub fn render_gui(&mut self, world: &World) { 
    self.ren_font.render(world);
    self.ren_hud.render(world);
    unsafe { BindVertexArray(0); }
  }
  pub fn update_size(&mut self, world: &World, dimensions: (u32, u32)) {
    world.write_resource::<Display>().update_size(dimensions);
    {
      let mut textmgr = world.write_resource::<TextMgr>();
      textmgr.update_size(world);
    }
    {
      let shader = &(*world.read_resource::<TexModShader>()).shader;
      let proj_mat = &world.read_resource::<Display>().proj_mat;
      shader.start();
      shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
      shader.stop();
    }
    {
      let shader = &(*world.read_resource::<TerrainShader>()).shader;
      let proj_mat = &world.read_resource::<Display>().proj_mat;
      shader.start();
      shader.load_matrix("u_Projection", &proj_mat); // Maybe move this to Shader
      shader.stop();
    }
  }
  pub fn clean_up(&mut self, world: &World) {
    let mut loader = world.write_resource::<Loader>();
    loader.clean_up();
    world.read_resource::<TerrainShader>().shader.clean_up();
    world.read_resource::<TexModShader>().shader.clean_up();
    self.ren_font.clean_up();
    self.ren_hud.clean_up();
  }
}