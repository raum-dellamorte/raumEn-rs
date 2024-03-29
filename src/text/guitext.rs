
use {
  specs::{World, WorldExt, },
  Loader,
  text::{
    // RFontType, 
    TextMgr, RTextMesh, 
  },
  util::{
    Vector2f, Vector3f, 
    rgl::*,
  },
};

pub struct GuiText {
  pub font: String,
  pub label: String,
  pub text: String,
  pub position: Vector2f<f32>,
  pub font_size: f32,
  pub line_max_size: f32,
  pub is_centered: bool,
  pub num_of_lines: u32,
  pub colour: Vector3f<f32>,
  pub text_mesh_vao: VaoID,
  pub vertex_count: VertexCount,
  pub loaded: bool,
}
impl GuiText {
  pub fn new(font: &str, label: &str, text: &str, position: Vector2f<f32>, font_size: f32, line_max_size: f32, is_centered: bool) -> Self {
    Self {
      font: font.to_owned(),
      label: label.to_owned(),
      text: text.to_owned(),
      position,
      font_size,
      line_max_size,
      is_centered,
      num_of_lines: 0,
      colour: Vector3f::blank(),
      text_mesh_vao: VaoID(0),
      vertex_count: VertexCount(0),
      loaded: false,
    }
  }
  pub fn load(&mut self, textmgr: &mut TextMgr, world: &World) {
    if self.loaded { return }
    // println!("Attempting to load guitext to vao");
    let mut data: Option<RTextMesh> = None;
    {
      if let Some(font) = textmgr.fonts.get_mut(&self.font) {
        let mut tmp = self.copy_vals();
        data = Some(font.load_text(&mut tmp));
        // println!("  data: {:?}", &data);
        self.num_of_lines = tmp.num_of_lines;
      }
    }
    // println!("  stage 2");
    let data = data.unwrap();
    let mut loader = world.write_resource::<Loader>();
    let vao = loader.load_to_vao_2d(&data.verts, &data.tex_coords);
    // println!("  vao: {:?}", vao);
    self.set_mesh_info(vao, data.vert_count);
    self.loaded = true;
  }
  pub fn update_text(&mut self, textmgr: &mut TextMgr, world: &World, text: &str) {
    self.text = text.to_string();
    self.update_size(textmgr, world);
  }
  pub fn update_size(&mut self, textmgr: &mut TextMgr, world: &World) {
    if self.text_mesh_vao == VaoID(0) { return }
    {
      let mut loader = world.write_resource::<Loader>();
      loader.rm_vao(self.text_mesh_vao);
    }
    self.loaded = false;
    // println!("Reloading GuiText");
    self.load(textmgr, world);
  }
  pub fn set_colour(&mut self, r: f32, g: f32, b: f32) { self.colour.copy_from_float(r, g, b); }
  pub fn set_mesh_info(&mut self, vao: VaoID, vert_count: VertexCount) {
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
#[derive(Debug)]
pub struct GuiTextVals {
  pub text: String,
  pub font_size: f32,
  pub line_max_size: f32,
  pub is_centered: bool,
  pub num_of_lines: u32,
}
