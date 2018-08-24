
use std::collections::HashMap;

use gamemgr::GameMgr;
use text::{RChar, }; // RLine, RWord, RFontType, SPACE_ASCII, LINE_HEIGHT, 

pub struct MetaFile {
  aspect_ratio: f32,
  vertical_per_pixel_size: f32,
  horizontal_per_pixel_size: f32,
  pub space_width: f32,
  padding: Vec<u32>,
  padding_width: u32,
  padding_height: u32,
  metadata: HashMap<u32, RChar>,
  // reader: BufferedReader,
  values: HashMap<String, String>,
}
impl MetaFile {
  pub fn new(mgr: GameMgr, file: &str) -> Self {
    let aspect_ratio = {
      let cam = mgr.camera.clone();
      let cam = cam.lock().unwrap();
      let (w, h) = cam.dimensions;
      w as f32 / h as f32
    };
    let mut out = Self {
      aspect_ratio: aspect_ratio,
      vertical_per_pixel_size: 0.0,
      horizontal_per_pixel_size: 0.0,
      space_width: 0.03,
      padding: Vec::new(),
      padding_width: 0,
      padding_height: 0,
      metadata: HashMap::new(),
      // reader: BufferedReader
      values: HashMap::new(),
    };
    out.open_file(file);
    out.load_padding_data();
    out.load_line_sizes();
    let image_width = out.get_var_value("scaleW");
    out.load_char_data(image_width);
    out.close();
    out
  }
  pub fn get(&self, ascii: u32) -> Option<&RChar> {
    self.metadata.get(&ascii)
  }
  fn open_file(&mut self, file: &str) {
    
  }
  fn load_padding_data(&mut self) {
    
  }
  fn load_line_sizes(&mut self) {
    
  }
  fn get_var_value(&mut self, var: &str) -> f32 {
    
    0.0
  }
  fn load_char_data(&mut self, width: f32) {
    
  }
  fn close(&mut self) {
    
  }
  // fn process_next_line(&mut self) -> bool {
  //   values.clear()
  //   var line:String? = null
  //   try {
  //     line = reader.readLine()
  //   } catch (e1:IOException) {}
  //   if (line == null) { return false }
  //   for (part in line.split((SPLITTER).toRegex()).dropLastWhile({ it.isEmpty() }).toTypedArray()) {
  //     val valuePairs = part.split(("=").toRegex()).dropLastWhile({ it.isEmpty() }).toTypedArray()
  //     if (valuePairs.size == 2) {
  //       values.put(valuePairs[0], valuePairs[1])
  //     }
  //   }
  //   return true
  // }
}
