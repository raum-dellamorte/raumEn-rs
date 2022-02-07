

use shader::{Shader, ShaderConf, };
pub fn gen_fog_shader() -> Shader {
  let mut shader = Shader::new(ShaderConf::new("fog")
      .with_attributes_auto(vec!("a_Pos"))
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

pub fn gen_overlay_shader() -> Shader {
  let mut shader = Shader::new(ShaderConf::new("overlay")
      .with_attributes_auto(vec!("a_Pos", "a_TexCoord"))
      // .with_uniforms(vec!(
      //   "translation",
      //   "offset",
      //   "colour",
      //   "fontAtlas",
      // ))
      .with_sampler_uniforms(vec!(
        ("bg_color", 0),
        ("fg_color", 1),
      ))
  );
  shader.setup();
  println!("Created font shader.");
  shader
}
