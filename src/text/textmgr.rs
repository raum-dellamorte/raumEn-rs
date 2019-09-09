
use {
  specs::{World, WorldExt, },
  Display,
  Loader,
  Textures,
  text::{
    guitext::GuiText,
    RFontType,
  },
};
use util::{Vector2f, HashMap, HashSet};

pub struct TextMgr {
  pub active_text: HashMap<String, HashSet<String>>,
  pub texts: HashMap<String, GuiText>,
  pub fonts: HashMap<String, RFontType>,
  
}
impl Default for TextMgr {
  fn default() -> Self {
    Self {
      active_text: HashMap::new(),
      texts: HashMap::new(),
      fonts: HashMap::new(),
    }
  }
}
impl TextMgr {
  pub fn add_font(&mut self, world: &World, fname: &str) {
    let mut loader = world.write_resource::<Loader>();
    let mut textures = world.write_resource::<Textures>();
    // println!("Adding Font: {}", fname);
    let display = world.read_resource::<Display>();
    self.fonts.insert(fname.to_owned(), RFontType::new(display.aspect_ratio, fname));
    // println!("Adding Font Texture: {}", fname);
    textures.load_texture(&mut loader, fname);
  }
  pub fn add_fonts(&mut self, world: &World, fnames: &[String]) {
    for fname in fnames { self.add_font(world, fname); }
  }
  pub fn new_text(&mut self, world: &World, label: &str, text: &str, font_name: &str,
              font_size: f32, x: f32, y: f32,
              line_max_size: f32, is_centered: bool, enable: bool)
  {
    // println!("Adding text {}", label);
    let gt = GuiText::new(font_name, label, text, Vector2f::new(x, y), font_size, line_max_size, is_centered);
    self.texts.insert(label.to_owned(), gt);
    if enable { self.enable_label(world, label); }
  }
  pub fn enable_label(&mut self, world: &World, label: &str) {
    // println!("Enabling text {}", label);
    let mut font = "".to_owned();
    let mut text = self.texts.remove(label);
    if let Some(ref mut text) = text {
      text.load(self, world);
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
  }
  pub fn disable_label(&mut self, label: &str) {
    let mut rm = false;
    let mut font = "".to_owned();
    if let Some(text) = self.texts.get_mut(label) {
      let mut text_batch = self.active_text.get_mut(&text.font);
      if text_batch.is_some() {
        let hs = &mut text_batch.as_mut().unwrap();
        hs.remove(label);
        rm = hs.is_empty();
        font = text.font.clone();
      }
    };
    if rm { self.active_text.remove(&font); }
  }
  #[allow(dead_code)]
  pub fn update_text(&mut self, world: &World, label: &str, new_text: &str) {
    let mut text = self.texts.remove(label);
    if let Some(ref mut text) = text {
      text.update_text(self, world, new_text);
    }
    if text.is_some() {
      self.texts.insert(label.to_owned(), text.unwrap());
    }
  }
  pub fn update_size(&mut self, world: &World) {
    let mut fonts = Vec::new();
    for font in self.fonts.keys() {
      fonts.push(font.to_owned());
    }
    for font in &fonts {
      let mut fnt = self.fonts.remove(font);
      if let Some(ref mut fnt) = fnt {
        fnt.update_size(world);
      }
      if fnt.is_some() {
        self.fonts.insert(font.to_owned(), fnt.unwrap());
      }
    }
    let mut labels = Vec::new();
    for label in self.texts.keys() {
      labels.push(label.to_owned());
    }
    for label in &labels {
      let mut text = self.texts.remove(label);
      if let Some(ref mut text) = text {
        text.update_size(self, world);
      }
      if text.is_some() {
        self.texts.insert(label.to_owned(), text.unwrap());
      }
    }
  }
}