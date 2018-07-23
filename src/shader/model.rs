

use shader::Shader;
pub fn gen_model_shader() -> Shader {
  let mut out = Shader::new("model");
  out.add_attributes(vec!("a_Pos", "a_TexCoord")) // , "a_Norm"
  .add_uniforms(vec!("u_Transform", "u_Projection", "u_View"))
  .load_defaults();
  println!("Created model shader.");
  out
}

