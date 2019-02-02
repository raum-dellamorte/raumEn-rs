
use gl::*;
use gl::types::{GLenum, GLuint, GLint, GLfloat, }; // GLchar, GLsizeiptr, GLboolean, 
// use std::mem;
use std::ptr;
use std::str;
use std::str::from_utf8;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem::transmute;
// use cgmath::{Matrix, Matrix4, };

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
// use std::rc::Rc;
use std::cell::RefCell;

use util::rmatrix::Matrix4f;
use util::rvector::{ Vector2f, Vector3f, Vector4f };

pub struct ShaderVar {
    var_name: String,
    var_id: GLint,
}
impl ShaderVar {
  pub fn new(name: &str) -> Self {
    ShaderVar {
      var_name: format!("{}", name),
      var_id: -1 as GLint,
    }
  }
}

pub struct ShaderUni {
    var_name: String,
    var_id: GLint,
    texture: GLint,
}
impl ShaderUni {
  pub fn new(name: &str) -> Self {
    ShaderUni {
      var_name: format!("{}", name),
      var_id: -1 as GLint,
      texture: -1 as GLint,
    }
  }
}

pub struct ShaderOutputVar {
    var_name: CString,
    var_loc: u32,
}
impl ShaderOutputVar {
  pub fn new(name: &str, loc: u32) -> Self {
    let name = format!("{}", name);
    let cname = CString::new(name.as_bytes()).unwrap();
    ShaderOutputVar {
      var_name: cname,
      var_loc: loc,
    }
  }
}

pub struct ShaderSrc {
  kind: GLenum,
  id: GLuint,
  src: CString,
}

impl ShaderSrc {
  pub fn new(kind: GLenum, id: GLuint, src: CString) -> Self {
    ShaderSrc { kind: kind, id: id, src: src }
  }
  pub fn kind(&self) -> &str {
    match self.kind {
      VERTEX_SHADER => { "Vertex" }
      FRAGMENT_SHADER => { "Fragment" }
      _ => { "Unknown" }
    }
  }
}

pub struct Shader {
  pub name: String,
  pub program: GLuint,
  pub done: bool,
  pub shaders: Vec<ShaderSrc>,
  pub vars: Vec<ShaderVar>,
  pub unis: Vec<ShaderUni>,
  pub unis_unavailable: RefCell<HashSet<String>>,
}

