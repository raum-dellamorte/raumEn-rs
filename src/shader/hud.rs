

use shader::{Shader, ShaderConf, };
pub fn gen_hud_shader() -> Shader {
  let mut shader = Shader::new(ShaderConf::new("hud")
      .with_attributes(vec!("a_Pos"))
      .with_uniforms(vec!(
        // Vertex
        "u_Transform",
        "row_count",
        "offset",
        "flip_y",
      ))
      .with_sampler_uniforms(vec!(
        // Fragment
        ("guiTexture", 0),
        ("depthMap", 1),
      ))
  );
  shader.setup();
  println!("Created HUD shader.");
  shader
}
