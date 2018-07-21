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
use nalgebra::Matrix4;

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

pub struct Shader {
  pub name: String,
  pub program: GLuint,
  using: bool,
  pub shaders: Vec<(GLenum, GLuint)>,
  pub vars: Vec<ShaderVar>,
  pub unis: Vec<ShaderUni>,
}

impl Shader {
  pub fn new(name: &str) -> Self {
    Shader { name: format!("{}", name), program: 0, using: false, shaders: Vec::new(), vars: Vec::new(), unis: Vec::new() }
  }
  pub fn load_defaults(&mut self) -> &mut Self {
    self
    .load_vert_shader()
    .load_frag_shader()
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
  pub fn add_uniform(&mut self, name: &str) -> &mut Self {
    self.unis.push(ShaderUni::new(name));
    self
  }
  pub fn bind_attributes(&mut self) -> &mut Self { unsafe {
    let mut count = 0 as GLint;
    for attrib in &mut self.vars {
      BindAttribLocation(self.program, count as GLuint, str_ptr(&attrib.var_name));
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
    println!("Uniform name not found.");
    -1 as GLint
  }
  pub fn load_int(&self, name: &str, value: GLint) { unsafe {
    Uniform1i(self.get_uniform_id(name), value);
  }}
  pub fn load_float(&self, name: &str, value: GLfloat) { unsafe {
    Uniform1f(self.get_uniform_id(name), value);
  }}
  pub fn load_vec_4f(&self, name: &str, vector: Vector4f) { unsafe {
    Uniform4f(self.get_uniform_id(name), vector.x, vector.y, vector.z, vector.w);
  }}
  pub fn load_vec_3f(&self, name: &str, vector: Vector3f) { unsafe {
    Uniform3f(self.get_uniform_id(name), vector.x, vector.y, vector.z);
  }}
  pub fn load_vec_2f(&self, name: &str, vector: Vector2f) { unsafe {
    Uniform2f(self.get_uniform_id(name), vector.x, vector.y);
  }}
  pub fn load_bool(&self, name: &str, value: bool) { unsafe {
    Uniform1f(self.get_uniform_id(name), if value { 1.0 as GLfloat } else { 0.0 as GLfloat })
  }}
  pub fn load_matrix(&self, name: &str, matrix: &Matrix4<f32>) { unsafe {
    UniformMatrix4fv(self.get_uniform_id(name), 1, 0, transmute(matrix.as_ref()));
  }}
  pub fn load_vert_shader(&mut self) -> &mut Self {
    let st = VERTEX_SHADER;
    let shader;
    unsafe {
      shader = CreateShader(VERTEX_SHADER);
    }
    assert!(shader != 0);
    self.shaders.push((VERTEX_SHADER, compile_shader(&self.name, st, shader)));
    println!("Vertex shader loaded.");
    self
  }
  pub fn load_frag_shader(&mut self) -> &mut Self {
    let st = FRAGMENT_SHADER;
    let shader;
    unsafe {
      shader = CreateShader(FRAGMENT_SHADER);
    }
    assert!(shader != 0);
    self.shaders.push((FRAGMENT_SHADER, compile_shader(&self.name, st, shader)));
    println!("Fragment shader loaded.");
    self
  }
  pub fn link(&mut self) -> &mut Self { unsafe {
    let program = CreateProgram();
    self.program = program;
    for (_, shader) in &self.shaders {
      AttachShader(program, *shader);
    }
    //self.start();
    BindFragDataLocation(self.program, 0, str_ptr("out_Color") );
    self.bind_attributes();
    LinkProgram(program);
    //ValidateProgram(program); // Maybe not needed?
    println!("Program linked.");
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
    self
  }}
  pub fn start(&mut self) { unsafe {
    if !self.using {
      UseProgram(self.program);
      self.using = true;
    }
  }}
  pub fn stop(&mut self) { unsafe {
    if self.using {
      UseProgram(0);
      self.using = false;
    }
  }}
  pub fn clean_up(&mut self) { unsafe {
    self.stop();
    for (_, shader) in &self.shaders {
      DetachShader(self.program, *shader);
      DeleteShader(*shader);
    }
    DeleteProgram(self.program);
  }}
}

pub fn compile_shader(name: &str, shader_type: GLenum, shader: GLuint) -> GLuint {
  let st: &str = match shader_type {
    VERTEX_SHADER => { "glslv" }
    FRAGMENT_SHADER => { "glslf" }
    _ => panic!("No support for given shader type.")
  };
  let path: &str = &format!("res/glsl/{}.{}", name, st);
  let src = match File::open(&Path::new(path)) {
    Ok(file) => {
      println!("Read shader file: {}", path);
      let mut buf = BufReader::new(file);
      let mut _src = String::new();
      let _ = buf.read_to_string(&mut _src); // Lazily not checking for error
      println!("Shader to string.");
      _src
    },
    _ => panic!("Failed to read shader file: {}", path)
  };
  unsafe {
    println!("{}", &src);
    // Attempt to compile the shader
    // let c_str = CString::new(src.as_bytes()).unwrap();
    ShaderSource(shader, 1, &str_ptr(&src), ptr::null()); // &c_str.as_ptr()
    println!("Compiling shader.");
    CompileShader(shader);
    println!("Shader compiled.");
    // Get the compile status
    let mut status = FALSE as GLint;
    GetShaderiv(shader, COMPILE_STATUS, &mut status);
    // Fail on error
    if status != (TRUE as GLint) {
      println!("Shader compile failed.");
      let mut buffer = [0u8; 512];
      let mut length: i32 = 0;
      GetShaderInfoLog(shader, buffer.len() as i32, &mut length,
        buffer.as_mut_ptr() as *mut i8);
      println!("Compiler log (length: {}):\n{}", length,
        from_utf8(CStr::from_ptr(transmute(&buffer)).to_bytes()).unwrap());
    }
  }
  shader
}
pub fn get_attrib_location(program: GLuint, name: &str) -> GLint {
  let location = unsafe { GetAttribLocation(program, str_ptr(name)) };
  assert!(location != -1);
  location
}
pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
  let location = unsafe { GetUniformLocation(program, str_ptr(name)) };
  assert!(location != -1);
  location
}
pub fn str_ptr(conv_str: &str) -> *const i8 {
  match CString::new(conv_str.as_bytes()) {
    Ok(out) => { out.as_ptr() }
    _ => panic!("Could not convert &str to CString.")
  }
}
