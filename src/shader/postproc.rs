

use Shader;
pub fn gen_fog_shader(effect: &str) -> Shader {
  let mut shader = Shader::new(effect);
  shader.add_attributes(vec!("a_Pos"))
  // .add_uniforms(vec!(
  //   // Vertex
  //   "flip_y",
  // ))
  .add_sampler_uniforms(vec!(
    // Fragment
    ("color_texture", 0), 
    ("depth_map", 1),
  ))
  .setup();
  println!("Created PostProc shader.");
  shader
}
