

use shader::{Shader, ShaderConf, };

pub struct TexModShader{pub shader: Shader}
impl Default for TexModShader {
  fn default() -> Self {
    TexModShader{ shader: gen_texmod_shader() }
  }
}

pub fn gen_texmod_shader() -> Shader {
  let mut shader = Shader::new(ShaderConf::new("model")
      .with_attributes(vec!("a_Pos", "a_TexCoord", "a_Norm"))
      .with_uniforms(vec!(
        "u_Transform", "u_Projection", "u_View", "t_Texture",
        // "use_fake_lighting",
        "row_count",
        "offset",
        // "player_loc",
        "color_id",
        "shine_damper",
        "reflectivity",
        // "plane",
        // "use_clip_plane",
        "light_pos",
        "light_color",
        // "attenuation"
      ))
  );
  shader.setup();
  println!("Created model shader.");
  shader
}

/*
// Vert
uniform mat4 u_Transform;
uniform mat4 u_Projection;
uniform mat4 u_View;

uniform mat4 player_loc;
uniform vec3 light_pos[4];

uniform float use_fake_lighting;

uniform float row_count;
uniform vec2 offset;

uniform vec4 plane;
uniform float use_clip_plane;

// Frag
uniform sampler2D t_Texture;
uniform vec3 lightColour[4];
uniform vec3 attenuation[4];
uniform float shine_damper;
uniform float reflectivity;
uniform vec3 sky_color;
 */
