

use Shader;
pub fn gen_hud_shader() -> Shader {
  let mut shader = Shader::new("hud");
  shader.add_attributes(vec!("a_Pos"))
  .add_uniforms(vec!(
    // Vertex
    "u_Transform",
    "row_count",
    "offset", 
    "flip_y",
  ))
  .add_sampler_uniforms(vec!(
    // Fragment
    ("guiTexture", 0), 
    ("depthMap", 1),
  ))
  .load_defaults();
  println!("Created HUD shader.");
  shader
}
