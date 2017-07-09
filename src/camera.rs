
use glium::{Display, Depth, DepthTest, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use glium::draw_parameters::BackfaceCullingMode;
use glium::texture::CompressedSrgbTexture2d;

use entities::entity::Entity;
use entities::position::PosMarker;
use util::rmatrix::Matrix4f;
use util::rvector::{RVec, Vector3f, XVEC, YVEC}; // , ZVEC
use util::rvertex::Vertex;

pub struct Camera {
  pub display: Display,
  pub target: Option<Frame>,
  pub target_dimensions: (u32, u32),
  pub pos: Vector3f,
  pub posBak: Vector3f,
  pub pitch: f32,
  pub pitchBak: f32,
  pub yaw: f32,
  pub yawBak: f32,
  pub roll: f32,
  pub rollBak: f32,
  pub distFromFocusPos: f32,
  pub angleAroundFocusPos: f32,
  
  toPos: Vector3f,
  toFocusPos: Vector3f,
  
  pub viewMat: Matrix4f,
  pub projMat: Matrix4f,
}

impl Camera {
  pub fn create(display: Display) -> Self {
    Camera {
      display: display,
      target: None,
      target_dimensions: (0, 0),
      pos: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      posBak: Vector3f {x: 0_f32, y: 5_f32, z: 0_f32},
      pitch: 25_f32,
      pitchBak: 25_f32,
      yaw: 0_f32,
      yawBak: 0_f32,
      roll: 0_f32,
      rollBak: 0_f32,
      distFromFocusPos: 40_f32,
      angleAroundFocusPos: 0_f32,
      toPos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      toFocusPos: Vector3f {x: 0_f32, y: 0_f32, z: 0_f32},
      viewMat: Matrix4f::new(),
      projMat: Matrix4f::new(),
    }
  }
  
  pub fn load_target(&mut self) {
    self.target = Some(self.display.draw());
    self.target_dimensions = self.target.as_ref().unwrap().get_dimensions()
  }
  
  pub fn finish(&mut self) {
    self.target.take().unwrap().finish().unwrap();
  }
  
  pub fn draw_entity(&mut self, ent: &mut Entity) {
    let mut frame: Option<Frame> = self.target.take();
    self.draw_entity_surface(ent, &mut frame);
    self.target = frame;
  }
  
  pub fn draw_entity_surface<S>(&mut self, ent: &mut Entity, fbo: &mut Option<S>) where S: Surface {
    let light = [0.0, 1000.0, -7000.0_f32];
    let mesh = ent.model.mesh.as_ref().unwrap().buffers.as_ref().unwrap();
    let trans = ent.marker.transformation();
    let view = self.view_matrix();
    let proj = self.projection();
    let params = DrawParameters {
      depth: Depth {
        test: DepthTest::IfLess,
        write: true,
        .. Default::default()
      },
      backface_culling: BackfaceCullingMode::CullClockwise,
      .. Default::default()
    };
    match ent.model.texture.as_ref() {
      Some(texture) => self.draw_with_texture(texture, &params, &mesh.verts, &mesh.indcs, trans, view, proj, light, fbo),
      None => self.draw_with_color(&params, &mesh.verts, &mesh.indcs, trans, view, proj, light, fbo),
    };
    
  }
  
  fn draw_with_texture<S>(&mut self, texture: &CompressedSrgbTexture2d, params: &DrawParameters,
      verts: &VertexBuffer<Vertex>, indcs: &IndexBuffer<u16>,
      trans: [[f32; 4]; 4], view: [[f32; 4]; 4], proj: [[f32; 4]; 4], light: [f32; 3],
      fbo: &mut Option<S>) where S: Surface {
    let program = Program::from_source(&self.display, vertex_with_texture, fragment_with_texture, None).unwrap();
    match fbo.as_mut() {
      Some(target) => target.draw(
                        verts, indcs, &program,
                        &uniform! {
                          transform: trans,
                          view: view,
                          projection: proj,
                          u_light: light,
                          tex: texture,
                        },
                        params
                      ).unwrap(),
      None => (),
    };
  }
  
  fn draw_with_color<S>(&mut self, params: &DrawParameters, verts: &VertexBuffer<Vertex>, indcs: &IndexBuffer<u16>,
      trans: [[f32; 4]; 4], view: [[f32; 4]; 4], proj: [[f32; 4]; 4], light: [f32; 3],
      fbo: &mut Option<S>) where S: Surface {
    let program = Program::from_source(&self.display, vertex_no_texture, fragment_no_texture, None).unwrap();
    fbo.as_mut().unwrap().draw(
      verts, indcs, &program,
      &uniform! {
        transform: trans,
        view: view,
        projection: proj,
        u_light: light },
      params
    ).unwrap();
  }
  
  pub fn view_matrix(&mut self) -> [[f32; 4]; 4] { self.createViewMatrix(); self.viewMat.as_slice() }
  
  pub fn get_dimensions(&self) -> (u32, u32) {
    match self.target.as_ref() {
      Some(target) => target.get_dimensions(),
      None => (0, 0),
    }
  }
  
  pub fn projection(&mut self) -> [[f32; 4]; 4] {
    let (width, height) = self.target_dimensions;
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;
    let yScale = 1_f32 / (fov / 2_f32).tan();
    let frustumLength = zfar - znear;
    
    self.projMat.setIdentity();
    self.projMat.m00 = yScale / aspect_ratio;
    self.projMat.m11 = yScale;
    self.projMat.m22 = -((zfar + znear) / frustumLength);
    self.projMat.m23 = -1_f32;
    self.projMat.m32 = -(2_f32 * znear * zfar) / frustumLength;
    self.projMat.m33 = 0_f32;
    self.projMat.as_slice()
  }
  
  pub fn attachListeners() {
    //var camera = this
    //DisplayMgr.mouse.scroll.setListener { dx: f32, dy: f32 ->
    //  camera.distFromFocusPos -= dy * 0.5
    //}
  }
  
  pub fn store(&mut self) {
    self.posBak.from_v3f(&self.pos);
    self.pitchBak = self.pitch;
    self.yawBak = self.yaw;
    self.rollBak = self.roll;
  }

  pub fn restore(&mut self) {
    self.pos.from_v3f(&self.posBak);
    self.pitch = self.pitchBak;
    self.yaw = self.yawBak;
    self.roll = self.rollBak;
  }

  pub fn calc_pos(&mut self, follow: &PosMarker) {
    self.calcPitch();
    self.calcAngle();
    self.calcCamPos(follow);
  }
  
  fn calcPitch(&mut self) {
    //if (DisplayMgr.mouse.isButtonDown(2)) self.pitch -= DisplayMgr.mouse.pos.getDY() * 0.1
  }

  fn calcAngle(&mut self) {
    //if (DisplayMgr.mouse.isButtonDown(2)) self.angleAroundFocusPos -= DisplayMgr.mouse.pos.getDX() * 0.3
  }

  fn calcCamPos(&mut self, follow: &PosMarker) {
    let hDist: f32 = self.calcHDistance();
    let vDist: f32 = self.calcVDistance() + 10_f32;
    let theta = follow.ry + self.angleAroundFocusPos;
    let xOffset = hDist * theta.to_radians().sin();
    let zOffset = hDist * theta.to_radians().cos();
    self.pos.x = follow.pos.x - xOffset;
    self.pos.z = follow.pos.z - zOffset;
    self.pos.y = follow.pos.y + vDist;
    self.yaw = 180_f32 - (follow.ry + self.angleAroundFocusPos);
  }

  fn calcHDistance(&self) -> f32 {self.distFromFocusPos * self.pitch.to_radians().cos()}
  fn calcVDistance(&self) -> f32 {self.distFromFocusPos * self.pitch.to_radians().sin()}

  pub fn reflection(&mut self, height: f32) {
    self.store();
    self.pos.y -= 2.0 * (self.pos.y - height); // y -= dist
    self.invertPitch();
  }

  pub fn invertPitch(&mut self) {
    self.pitch = -self.pitch;
  }

  pub fn distToPos(&mut self, vec: &Vector3f) -> f32 {
    vec.subTo(&self.pos, &mut self.toPos);
    self.toPos.len()
  }

  pub fn angleToEntity(&mut self, focus_pos: &Vector3f, entity: &mut Entity) -> f32 {
    entity.distance = self.distToPos(&entity.marker.pos);
    self.toPos.normalize();
    focus_pos.subTo(&self.pos, &mut self.toFocusPos);
    self.toFocusPos.normalize();
    self.toFocusPos.dot(&self.toPos)
  }
  
  pub fn createViewMatrix(&mut self) {
    self.viewMat.setIdentity();
    self.viewMat.rotate(self.pitch.to_radians(), &XVEC);
    self.viewMat.rotate(self.yaw.to_radians(), &YVEC);
    let pos = self.pos;
    let mut negCam = Vector3f::blank();
    pos.negateTo(&mut negCam);
    self.viewMat.translate_v3f(&negCam);
  }
}

const vertex_no_texture: &str = r#"
#version 400
in vec3 position;
in vec3 normal;

out vec3 surface_normal;
out vec3 v_position;
out vec3 toLightVector;
out vec3 toCameraVector;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 u_light;

void main() {
  v_position = gl_Position.xyz / gl_Position.w;
  vec4 worldPos = transform * vec4(position, 1.0);
  vec4 posRelToCam = view * worldPos;
  gl_Position = projection * posRelToCam;
  
  surface_normal = (transform * vec4(normal, 0.0)).xyz;
  
  toLightVector = u_light - worldPos.xyz;
  toCameraVector = (inverse(view) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
}
"#;
const fragment_no_texture: &str = r#"
#version 400
in vec3 surface_normal;
in vec3 v_position;
in vec3 toLightVector;
in vec3 toCameraVector;

out vec4 color;

const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
vec3 lightColour = vec3(1.0);
vec3 unitNormal = normalize(surface_normal);
vec3 unitCameraVector = normalize(toCameraVector);
vec3 unitLightVector = normalize(toLightVector);

float diffuse = max(dot(unitNormal, unitLightVector), 0.0);
float specular = max(dot(reflect(unitLightVector, unitNormal), unitCameraVector), 0.0);

color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}
"#;

const vertex_with_texture: &str = r#"
#version 400
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 v_position;
out vec3 surface_normal;
out vec2 pass_tex_coords;
out vec3 toLightVector;
out vec3 toCameraVector;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 u_light;

void main() {
  v_position = gl_Position.xyz / gl_Position.w;
  vec4 worldPos = transform * vec4(position, 1.0);
  vec4 posRelToCam = view * worldPos;
  gl_Position = projection * posRelToCam;
  
  surface_normal = (transform * vec4(normal, 0.0)).xyz;
  
  pass_tex_coords = tex_coords;
  toLightVector = u_light - worldPos.xyz;
  toCameraVector = (inverse(view) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
}
"#;
const fragment_with_texture: &str = r#"
#version 400
in vec3 v_position;
in vec3 surface_normal;
in vec2 pass_tex_coords;
in vec3 toLightVector;
in vec3 toCameraVector;

out vec4 color;

uniform sampler2D tex;

const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
  vec3 lightColour = vec3(1.0);
  vec3 unitNormal = normalize(surface_normal);
  vec3 unitCameraVector = normalize(toCameraVector);
  vec3 unitLightVector = normalize(toLightVector);
  
  float diffuse = max(dot(unitNormal, unitLightVector), 0.0);
  float specular = max(dot(reflect(unitLightVector, unitNormal), unitCameraVector), 0.0);
  
  vec4 texture_colour = texture(tex, pass_tex_coords);
  if(texture_colour.a < 0.5){
    discard;
  }
  
  color = texture_colour + vec4(specular * specular_color, 1.0);
}
"#;
