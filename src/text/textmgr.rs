
use std::collections::{HashMap, HashSet};

use gamemgr::GameMgr;
use text::guitext::GuiText;
use text::RFontType;
use util::Vector2f;

pub struct TextMgr {
  pub active_text: HashMap<String, HashSet<String>>,
  pub texts: HashMap<String, GuiText>,
  pub fonts: HashMap<String, RFontType>,
  
}
impl TextMgr {
  pub fn new() -> Self {
    Self {
      active_text: HashMap::new(),
      texts: HashMap::new(),
      fonts: HashMap::new(),
    }
  }
  pub fn add_font(&mut self, mgr: GameMgr, fname: &str) {
    println!("Adding Font: {}", fname);
    self.fonts.insert(fname.to_owned(), RFontType::new(mgr, fname));
  }
  
  pub fn add_fonts(&mut self, mgr: GameMgr, fnames: &[String]) {
    for fname in fnames { self.add_font(mgr.clone(), fname) }
  }
  
  pub fn new_text(&mut self, mgr: GameMgr, label: &str, text: &str, font_name: &str,
              font_size: f32, x: f32, y: f32,
              line_max_size: f32, is_centered: bool, enable: bool)
  {
    let gt = GuiText::new(font_name, label, text, Vector2f::new(x, y), font_size, line_max_size, is_centered);
    self.texts.insert(label.to_owned(), gt);
    if enable { self.enable_label(mgr, label) }
  }
  pub fn enable_label(&mut self, mgr: GameMgr, label: &str) {
    let mut font = "".to_owned();
    if let Some(text) = self.texts.get_mut(label) {
      text.load(mgr);
      font = text.font.clone();
    }
    let mut hs: Option<HashSet<String>> = None;
    if !font.is_empty() {
      let text_batch = self.active_text.get_mut(&font);
      if text_batch.is_none() {
        let mut _hs = HashSet::new();
        _hs.insert(label.to_owned());
        hs = Some(_hs)
      } else {
        let hs = text_batch.unwrap();
        hs.insert(label.to_owned());
        // hs = Some(*_hs)
      }
    }
    if hs.is_some() {
      self.active_text.insert(font, hs.unwrap());
    }
  }
  pub fn disable_label(&mut self, label: &str) {
    let mut rm = false;
    let mut font = "".to_owned();
    if let Some(text) = self.texts.get_mut(label) {
      let mut text_batch = self.active_text.get_mut(&text.font);
      if text_batch.is_some() {
        let mut hs = &mut text_batch.as_mut().unwrap();
        hs.remove(label);
        rm = hs.is_empty();
        font = text.font.clone();
      }
    };
    if rm { self.active_text.remove(&font); }
  }
  #[allow(dead_code)]
  pub fn update(&mut self, mgr: GameMgr, label: String, new_text: &str) {
    if let Some(text) = self.texts.get_mut(&label) {
      text.update(mgr, new_text);
    }
  }
}