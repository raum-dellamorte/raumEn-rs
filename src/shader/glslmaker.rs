//! The problem I was trying to solve here was the problem of having to make sure my variable names
//! were the same in both my glsl and cpu side code.  I want to define things in one place and have
//! access to said defined things on both sides.  The idea here is to have rust write the glsl from
//! lists of in vars, out vars, uniforms, and consts, with functions built from lists of assignment 
//! statements that can be parsed just well enough to check that variable names are declared and
//! whatnot, then to be able to pass the needed variables back to the CPU side shader code, but it
//! may be simpler to parse the needed variables out of the glsl and pass those names to the cpu side 
//! shader code that way, still with basic variable name checking.  There are other projects that
//! deal with GLSL, perhaps I could fork something so that I can pass those variable names to my cpu
//! side shader code instead of rolling my own for everything I want to do.  But if I do roll my own,
//! I know how to use it.  


use {
  std::collections::HashSet,
  gl::types::{GLenum},
};

#[derive(Debug)]
pub struct GlslMaker {
  shader_version: String,
  stages: Vec<GlslMakerStage>,
}
impl Default for GlslMaker {
  fn default() -> Self {
    Self {
      shader_version: "#version 400".to_owned(),
      stages: Vec::new(),
    }
  }
}
impl GlslMaker {
  pub fn with_stage<F>(&mut self, stage: GLenum, f: F) -> &mut Self
      where F: Fn(&GlslMaker, &mut GlslMakerStage) -> ()
  {
    let mut out = GlslMakerStage::new(stage);
    f(&(*self), &mut out);
    self.stages.push(out);
    self
  }
  pub fn build(&mut self) {
    println!("Testing\n{}", self.shader_version);
  }
}

#[derive(Debug)]
pub struct GlslMakerStage {
  pub shader_stage: GLenum,
  glsl: String,
  _in: HashSet<String>,
  _out: HashSet<String>,
  _uniform: HashSet<String>,
  _const: HashSet<String>,
  _funcs: Vec<GlslMakerFunc>,
}
impl GlslMakerStage {
  pub fn new(stage: GLenum) -> Self {
    Self {
      shader_stage: stage,
      glsl: String::new(),
      _in: HashSet::new(),
      _out: HashSet::new(),
      _uniform: HashSet::new(),
      _const: HashSet::new(),
      _funcs: Vec::new(),
    }
  }
  pub fn build(&mut self, _shader_version: String) {
    self.glsl = String::new();
    
  }
  pub fn with_in(&mut self, name: &str) -> &mut Self {
    self._in.insert(name.to_owned());
    self
  }
  pub fn with_ins(&mut self, names: &[&str]) -> &mut Self {
    for name in names {
      self.with_in(name);
    }
    self
  }
  pub fn with_out(&mut self, name: &str) -> &mut Self {
    self._out.insert(name.to_owned());
    self
  }
  pub fn with_outs(&mut self, names: &[&str]) -> &mut Self {
    for name in names {
      self.with_out(name);
    }
    self
  }
  pub fn with_uniform(&mut self, name: &str) -> &mut Self {
    self._uniform.insert(name.to_owned());
    self
  }
  pub fn with_uniforms(&mut self, names: &[&str]) -> &mut Self {
    for name in names {
      self.with_uniform(name);
    }
    self
  }
  pub fn with_const(&mut self, name: &str) -> &mut Self {
    self._const.insert(name.to_owned());
    self
  }
  pub fn with_consts(&mut self, names: &[&str]) -> &mut Self {
    for name in names {
      self.with_const(name);
    }
    self
  }
  pub fn with_func<F>(&mut self, name: &str, params: &[(&str, &str)], returns: &str, f: F) -> &mut Self 
      where F: Fn(&GlslMakerStage, &mut GlslMakerFunc) -> ()
  {
    let mut out = GlslMakerFunc::new(name, params, returns);
    f(&(*self), &mut out);
    self._funcs.push(out);
    self
  }
}

#[derive(Debug)]
pub struct GlslMakerFunc {
  pub name: String,
  pub params: Vec<(String, String)>,
  pub returns: String,
  _lines: Vec<String>,
  _vars: HashSet<String>,
}
impl GlslMakerFunc {
  pub fn new(name: &str, params: &[(&str, &str)], returns: &str) -> Self {
    let mut _params: Vec<(String, String)> = Vec::new();
    for (nam,typ) in params {
      _params.push(((*nam).to_owned(), (*typ).to_owned()));
    }
    Self {
      name: name.to_owned(),
      params: _params,
      returns: returns.to_owned(),
      _vars: HashSet::new(),
      _lines: Vec::new(),
    }
  }
  pub fn with_assign(&mut self, var: &str, typ: &str, val: &str) -> &mut Self {
    if typ.is_empty() {
      self._lines.push(format!("  {} = {};\n", var, val));
    } else {
      self._lines.push(format!("  {} {} = {};\n", typ, var, val));
    }
    self
  }
}

