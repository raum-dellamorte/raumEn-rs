pub mod lighting;
pub mod model;
//pub mod compute;

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
}

impl ShaderUni {
  pub fn new(name: &str) -> Self {
    ShaderUni {
      var_name: format!("{}", name),
      var_id: -1 as GLint,
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
}

impl Shader {
  pub fn new(name: &str) -> Self {
    Shader { 
      name: format!("{}", name), program: 0, done: false,
      shaders: Vec::new(), vars: Vec::new(), unis: Vec::new() 
    }
  }
  pub fn load_defaults(&mut self) -> &mut Self {
    self
    .load_vert_shader()
    .load_frag_shader()
    .compile_shaders()
    .link()
    .gen_uniforms()
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
  pub fn load_int(&self, name: &str, value: GLint) { unsafe {
    Uniform1i(self.get_uniform_id(name), value);
  }}
  pub fn load_float(&self, name: &str, value: GLfloat) { unsafe {
    Uniform1f(self.get_uniform_id(name), value);
  }}
  pub fn load_vec_4f(&self, name: &str, vector: &Vector4f) { unsafe {
    Uniform4f(self.get_uniform_id(name), vector.x, vector.y, vector.z, vector.w);
  }}
  pub fn load_vec_3f(&self, name: &str, vector: &Vector3f) { unsafe {
    Uniform3f(self.get_uniform_id(name), vector.x, vector.y, vector.z);
  }}
  pub fn load_vec_2f(&self, name: &str, vector: &Vector2f) { unsafe {
    Uniform2f(self.get_uniform_id(name), vector.x, vector.y);
  }}
  pub fn load_bool(&self, name: &str, value: bool) { unsafe {
    Uniform1f(self.get_uniform_id(name), if value { 1.0 as GLfloat } else { 0.0 as GLfloat })
  }}
  pub fn load_matrix(&self, name: &str, matrix: &[f32; 16]) { unsafe {
    UniformMatrix4fv(self.get_uniform_id(name), 1, 0, transmute(&matrix[0]) );
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
  pub fn clean_up(&mut self) { unsafe {
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
      println!("Model shader linked. Program: {}", program);
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
  if location < 0 {
    panic!("Failed to get uniform location: {}", name);
  }
  location
}
pub fn get_ext(kind: GLenum) -> String {
  match kind {
    VERTEX_SHADER => { "glslv".to_string() }
    FRAGMENT_SHADER => { "glslf".to_string() }
    _ => panic!("Unknown Shader Type for file extension.")
  }
}
