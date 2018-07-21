

use shader::Shader;
pub fn gen_model_shader() -> Shader {
  let mut out = Shader::new("model");
  out.add_attributes(vec!("a_Pos")) // , "a_Norm", "a_TexCoord"
  // .add_uniforms(vec!("u_Transform"))
  .load_defaults();
  println!("Created model shader.");
  out
}

