
use gl::*;
use gl::types::{GLenum, GLuint, GLint, GLfloat, }; // GLchar, GLsizeiptr, GLboolean, 
// use std::mem;
use std::ptr;
use std::str;
use std::str::from_utf8;
use std::ffi::CStr;
use std::ffi::CString;
// use std::mem::transmute;
// use cgmath::{Matrix, Matrix4, };

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use util::rmatrix::Matrix4f;
use util::{ Vector2f, Vector3f, Quaternion, Arc, Mutex, HashSet };

pub struct ShaderVar {
    var_name: String,
    var_id: GLint,
}
impl ShaderVar {
  pub fn new(name: &str) -> Self {
    ShaderVar {
      var_name: name.to_string(),
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
      var_name: name.to_string(),
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
    let name = name.to_string();
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
    ShaderSrc { kind, id, src, }
  }
  pub fn kind(&self) -> &str {
    match self.kind {
      VERTEX_SHADER => { "Vertex" }
      GEOMETRY_SHADER => { "Geometry" }
      // TESS_CONTROL_SHADER => { "Tess Control" }
      // TESS_EVALUATION_SHADER => { "Tess Evaluation" }
      FRAGMENT_SHADER => { "Fragment" }
      COMPUTE_SHADER => { "Compute" }
      _ => { "Unknown" }
    }
  }
}

pub struct ShaderTypesUsed {
  pub defaults: bool,
  pub geometry: bool,
  // pub tess_control: bool,
  // pub tess_eval: bool,
  pub compute: bool, 
}
impl Default for ShaderTypesUsed {
  fn default() -> Self {
    Self {
      defaults: true,
      geometry: false,
      // tess_control: false,
      // tess_eval: false,
      compute: false,
    }
  }
}
impl ShaderTypesUsed {
  pub fn use_geometry(&mut self) -> &mut Self {
    self.defaults = true;
    self.compute = false;
    self.geometry = true;
    self
  }
  // pub fn use_tesselation(&mut self) -> &mut Self {
  //   // TODO: Tessellation
  //   self
  // }
  pub fn use_compute(&mut self) -> &mut Self {
    self.defaults = false;
    self.compute = true;
    self.geometry = false;
    // self.tess_control = false;
    // self.tess_eval = false;
    self
  }
}

pub struct Shader {
  pub name: String,
  pub program: GLuint,
  pub done: bool,
  pub shader_types_used: ShaderTypesUsed,
  pub shaders: Vec<ShaderSrc>,
  pub vars: Vec<ShaderVar>,
  pub unis: Vec<ShaderUni>,
  unis_unavailable: Arc<Mutex<HashSet<String>>>,
}

impl Shader {
  pub fn new(name: &str) -> Self {
    Shader { 
      name: name.to_string(), program: 0, done: false,
      shader_types_used: ShaderTypesUsed::default(),
      shaders: Vec::new(), vars: Vec::new(), unis: Vec::new(), 
      unis_unavailable: Arc::new(Mutex::new(HashSet::new())),
    }
  }
  pub fn start(&self) { unsafe {
    UseProgram(self.program);
  }}
  pub fn stop(&self) { unsafe {
    UseProgram(0);
  }}
  pub fn clean_up(&self) { unsafe {
    self.stop();
    for shader in &self.shaders {
      DetachShader(self.program, shader.id);
      DeleteShader(shader.id);
    }
    DeleteProgram(self.program);
  }}
  pub fn use_geometry(&mut self) -> &mut Self {
    self.shader_types_used.use_geometry();
    self
  }
  // pub fn use_tesselation(&mut self) -> &mut Self {
  //   // TODO: Tessellation
  //   self.shader_types_used.use_tesselation();
  //   self
  // }
  pub fn use_compute(&mut self) -> &mut Self {
    self.shader_types_used.use_compute();
    self
  }
  pub fn load_defaults(&mut self) -> &mut Self {
    if self.shader_types_used.defaults {
      self.load_vert_shader();
      if self.shader_types_used.geometry {
        self.load_geom_shader();
      }
      self.load_frag_shader();
    } else if self.shader_types_used.compute {
      self.load_comp_shader();
    } else {
      panic!("Shader: shader_types_used in unsupported state. Not a Vertex -> .. -> Fragment chain or a Compute shader")
    }
    self.compile_shaders()
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
  pub fn load_quaternion(&self, name: &str, quat: Quaternion) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_quaternion") { return }
    Uniform4f(id, quat.w, quat.x, quat.y, quat.z);
  }}
  pub fn load_vec_3f(&self, name: &str, vector: Vector3f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_vec_3f") { return }
    Uniform3f(id, vector.x, vector.y, vector.z);
  }}
  pub fn load_vec_2f(&self, name: &str, vector: Vector2f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_vec_2f") { return }
    Uniform2f(id, vector.x, vector.y);
  }}
  pub fn load_matrix(&self, name: &str, matrix: &Matrix4f) { unsafe {
    let id = self.get_uniform_id(name);
    if self.check_id(id, name, "load_matrix") { return }
    UniformMatrix4fv(id, 1, 0, &matrix.matrix[0] as *const f32 );
  }}
  fn check_id(&self, id: GLint, name: &str, caller: &str) -> bool {
    let mut unis_unavailable = self.unis_unavailable.lock().unwrap();
    let test = unis_unavailable.contains(name);
    if test { return true } else if id < 0 { 
      unis_unavailable.insert(name.to_string());
      println!("{}(): Uniform {} not available for shader {}", caller, name, self.name); 
      return true;
    }
    false
  }
  fn load_vert_shader(&mut self) -> &mut Self {
    self.add_shader(VERTEX_SHADER)
  }
  fn load_geom_shader(&mut self) -> &mut Self {
    self.add_shader(GEOMETRY_SHADER)
  }
  fn load_frag_shader(&mut self) -> &mut Self {
    self.add_shader(FRAGMENT_SHADER)
  }
  fn load_comp_shader(&mut self) -> &mut Self {
    self.add_shader(COMPUTE_SHADER)
  }
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
      let mut status = i32::from(FALSE);
      GetShaderiv(shader.id, COMPILE_STATUS, &mut status);
      // Fail on error
      if status != i32::from(TRUE) {
        println!("Shader compile failed.");
        let mut buffer = [0u8; 512];
        let mut length: i32 = 0;
        GetShaderInfoLog(shader.id, buffer.len() as i32, &mut length,
          buffer.as_mut_ptr() as *mut i8);
        println!("Compiler log (length: {}):\n{}", length,
          from_utf8(CStr::from_ptr(&buffer as *const [u8; 512] as *const i8).to_bytes()).unwrap());
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
    let mut status = i32::from(FALSE);
    GetProgramiv(program, LINK_STATUS, &mut status);
    // Fail on error
    if status != i32::from(TRUE) {
      println!("Program link failed. Program: {}", program);
      let mut buffer = [0u8; 512];
      let mut length: i32 = 0;
      GetProgramInfoLog(program, buffer.len() as i32, &mut length,
        buffer.as_mut_ptr() as *mut i8);
      println!("Linker log (length: {}):\n{}", length,
        from_utf8(CStr::from_ptr(&buffer as *const [u8; 512] as *const i8).to_bytes()).unwrap());
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
  unsafe { GetUniformLocation(program, cname.as_ptr()) }
}
pub fn get_ext(kind: GLenum) -> String {
  match kind {
    VERTEX_SHADER => { "glslv".to_string() }
    GEOMETRY_SHADER => { "glslg".to_string() }
    TESS_CONTROL_SHADER => { "glsltc".to_string() }    // TODO: Tessellation Control Shaders
    TESS_EVALUATION_SHADER => { "glslte".to_string() } // TODO: Tessellation Evaluation Shaders
    FRAGMENT_SHADER => { "glslf".to_string() }
    COMPUTE_SHADER => { "glslc".to_string() }
    _ => panic!("shader::get_ext(): Unknown Shader Type")
  }
}
