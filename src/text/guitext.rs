


use gamemgr::GameMgr;
use text::{RTextMesh, }; // RFontType, 
use util::rvector::{Vector2f, Vector3f, };

pub struct GuiText {
  pub font: String,
  pub label: String,
  pub text: String,
  pub position: Vector2f,
  pub font_size: f32,
  pub line_max_size: f32,
  pub is_centered: bool,
  pub num_of_lines: u32,
  pub colour: Vector3f,
  pub text_mesh_vao: u32,
  pub vertex_count: u32,
  pub loaded: bool,
}
impl GuiText {
  pub fn new(font: &str, label: &str, text: &str, position: Vector2f, font_size: f32, line_max_size: f32, is_centered: bool) -> Self {
    Self {
      font: font.to_owned(),
      label: label.to_owned(),
      text: text.to_owned(),
      position: position,
      font_size: font_size,
      line_max_size: line_max_size,
      is_centered: is_centered,
      num_of_lines: 0,
      colour: Vector3f::blank(),
      text_mesh_vao: 0,
      vertex_count: 0,
      loaded: false,
    }
  }
  pub fn load(&mut self, mgr: GameMgr) {
    if self.loaded { return }
    let fonts_arc = mgr.fonts.clone();
    let mut fonts = fonts_arc.lock().unwrap();
    if let Some(font) = fonts.get_mut(&self.font) {
      let mut tmp = self.copy_vals();
      let data: RTextMesh = font.load_text(&mut tmp);
      let vao = {
        let _arc = mgr.loader.clone();
        let mut loader = _arc.lock().unwrap();
        loader.load_to_vao_2d(&data.verts, &data.tex_coords)
      };
      self.set_mesh_info(vao, data.vert_count);
      self.num_of_lines = tmp.num_of_lines;
      self.loaded = true;
    }
  }
  pub fn update(&mut self, mgr: GameMgr, text: &str) {
    self.text = text.to_string();
    if self.text_mesh_vao == 0 { return }
    {
      let _arc = mgr.loader.clone();
      let mut loader = _arc.lock().unwrap();
      loader.rm_vao(self.text_mesh_vao);
    }
    self.loaded = false;
    self.load(mgr);
  }
  pub fn set_colour(&mut self, r: f32, g: f32, b: f32) { self.colour.from_f32(r, g, b); }
  pub fn set_mesh_info(&mut self, vao: u32, vert_count: u32) {
    self.text_mesh_vao = vao;
    self.vertex_count = vert_count;
  }
  fn copy_vals(&self) -> GuiTextVals {
    GuiTextVals {
      text: self.text.clone(),
      font_size: self.font_size,
      line_max_size: self.line_max_size,
      is_centered: self.is_centered,
      num_of_lines: self.num_of_lines,
    }
  }
}

pub struct GuiTextVals {
  pub text: String,
  pub font_size: f32,
  pub line_max_size: f32,
  pub is_centered: bool,
  pub num_of_lines: u32,
}