impl Shader {
  pub fn new(name: &str) -> Self {
    Shader { 
      name: format!("{}", name), program: 0, done: false,
      shaders: Vec::new(), vars: Vec::new(), unis: Vec::new(), 
      unis_unavailable: RefCell::new(HashSet::new()),
    }
  }
  pub fn load_defaults(&mut self) -> &mut Self {
    self
    .load_vert_shader()
    .load_frag_shader()
    .compile_shaders()
    .link()
    .gen_uniforms();
    self.start();
    self.connect_sampler_uniforms();
    self.stop();
    self
  }
  pub fn add_attributes(&mut self, names: Vec<&str>) -> &mut Self {
    for name in names {
      self.add_attribute(name);
    }
    self
  }
  pub fn add_attribute(&mut self, name: &str) -> &mut Self {
    self.vars.push(ShaderVar::new(name));
    self
  }
  pub fn add_uniforms(&mut self, names: Vec<&str>) -> &mut Self {
    for name in names {
      self.add_uniform(name);
    }
    self
  }
  pub fn add_sampler_uniforms(&mut self, names: Vec<(&str, GLint)>) -> &mut Self {
    for (name, num) in names {
      self.add_texture_uniform(name, num);
    }
    self
  }
  pub fn add_uniforms_array(&mut self, names: Vec<&str>, count: usize) -> &mut Self {
    for name in names {
      let mut i = 0;
      while i < count {
        self.add_uniform(&format!("{}[{}]", name, i));
        i += 1;
      }
    }
    self
  }
  pub fn add_uniform(&mut self, name: &str) -> &mut Self {
    self.unis.push(ShaderUni::new(name));
    self
  }
  pub fn add_texture_uniform(&mut self, name: &str, texture: GLint) -> &mut Self {
    let mut shuni = ShaderUni::new(name);
    shuni.texture = texture;
    self.unis.push(shuni);
    self
  }
  pub fn bind_attributes(&mut self) -> &mut Self { unsafe {
    let mut count = 0 as GLint;
    let mut cname;
    for attrib in &mut self.vars {
      cname = CString::new(attrib.var_name.as_bytes()).unwrap();
      BindAttribLocation(self.program, count as GLuint, cname.as_ptr());
      attrib.var_id = count;
      count += 1;
    }
    self
  }}
  pub fn bind_frag_data_locations(&mut self, outputs: &mut Vec<ShaderOutputVar>) -> &mut Self { unsafe {
    // Use this for multiple render targets if you can't use layout (location) in glsl
    for output in outputs {
      BindFragDataLocation(self.program, output.var_loc, output.var_name.as_ptr());
    }
    self
  }}
  pub fn connect_sampler_uniforms(&self) {
    for uni in &self.unis {
      if uni.texture >= 0 { self.load_int(&uni.var_name, uni.texture); }
    }
  }
  pub fn gen_uniforms(&mut self) -> &mut Self {
    self.start();
    for uniform in &mut self.unis {
      uniform.var_id = get_uniform_location(self.program, &uniform.var_name);
    }
    self.stop();
    self
  }
  pub fn get_uniform_id(&self, name: &str) -> GLint {
    for uni in &self.unis {
      if uni.var_name == name {
        return uni.var_id
      }
    }
    println!("Uniform name not found: {}", name);
    -1 as GLint
  }
  pub fn load_proj_mat(&self, matrix: &Matrix4f) {
    self.load_matrix("u_Projection", matrix);
  }
  pub fn load_int(&self, name: &str, value: GLint) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_int") { return }
    Uniform1i(id, value);
  }}
  pub fn load_float(&self, name: &str, value: GLfloat) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_float") { return }
    Uniform1f(id, value);
  }}
  pub fn load_bool(&self, name: &str, value: bool) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_bool") { return }
    Uniform1f(id, if value { 1.0 as GLfloat } else { 0.0 as GLfloat })
  }}
  pub fn load_vec_4f(&self, name: &str, vector: &Vector4f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_vec_4f") { return }
    Uniform4f(id, vector.x, vector.y, vector.z, vector.w);
  }}
  pub fn load_vec_3f(&self, name: &str, vector: &Vector3f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_vec_3f") { return }
    Uniform3f(id, vector.x, vector.y, vector.z);
  }}
  pub fn load_vec_2f(&self, name: &str, vector: &Vector2f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_vec_2f") { return }
    Uniform2f(id, vector.x, vector.y);
  }}
  pub fn load_matrix(&self, name: &str, matrix: &Matrix4f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_matrix") { return }
    UniformMatrix4fv(id, 1, 0, transmute(&matrix.matrix[0]) );
  }}
  pub fn load_vert_shader(&mut self) -> &mut Self {
    self.add_shader(VERTEX_SHADER)
  }
  pub fn load_frag_shader(&mut self) -> &mut Self {
    self.add_shader(FRAGMENT_SHADER)
  }
  pub fn start(&self) { unsafe {
    UseProgram(self.program);
  }}
  pub fn stop(&self) { unsafe {
    UseProgram(0);
  }}
  fn check_id(&self, id: GLint, name: &str, caller: &str) -> bool {
    let test = self.unis_unavailable.borrow().contains(name);
    if test { return true } else {
      if id < 0 { 
        self.unis_unavailable.borrow_mut().insert(name.to_string());
        println!("{}(): Uniform {} not available for shader {}", caller, name, self.name); 
        return true;
      }
    }
    false
  }
  pub fn clean_up(&self) { unsafe {
    self.stop();
    for shader in &self.shaders {
      DetachShader(self.program, shader.id);
      DeleteShader(shader.id);
    }
    DeleteProgram(self.program);
  }}
  pub fn add_shader(&mut self, shader_type: GLenum) -> &mut Self {
    if self.done { return self }
    let shader_id;
    unsafe {
      shader_id = CreateShader(shader_type);
    }
    assert!(shader_id != 0);
    let path: &str = &format!("res/glsl/{}.{}", self.name, &get_ext(shader_type));
    let src = match File::open(&Path::new(path)) {
      Ok(file) => {
        let mut buf = BufReader::new(file);
        let mut _src = String::new();
        let _ = buf.read_to_string(&mut _src); // Lazily not checking for error
        _src
      },
      _ => panic!("Failed to read shader file: {}", path)
    };
    self.shaders.push(ShaderSrc::new(shader_type, shader_id, CString::new(src.as_bytes()).unwrap() ));
    self
  }
  pub fn compile_shaders(&mut self) -> &mut Self { unsafe {
    if self.done { return self }
    for shader in &self.shaders {
      // println!("{:?}", &shader.src);
      // Attempt to compile the shader
      ShaderSource(shader.id, 1, &shader.src.as_ptr(), ptr::null());
      CompileShader(shader.id);
      // Get the compile status
      let mut status = FALSE as GLint;
      GetShaderiv(shader.id, COMPILE_STATUS, &mut status);
      // Fail on error
      if status != (TRUE as GLint) {
        println!("Shader compile failed.");
        let mut buffer = [0u8; 512];
        let mut length: i32 = 0;
        GetShaderInfoLog(shader.id, buffer.len() as i32, &mut length,
          buffer.as_mut_ptr() as *mut i8);
        println!("Compiler log (length: {}):\n{}", length,
          from_utf8(CStr::from_ptr(transmute(&buffer)).to_bytes()).unwrap());
      } else { println!("Shader compiled"); }
    }
  } self }
  pub fn link(&mut self) -> &mut Self { unsafe {
    if self.done { return self }
    let program = CreateProgram();
    self.program = program;
    for shader in &self.shaders {
      println!("Attach Shader: {} {}", shader.kind(), shader.id);
      AttachShader(program, shader.id);
    }
    // self.start();
    // let cname = CString::new(b"out_Color").unwrap();
    // BindFragDataLocation(self.program, 0, cname.as_ptr() );
    self.bind_attributes();
    LinkProgram(program);
    //ValidateProgram(program); // Maybe not needed?
    // Get the link status
    let mut status = FALSE as GLint;
    GetProgramiv(program, LINK_STATUS, &mut status);
    // Fail on error
    if status != (TRUE as GLint) {
      println!("Program link failed. Program: {}", program);
      let mut buffer = [0u8; 512];
      let mut length: i32 = 0;
      GetProgramInfoLog(program, buffer.len() as i32, &mut length,
        buffer.as_mut_ptr() as *mut i8);
      println!("Linker log (length: {}):\n{}", length,
        from_utf8(CStr::from_ptr(transmute(&buffer)).to_bytes()).unwrap());
    } else {
      println!("{} shader linked. Program: {}", self.name, program);
    }
    self.done = true;
    self
  }}
}

pub fn get_attrib_location(program: GLuint, name: &str) -> GLint {
  let cname = CString::new(name.as_bytes()).unwrap();
  let location = unsafe { GetAttribLocation(program, cname.as_ptr()) };
  if location < 0 {
    panic!("Failed to get attribute location: {}", name);
  }
  location
}
pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
  let cname = CString::new(name.as_bytes()).unwrap();
  let location = unsafe { GetUniformLocation(program, cname.as_ptr()) };
  // if location < 0 {
  //   println!("Failed to get uniform location: {}", name);
  // }
  location
}
pub fn get_ext(kind: GLenum) -> String {
  match kind {
    VERTEX_SHADER => { "glslv".to_string() }
    FRAGMENT_SHADER => { "glslf".to_string() }
    _ => panic!("Unknown Shader Type for file extension.")
  }
}
