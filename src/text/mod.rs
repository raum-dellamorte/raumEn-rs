pub mod guitext;
pub mod metafile;
pub mod rtmc;
pub mod textmgr;

pub use crate::text::textmgr::TextMgr;

use {
  crate::{
    text::{
      // metafile::MetaFile,
      guitext::GuiTextVals,
      rtmc::RTextMeshCreator,
    },
    util::{
      rgl::*,
    },
  },
};

pub const SPACE_ASCII: u32 = 32;
pub const NEWLINE_ASCII: u32 = 10;
pub const LINE_HEIGHT: f32 = 0.03;

#[derive(Debug)]
pub struct RFontType {
  pub tex_atlas: String,
  pub rtmc: RTextMeshCreator,
}
impl RFontType {
  pub fn new(aspect_ratio: f32, font: &str) -> Self {
    Self {
      tex_atlas: font.to_owned(),
      rtmc: RTextMeshCreator::new(aspect_ratio, font),
    }
  }
  pub fn load_text(&mut self, text: &mut GuiTextVals) -> RTextMesh {
    self.rtmc.create_text_mesh(text)
  }
  pub fn update_size(&mut self) {
    self.rtmc.update_size();
  }
}

#[derive(Debug)]
pub struct RTextMesh {
  pub verts: Vec<f32>,
  pub tex_coords: Vec<f32>,
  pub vert_count: VertexCount,
}
impl RTextMesh {
  pub fn new(verts: Vec<f32>, tex_coords: Vec<f32>) -> Self {
    let vert_count = VertexCount((verts.len() / 2) as i32);
    Self {
      verts,
      tex_coords,
      vert_count,
    }
  }
}

#[derive(Debug)]
pub struct RLine {
  pub words: Vec<RWord>,
  pub line_length: f32,
  pub max_length: f32,
  pub space_size: f32,
}
impl RLine {
  pub fn new(space_width: f32, font_size: f32, max_length: f32) -> Self {
    Self {
      words: Vec::new(),
      line_length: 0.0,
      max_length,
      space_size: space_width * font_size,
    }
  }
  pub fn try_add_word(&mut self, word: &mut Option<RWord>) -> Option<RWord> {
    let word = word.take().unwrap();
    let mut plus_length = word.width;
    if !self.words.is_empty() { plus_length += self.space_size; }
    // println!("size: {} trying to add word: {:?}, ", plus_length, word);
    if self.line_length + plus_length <= self.max_length {
      self.words.push(word);
      self.line_length += plus_length;
      None
    } else {
      Some(word)
    }
  }
}

#[derive(Debug)]
pub struct RWord {
  pub font_size: f32,
  pub chars: Vec<RChar>,
  pub width: f32,
}
impl RWord {
  pub fn new(size: f32) -> Self {
    Self {
      font_size: size,
      chars: Vec::new(),
      width: 0.0,
    }
  }
  pub fn add_char(&mut self, char: Option<&RChar>) {
    if let Some(character) = char {
      self.width += character.x_advance * self.font_size;
      self.chars.push((*character).clone());
    }
  }
}

#[derive(Clone, Debug)]
pub struct RChar {
  pub id: u32,
  pub x_tex: f32, pub y_tex: f32,
  pub x_tex_max: f32, pub y_tex_max: f32,
  pub x_offset: f32, pub y_offset: f32,
  pub x_size: f32, pub y_size: f32,
  pub x_advance: f32,
}
impl RChar {
  pub fn new(
    id: u32,
    x_tex: f32, y_tex: f32,
    x_tex_size: f32, y_tex_size: f32,
    x_offset: f32, y_offset: f32,
    x_size: f32, y_size: f32,
    x_advance: f32,
  ) -> Self {
    Self {
      id,
      x_tex, y_tex,
      x_tex_max: x_tex_size + x_tex, y_tex_max: y_tex_size + y_tex,
      x_offset, y_offset, x_size, y_size, x_advance,
    }
  }
}