

use shader::Shader;
pub fn gen_font_shader() -> Shader {
  let mut out = Shader::new("font");
  out.add_attributes(vec!("a_Pos", "a_TexCoord"))
  .add_uniforms(vec!(
    "translation",
    "offset", 
    "colour", 
    "fontAtlas", 
  ))
  .load_defaults();
  println!("Created font shader.");
  out
}
