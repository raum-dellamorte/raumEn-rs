

use shader::Shader;

pub struct TerrainShader{pub shader: Shader}
impl Default for TerrainShader {
  fn default() -> Self {
    TerrainShader{ shader: gen_terrain_shader() }
  }
}

pub fn gen_terrain_shader() -> Shader {
  let mut out = Shader::new("terrain");
  out.add_attributes(vec!("a_Pos", "a_TexCoord", "a_Norm"))
  .add_uniforms(vec!(
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
  .load_defaults();
  println!("Created terrain shader.");
  out
}

