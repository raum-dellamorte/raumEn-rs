

use Shader;
pub fn gen_hud_shader() -> Shader {
  let mut out = Shader::new("hud");
  out.add_attributes(vec!("a_Pos"))
  .add_uniforms(vec!(
    // Vertex
    "u_Transform",
    "row_count",
    "offset", 
    "flip_y",
    // Fragment
    "guiTexture", 
    "depthMap",
  ))
  .load_defaults();
  println!("Created HUD shader.");
  out
}
