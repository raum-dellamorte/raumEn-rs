

use shader::Shader;
pub fn gen_model_shader() -> Shader {
  let mut out = Shader::new("model");
  out.add_attributes(vec!("a_Pos", "a_TexCoord", "a_Norm"))
  .add_uniforms(vec!(
    "u_Transform", "u_Projection", "u_View", "t_Texture", 
    "use_fake_lighting", "row_count", "offset", // "player_loc", 
    "sky_color", "shine_damper", "reflectivity", // "plane", "use_clip_plane", 
    "light_pos", "light_color", "attenuation"
  ))
  .load_defaults();
  println!("Created model shader.");
  out
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
