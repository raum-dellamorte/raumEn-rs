
use {
  gl::*,
  crate::{
    ecs::resource::{
      Texture,
    },
    shader::{
      Shader,
      gen_fog_shader,
    }, 
  },
};

pub struct RenderPostProc {
  pub shader: Shader,
  pub quad_id: u32,
  pub textures: Vec<Texture>,
}
impl RenderPostProc {
  pub fn new(effect: &str, quad_id: u32, textures: Vec<Texture>) -> Self {
    Self {
      shader: gen_fog_shader(effect),
      quad_id,
      textures,
    }
  }
  pub fn render(&self) {
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Gui Render Pass");
    {
      // todo: switch from HUD to fullscreen texture to process
      self.shader.start();
      unsafe {
        BindVertexArray(self.quad_id);
        EnableVertexAttribArray(0);
        
        // Using more than one texture in a shader.
        // This took me forever to solve.  I could only get TEXTURE0 to work. TEXTURE1 was being ignored.
        // I eventually found out that if you have more that one texture you have to declare which
        // uniform sampler is which with Uniform1i(), that's i at the end for integer, not f for float.
        // So I added add_sampler_uniforms() to Shader for the gen_x_shader() functions to specify
        // which uniforms are texture samplers and what TEXTURE unit to attach them to, and then added
        // load_sampler_uniforms() to do all the Uniform1i() stuff automatically for any uniform with a
        // texture value greater than -1, and put that function inside load_defaults(), which was already
        // being called in all the gen_x_shader() functions.  The end result is that I can declare the 
        // regular uniforms as normal and declare the sampler uniforms with their TEXTURE number and it
        // all just works.  When I switch to using (probably) JSON files (or XML or whatever) to load all
        // the game details, half the work is done because Shader already turns lists of strings into
        // all the glsl variable connections I need.
        
        // Texture!
        use util::rgl::r_bind_texture;
        for tex in &self.textures {
          r_bind_texture(tex);
        }
        
        // // Shader Vars!
        // self.shader.load_bool("flip_y", false);
        
        // Draw!
        DrawArrays(TRIANGLE_STRIP, 0, 4_i32);
        DisableVertexAttribArray(0);
        BindVertexArray(0);
      }
      self.shader.stop();
      unsafe {
        Disable(BLEND);
        Enable(DEPTH_TEST);
      }
    }
  }
  pub fn clean_up(&self) {
    self.shader.clean_up();
  }
}
