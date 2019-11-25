

use shader::{Shader, ShaderConf, };
pub fn gen_font_shader() -> Shader {
  let mut shader = Shader::new(ShaderConf::new("font")
      .with_attributes_auto(vec!("a_Pos", "a_TexCoord"))
      .with_uniforms(vec!(
        "translation",
        "offset",
        "colour",
        "fontAtlas",
      ))
  );
  shader.setup();
  println!("Created font shader.");
  shader
}
