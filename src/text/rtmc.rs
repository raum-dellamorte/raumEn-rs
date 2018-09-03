


use gamemgr::GameMgr;
use text::{RChar, RLine, RWord, RTextMesh, SPACE_ASCII, LINE_HEIGHT, }; // RFontType, 
use text::guitext::GuiTextVals;
use text::metafile::MetaFile;

#[derive(Debug)]
pub struct RTextMeshCreator {
  pub line_ht: f32,
  pub space_ascii: u32,
  pub metadata: MetaFile,
}
impl RTextMeshCreator {
  pub fn new(mgr: GameMgr, file: &str) -> Self {
    Self {
      line_ht: LINE_HEIGHT,
      space_ascii: SPACE_ASCII,
      metadata: MetaFile::new(mgr, file),
    }
  }
  pub fn update_size(&mut self, mgr: GameMgr) {
    self.metadata.update_size(mgr.aspect_ratio());
    
  }
  pub fn create_text_mesh(&mut self, text: &mut GuiTextVals) -> RTextMesh {
    let lines: Vec<RLine> = self.create_structure(text);
    text.num_of_lines = lines.len() as u32;
    let mut x_cursor = 0.0;
    let mut y_cursor = 0.0;
    let mut verts: Vec<f32>  = Vec::new();
    let mut t_coords: Vec<f32> = Vec::new();
    for line in lines {
      if text.is_centered { x_cursor = (line.max_length - line.line_length) / 2.0 }
      for word in &line.words {
        for letter in &word.chars {
          add_verts_for_char(&mut verts, x_cursor, y_cursor, letter, text.font_size);
          push_tex_coords(&mut t_coords, letter.x_tex, letter.y_tex, letter.x_tex_max, letter.y_tex_max);
          x_cursor += letter.x_advance * text.font_size;
        }
        x_cursor += self.metadata.space_width * text.font_size;
      }
      x_cursor = 0.0;
      y_cursor += self.line_ht * text.font_size;
    }
    RTextMesh::new(verts, t_coords)
  }
  fn create_structure(&mut self, text: &GuiTextVals) -> Vec<RLine> {
    // println!("GuiTextVals: {:?}", text);
    let chars = text.text.as_bytes();
    let mut lines: Vec<RLine> = Vec::new();
    let mut current_line = RLine::new(self.metadata.space_width, text.font_size, text.line_max_size);
    let mut current_word = Some(RWord::new(text.font_size));
    for chr in chars {
      let ascii = *chr as u32;
      if ascii == self.space_ascii {
        current_word = current_line.try_add_word(&mut current_word);
        if current_word.is_some() {
          lines.push(current_line);
          current_line = RLine::new(self.metadata.space_width, text.font_size, text.line_max_size);
          current_line.try_add_word(&mut current_word);
          // println!("CurrentLine: {:?}", &current_line);
        }
        current_word = Some(RWord::new(text.font_size));
        continue
      }
      let character = self.metadata.get(ascii);
      // println!("RChar: {:?}", &character);
      if current_word.is_some() { let mut cw = current_word.take().unwrap(); cw.add_char(character); current_word = Some(cw); }
    }
    current_word = current_line.try_add_word(&mut current_word);
    if current_word.is_some() {
      lines.push(current_line);
      current_line = RLine::new(self.metadata.space_width, text.font_size, text.line_max_size);
      current_line.try_add_word(&mut current_word);
    }
    lines.push(current_line);
    return lines
  }
}
fn add_verts_for_char(verts: &mut Vec<f32>, x_curser: f32, y_curser: f32, rchar: &RChar, font_size: f32) {
  let x = x_curser + (rchar.x_offset * font_size);
  let y = y_curser + (rchar.y_offset * font_size);
  let x_max = x + (rchar.x_size * font_size);
  let y_max = y + (rchar.y_size * font_size);
  let x_proper: f32 = (2.0 * x) - 1.0;
  let y_proper: f32 = (-2.0 * y) + 1.0;
  let x_proper_max: f32 = ( 2.0 * x_max) - 1.0;
  let y_proper_max: f32 = (-2.0 * y_max) + 1.0;
  push_verts(verts, x_proper, y_proper, x_proper_max, y_proper_max);
}
fn push_verts(verts: &mut Vec<f32>, x: f32, y: f32, x_max: f32, y_max: f32) {
  let mut tmp = Vec::new();
  tmp.append(verts);
  verts.clear();
  verts.push(x);
  verts.push(y);
  verts.push(x);
  verts.push(y_max);
  verts.push(x_max);
  verts.push(y_max);
  verts.push(x_max);
  verts.push(y_max);
  verts.push(x_max);
  verts.push(y);
  verts.push(x);
  verts.push(y);
  verts.append(&mut tmp);
}
fn push_tex_coords(tex_coords: &mut Vec<f32>, x: f32, y: f32, x_max: f32, y_max: f32) {
  let mut tmp = Vec::new();
  tmp.append(tex_coords);
  tex_coords.clear();
  tex_coords.push(x);
  tex_coords.push(y);
  tex_coords.push(x);
  tex_coords.push(y_max);
  tex_coords.push(x_max);
  tex_coords.push(y_max);
  tex_coords.push(x_max);
  tex_coords.push(y_max);
  tex_coords.push(x_max);
  tex_coords.push(y);
  tex_coords.push(x);
  tex_coords.push(y);
  tex_coords.append(&mut tmp);
}
