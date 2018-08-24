pub mod guitext;
pub mod metafile;
pub mod rtmc;

use gamemgr::GameMgr;
use text::guitext::GuiTextVals;
// use text::metafile::MetaFile;
use text::rtmc::RTextMeshCreator;

const SPACE_ASCII: u32 = 32;
const LINE_HEIGHT: f32 = 0.03;

pub struct RFontType {
  pub tex_atlas: String,
  pub loader: RTextMeshCreator,
}
impl RFontType {
  pub fn new(mgr: GameMgr, font: &str) -> Self {
    Self {
      tex_atlas: String::new(),
      loader: RTextMeshCreator::new(mgr, &format!("res/fonts/{}.fnt", font)),
    }
  }
  pub fn load_text(&mut self, text: &mut GuiTextVals) -> RTextMesh {
    self.loader.create_text_mesh(text)
  }
}

pub struct RTextMesh {
  pub verts: Vec<f32>,
  pub tex_coords: Vec<f32>,
  pub vert_count: u32,
}
impl RTextMesh {
  pub fn new(verts: Vec<f32>, tex_coords: Vec<f32>) -> Self {
    let count = verts.len();
    Self {
      verts: verts,
      tex_coords: tex_coords,
      vert_count: count as u32,
    }
  }
}

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
      max_length: max_length,
      space_size: space_width * font_size,
    }
  }
  pub fn try_add_word(&mut self, word: &mut Option<RWord>) -> Option<RWord> {
    let word = word.take().unwrap();
    let mut plus_length = (&word).width;
    if !self.words.is_empty() { plus_length += self.space_size; }
    if self.line_length + plus_length <= self.max_length {
      self.words.push(word);
      self.line_length += plus_length;
      None
    } else {
      Some(word)
    }
  }
}

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
    if !char.is_none() {
      let char = char.unwrap();
      self.width += char.x_advance * self.font_size;
      self.chars.push((*char).clone());
    }
  }
}

#[derive(Clone)]
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
      id: id,
      x_tex: x_tex, y_tex: y_tex,
      x_tex_max: x_tex_size + x_tex, y_tex_max: y_tex_size + y_tex,
      x_offset: x_offset, y_offset: y_offset,
      x_size: x_size, y_size: y_size,
      x_advance: x_advance,
    }
  }
}