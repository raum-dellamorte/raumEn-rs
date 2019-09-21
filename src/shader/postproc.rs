

use shader::{Shader, ShaderConf, };
pub fn gen_fog_shader(effect: &str) -> Shader {
  let mut shader = Shader::new(ShaderConf::new(effect)
      .with_attributes(vec!("a_Pos"))
      // .add_uniforms(vec!(
      //   // Vertex
      //   "flip_y",
      // ))
      .with_sampler_uniforms(vec!(
        // Fragment
        ("color_texture", 0),
        ("depth_map", 1),
      ))
  );
  shader.setup();
  println!("Created PostProc shader.");
  shader
}
