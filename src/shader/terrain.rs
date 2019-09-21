

use shader::{Shader, ShaderConf, };

pub struct TerrainShader{pub shader: Shader}
impl Default for TerrainShader {
  fn default() -> Self {
    TerrainShader{ shader: gen_terrain_shader() }
  }
}

pub fn gen_terrain_shader() -> Shader {
  let mut shader = Shader::new(ShaderConf::new("terrain")
      .with_attributes(vec!("a_Pos", "a_TexCoord", "a_Norm"))
      .with_uniforms(vec!(
        "u_Transform", "u_Projection", "u_View", "t_Texture",
        // "use_fake_lighting",
        // "row_count",
        // "offset",
        // "player_loc",
        // "sky_color",
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
  println!("Created terrain shader.");
  shader
}

// use {
//   cheddar, 
//   cheddar::{
//     ParseResult, // Module, 
//   }, 
// };
// pub fn test_terrain_cheddar() { // -> ParseResult<Module>
//   let terrain_str = "
// uniform mat4 u_Transform;
// uniform mat4 u_Projection;
// uniform mat4 u_View;

// uniform vec3 u_light_pos;

// uniform sampler2D u_Texture;
// uniform float u_shine_damper;
// uniform float u_reflectivity;

// uniform vec3 u_light_color;

// struct V {
//   vec4 chdr_Position;
//   vec2 texCoord;
//   vec3 surfaceNorm;
//   vec3 toLight;
//   vec3 toCam;
// };

// struct F {
//   vec4 color;
// };

// V map_vertex(vec3 pos, vec2 texCoord, vec3 norm) {
//   vec4 worldPos = u_Transform * vec4(pos, 1.0);
  
//   vec4 fpos = u_Projection * u_View * worldPos;
//   vec2 tex_coord = texCoord;
  
//   vec3 surface_norm = (u_Transform * vec4(norm, 0.0)).xyz;
//   vec3 to_light = u_light_pos - worldPos.xyz;
  
//   vec3 to_cam = (inverse(u_View) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
  
//   return V(fpos, tex_coord, surface_norm, to_light, to_cam);
// }

// F map_frag_data(V v) {
//   vec3 unitNormal = normalize(v.surfaceNorm);
//   vec3 unitCameraVector = normalize(v.toCam);
//   vec3 unitLightVector = normalize(v.toLight);
  
//   float nDotl = dot(unitNormal, unitLightVector);
//   float brightness = max(nDotl, 0.0);
//   vec3 diffuse = brightness * u_light_color;
//   vec3 lightDirection = -unitLightVector;
//   vec3 reflectedLightDirection = reflect(lightDirection, unitNormal);
//   float specularFactor = dot(reflectedLightDirection, unitCameraVector);
//   specularFactor = max(specularFactor, 0.0);
//   float dampedFactor = pow(specularFactor, u_shine_damper);
//   vec3 totalSpecular = dampedFactor * u_reflectivity * u_light_color;
  
//   vec4 texture_colour = vec4(diffuse, 1.0) * texture(u_Texture, v.texCoord) + vec4(totalSpecular, 1.0);
  
//   return F(texture_colour);
// }
// ";
//   match cheddar::parse_str(terrain_str) {
//     ParseResult::Ok(cheddar_module) => {
//       match cheddar_module.to_glsl_setup() {
//         Ok(cheddar_fold) => { println!("Vertex Shader:\n{}\n\nFragment Shader:\n{}", cheddar_fold.vs, cheddar_fold.fs); }
//         Err(e) => { println!("Error converting Cheddar to GLSL:\n{}", e); }
//       }
//     }
//     ParseResult::Err(e) => { println!("Error parsing Cheddar:\n{}", e); }
//     ParseResult::Incomplete(n) => { println!("Parsing Chedder Incomplete:\n{:?}", n); }
//   }
// }
