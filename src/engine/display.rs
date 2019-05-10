
use gl::Viewport;

use util::Matrix4f;

pub struct Display {
  pub w: u32,
  pub h: u32,
  pub aspect_ratio: f32,
  pub proj_mat: Matrix4f,
}
impl Default for Display {
  fn default() -> Self {
    Self {
      w: 640,
      h: 480,
      aspect_ratio: 1.333334,
      proj_mat: Matrix4f::new(),
    }
  }
}
impl Display {
  pub fn dimensions(&self) -> (u32, u32) {
    (self.w, self.h)
  }
  pub fn update_size(&mut self, dimensions: (u32, u32)) {
    let (w, h) = dimensions;
    unsafe { Viewport(0, 0, w as i32, h as i32); }
    self.w = w;
    self.h = h;
    self.aspect_ratio = w as f32 / h as f32;
    self.projection();
  }
  fn projection(&mut self) {
    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;
    let y_scale = 1_f32 / (fov / 2_f32).tan();
    let frustum_length = zfar - znear;
    
    self.proj_mat.set_identity();
    self.proj_mat.set_m00(y_scale * self.aspect_ratio);
    self.proj_mat.set_m11(y_scale);
    self.proj_mat.set_m22(-((zfar + znear) / frustum_length));
    self.proj_mat.set_m23(-1_f32);
    self.proj_mat.set_m32(-(2_f32 * znear * zfar) / frustum_length);
    self.proj_mat.set_m33(0_f32);
  }
}
