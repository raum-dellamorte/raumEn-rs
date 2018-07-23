

use shader::Shader;
pub fn gen_model_shader() -> Shader {
  let mut out = Shader::new("model");
  out.add_attributes(vec!("a_Pos", "a_Norm", "a_TexCoord")) // , "a_Norm"
  .add_uniforms(vec!(
    "u_Transform", "u_Projection", "u_View",
    "playerLoc", "lightPosition", "useFakeLighting",
    "useFakeLighting", "offset", "plane", "useClipPlane",
    "t_Texture", "lightColour", "attenuation",
    "shineDamper", "reflectivity", "skyColour"
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

uniform mat4 playerLoc;
uniform vec3 lightPosition[4];

uniform float useFakeLighting;

uniform float numOfRows;
uniform vec2 offset;

uniform vec4 plane;
uniform float useClipPlane;

// Frag
uniform sampler2D t_Texture;
uniform vec3 lightColour[4];
uniform vec3 attenuation[4];
uniform float shineDamper;
uniform float reflectivity;
uniform vec3 skyColour;
 */
