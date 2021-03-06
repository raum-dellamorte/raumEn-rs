
use GameMgr;
use text::guitext::GuiText;
use text::RFontType;
use util::{Vector2f, HashMap, HashSet};

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
  pub fn add_font(&mut self, mgr: Box<GameMgr>, fname: &str) -> Box<GameMgr> {
    let mut mgr = mgr;
    // println!("Adding Font: {}", fname);
    self.fonts.insert(fname.to_owned(), RFontType::new(mgr.aspect_ratio(), fname));
    // println!("Adding Font Texture: {}", fname);
    mgr.new_texture(fname);
    mgr
  }
  pub fn add_fonts(&mut self, mgr: Box<GameMgr>, fnames: &[String]) -> Box<GameMgr> {
    let mut mgr = mgr;
    for fname in fnames { mgr = self.add_font(mgr, fname); }
    mgr
  }
  pub fn new_text(&mut self, mgr: Box<GameMgr>, label: &str, text: &str, font_name: &str,
              font_size: f32, x: f32, y: f32,
              line_max_size: f32, is_centered: bool, enable: bool) -> Box<GameMgr>
  {
    let mut mgr = mgr;
    // println!("Adding text {}", label);
    let gt = GuiText::new(font_name, label, text, Vector2f::new(x, y), font_size, line_max_size, is_centered);
    self.texts.insert(label.to_owned(), gt);
    if enable { mgr = self.enable_label(mgr, label); }
    mgr
  }
  pub fn enable_label(&mut self, mgr: Box<GameMgr>, label: &str) -> Box<GameMgr> {
    let mut mgr = mgr;
    // println!("Enabling text {}", label);
    let mut font = "".to_owned();
    let mut text = self.texts.remove(label);
    if let Some(ref mut text) = text {
      mgr = text.load(self, mgr);
      font = text.font.clone();
    }
    // println!("Text font: {}", font);
    if text.is_some() {
      self.texts.insert(label.to_owned(), text.unwrap());
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
      // println!("Adding text {} to active_text", label);
      self.active_text.insert(font, hs.unwrap());
    }
    mgr
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
  pub fn update_text(&mut self, mgr: Box<GameMgr>, label: &str, new_text: &str) -> Box<GameMgr> {
    let mut mgr = mgr;
    let mut text = self.texts.remove(label);
    if let Some(ref mut text) = text {
      mgr = text.update_text(self, mgr, new_text);
    }
    if text.is_some() {
      self.texts.insert(label.to_owned(), text.unwrap());
    }
    mgr
  }
  pub fn update_size(&mut self, mgr: Box<GameMgr>) -> Box<GameMgr> {
    let mut mgr = mgr;
    let mut fonts = Vec::new();
    for (font, _) in &self.fonts {
      fonts.push(font.to_owned());
    }
    for font in &fonts {
      let mut fnt = self.fonts.remove(font);
      if let Some(ref mut fnt) = fnt {
        mgr = fnt.update_size(mgr);
      }
      if fnt.is_some() {
        self.fonts.insert(font.to_owned(), fnt.unwrap());
      }
    }
    let mut labels = Vec::new();
    for (label, _) in &self.texts {
      labels.push(label.to_owned());
    }
    for label in &labels {
      let mut text = self.texts.remove(label);
      if let Some(ref mut text) = text {
        mgr = text.update_size(self, mgr);
      }
      if text.is_some() {
        self.texts.insert(label.to_owned(), text.unwrap());
      }
    }
    mgr
  }
}