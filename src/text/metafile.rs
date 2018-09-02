
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;
use std::str::FromStr;

use nom::{space, digit, float_s, }; // alpha, alphanumeric, 

use gamemgr::GameMgr;
use text::{RChar, }; // RLine, RWord, RFontType, SPACE_ASCII, LINE_HEIGHT, 
use text::{LINE_HEIGHT, SPACE_ASCII};
use eof;

pub fn test_noms() {
  test_get_info();
  test_get_common();
  test_get_page();
  test_get_char_count();
  test_get_char();
}

named!(u32_digit<&str, u32 >,
    map_res!( digit, FromStr::from_str )
);
named!(i32_digit<&str, i32 >,
    map_res!( digit, FromStr::from_str )
);
// info face="Times New Roman" size=59 bold=0 italic=0 charset="" unicode=0 stretchH=100 smooth=1 aa=1 padding=8,8,8,8 spacing=0,0
#[derive(Debug)]
pub struct InfoVars {
  face: String,
  size: u32,
  bold: u32,
  italic: u32,
  charset: String,
  unicode: u32,
  stretch_h: u32,
  smooth: u32,
  aa: u32,
  padding: Vec<u32>,
  spacing: Vec<u32>,
}
fn get_info(tstr: &str) -> ( InfoVars ) {
  let eofs = eof(tstr);
  match _get_info(&eofs) {
    Ok((_, result)) => { result }
    Err(e) => panic!("{}", e)
  }
}
named!(_get_info<&str, ( InfoVars ) >,
  do_parse!(
    tag!("info") >> space >>
    tag!("face=\"") >> face: take_until!("\"") >> char!('"') >> space >>
    tag!("size=") >> size: u32_digit >> space >>
    tag!("bold=") >> bold: u32_digit >> space >>
    tag!("italic=") >> italic: u32_digit >> space >>
    tag!("charset=\"") >> charset: take_until!("\"") >> char!('"') >> space >>
    tag!("unicode=") >> unicode: u32_digit >> space >>
    tag!("stretchH=") >> stretch_h: u32_digit >> space >>
    tag!("smooth=") >> smooth: u32_digit >> space >>
    tag!("aa=") >> aa: u32_digit >> space >>
    tag!("padding=") >> p1: u32_digit >> char!(',') >> p2: u32_digit >> 
    char!(',') >> p3: u32_digit >> char!(',') >> p4: u32_digit >> space >>
    tag!("spacing=") >> s1: u32_digit >> char!(',') >> s2: u32_digit >>
    ( InfoVars { face: face.to_string(), size: size, bold: bold, italic: italic, charset: charset.to_string(), 
      unicode: unicode, stretch_h: stretch_h, smooth: smooth, aa: aa, 
      padding: vec![p1, p2, p3, p4], spacing: vec![s1, s2] } )
  )
);
pub fn test_get_info() {
  let tstr = "info face=\"Times New Roman\" size=59 bold=0 italic=0 charset=\"\" unicode=0 stretchH=100 smooth=1 aa=1 padding=8,8,8,8 spacing=0,0";
  let test = get_info(tstr);
  println!("{:?}", test);
}
// common lineHeight=84 base=54 scaleW=512 scaleH=512 pages=1 packed=0
#[derive(Debug)]
pub struct CommonVars {
  pub line_height: u32,
  pub base: u32,
  pub scale_w: u32,
  pub scale_h: u32,
  pub pages: u32,
  pub packed: u32,
}
fn get_common(tstr: &str) -> CommonVars {
  let eofs = eof(tstr);
  match _get_common(&eofs) {
    Ok((_, result)) => { result }
    Err(e) => panic!("{}", e)
  }
}
named!(_get_common<&str, ( CommonVars ) >,
  do_parse!(
    tag!("common") >> space >>
    tag!("lineHeight=") >> line_height: u32_digit >> space >>
    tag!("base=") >> base: u32_digit >> space >>
    tag!("scaleW=") >> scale_w: u32_digit >> space >>
    tag!("scaleH=") >> scale_h: u32_digit >> space >>
    tag!("pages=") >> pages: u32_digit >> space >>
    tag!("packed=") >> packed: u32_digit >>
    ( CommonVars { line_height: line_height, base: base, scale_w: scale_w, scale_h: scale_h, pages: pages, packed: packed } )
  )
);
pub fn test_get_common() {
  let tstr = "common lineHeight=84 base=54 scaleW=512 scaleH=512 pages=1 packed=0";
  let test = get_common(tstr);
  println!("{:?}", test);
}
// page id=0 file="TimesNewRoman.png"
#[derive(Debug)]
pub struct PageVars {
  id: u32,
  file: String,
}
fn get_page(tstr: &str) -> PageVars {
  let eofs = eof(tstr);
  match _get_page(&eofs) {
    Ok((_, result)) => { result }
    Err(e) => panic!("{}", e)
  }
}
named!(_get_page<&str, ( PageVars ) >,
  do_parse!(
    tag!("page") >> space >>
    tag!("id=") >> id: u32_digit >> space >>
    tag!("file=\"") >> file: take_until!("\"") >> char!('"') >> 
    ( PageVars { id: id, file: file.to_string() } )
  )
);
pub fn test_get_page() {
  let tstr = &eof("page id=0 file=\"TimesNewRoman.png\"");
  let test = get_page(tstr);
  println!("{:?}", test);
}
// chars count=95
fn get_char_count(tstr: &str) -> ( u32 ) {
  let eofs = eof(tstr);
  match _get_char_count(&eofs) {
    Ok((_, result)) => { result }
    Err(e) => panic!("{}", e)
  }
}
named!(_get_char_count<&str, ( u32 ) >,
  do_parse!(
    tag!("chars") >> space >> tag!("count=") >> cnt: u32_digit >> ( cnt )
  )
);
pub fn test_get_char_count() {
  let tstr = &eof("chars count=95");
  let test = get_char_count(tstr);
  println!("{:?}", test);
}
// char id=32   x=0     y=0     width=0     height=0     xoffset=-5     yoffset=54    xadvance=31     page=0  chnl=0
#[derive(Debug)]
pub struct CharVars {
  pub id: u32,
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub xoffset: f32,
  pub yoffset: f32,
  pub xadvance: i32,
  pub page: u32,
  pub chnl: u32,
}
fn get_char(tstr: &str) -> CharVars {
  let eofs = eof(tstr);
  match _get_char(&eofs) {
    Ok((_, result)) => { result }
    Err(e) => panic!("{}", e)
  }
}
named!(_get_char<&str, ( CharVars ) >,
  do_parse!(
    tag!("char") >> space >>
    tag!("id=") >> id: u32_digit >> space >>
    tag!("x=") >> x: i32_digit >> space >>
    tag!("y=") >> y: i32_digit >> space >>
    tag!("width=") >> width: i32_digit >> space >>
    tag!("height=") >> height: i32_digit >> space >>
    tag!("xoffset=") >> xoffset: float_s >> space >>
    tag!("yoffset=") >> yoffset: float_s >> space >>
    tag!("xadvance=") >> xadvance: i32_digit >> space >>
    tag!("page=") >> page: u32_digit >> space >>
    tag!("chnl=") >> chnl: u32_digit >> space >>
    ( CharVars { id: id, x: x, y: y, width: width, height: height, 
        xoffset: xoffset, yoffset: yoffset, xadvance: xadvance, page: page, chnl: chnl } )
  )
);
pub fn test_get_char() {
  let tstr = "char id=32   x=0     y=0     width=0     height=0     xoffset=-5     yoffset=54    xadvance=31     page=0  chnl=0 ";
  let test = get_char(tstr);
  println!("{:?}", test);
}

const PAD_TOP: usize = 0;
const PAD_LEFT: usize = 1;
const PAD_BOTTOM: usize = 2;
const PAD_RIGHT: usize = 3;
const DESIRED_PADDING: i32 = 8;

#[derive(Debug)]
pub struct MetaFile {
  aspect_ratio: f32,
  vertical_per_pixel_size: f32,
  horizontal_per_pixel_size: f32,
  pub space_width: f32,
  padding: Vec<u32>,
  padding_width: u32,
  padding_height: u32,
  metadata: HashMap<u32, RChar>,
  info: Option<InfoVars>,
  common: Option<CommonVars>,
  page: Option<PageVars>,
  count: u32,
  chars: Vec<CharVars>,
}
impl MetaFile {
  pub fn new(mgr: GameMgr, font_file: &str) -> Self {
    let aspect_ratio = {
      let cam = mgr.camera.clone();
      let cam = cam.lock().unwrap();
      let (w, h) = cam.dimensions;
      // println!("w: {:?} h: {:?}", w, h);
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
      info: None,
      common: None,
      page: None,
      count: 0,
      chars: Vec::new(),
    };
    let filename = format!("res/fonts/{}.fnt", font_file);
    let path = Path::new(&filename);
    let display = path.display();
    let file = match File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}", display, why.description()),
      Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
      match &(line.unwrap()) {
        l if &l[..5] == "info " => { out.info = Some(get_info(l)); }
        l if &l[..7] == "common " => { out.common = Some(get_common(l)); }
        l if &l[..5] == "page " => { out.page = Some(get_page(l)); }
        l if &l[..6] == "chars " => { out.count = get_char_count(l); }
        l if &l[..5] == "char " => { out.chars.push(get_char(l)); }
        _ => ()
      }
    }
    out.load_padding_data();
    let image_width = out.load_line_sizes();
    out.load_char_data(image_width);
    out
  }
  pub fn get(&self, ascii: u32) -> Option<&RChar> {
    self.metadata.get(&ascii)
  }
  fn load_padding_data(&mut self) {
    let info = self.info.take().unwrap();
    self.padding_width = (info.padding[PAD_LEFT] + info.padding[PAD_RIGHT]) as u32;
    self.padding_height = (info.padding[PAD_TOP] + info.padding[PAD_BOTTOM]) as u32;
    for n in &info.padding {
      self.padding.push(n.to_owned());
    }
    // println!("padding: {:?}", self.padding);
    self.info = Some(info);
  }
  fn load_line_sizes(&mut self) -> f32 {
    let common = self.common.take().unwrap();
    let line_height_pixels = common.line_height as i32 - self.padding_height as i32;
    self.vertical_per_pixel_size = LINE_HEIGHT / line_height_pixels as f32;
    self.horizontal_per_pixel_size = self.vertical_per_pixel_size / self.aspect_ratio;
    // println!("self.vertical_per_pixel_size: {:?}", self.vertical_per_pixel_size);
    // println!("self.horizontal_per_pixel_size: {:?}", self.horizontal_per_pixel_size);
    // println!("self.aspect_ratio: {:?}", self.aspect_ratio);
    let image_width = common.scale_w;
    self.common = Some(common);
    image_width as f32
  }
  fn load_char_data(&mut self, image_width: f32) {
    for chr in &self.chars {
      // println!("CharVal: {:?}", chr);
      let id = chr.id;
      if id == SPACE_ASCII {
        self.space_width = (chr.xadvance - self.padding_width as i32) as f32 * self.horizontal_per_pixel_size;
        continue
      }
      let x_tex = (chr.x + (self.padding[PAD_LEFT] as i32 - DESIRED_PADDING)) as f32 / image_width;
      let y_tex = (chr.y + (self.padding[PAD_TOP] as i32 - DESIRED_PADDING)) as f32 / image_width;
      let width = (chr.width - (self.padding_width as i32 - (2_i32 * DESIRED_PADDING))) as f32;
      let height = (chr.height - (self.padding_height as i32 - (2_i32 * DESIRED_PADDING))) as f32;
      let x_size = width * self.horizontal_per_pixel_size as f32;
      let y_size = height * self.vertical_per_pixel_size as f32;
      let x_tex_size = width as f32 / image_width;
      let y_tex_size = height as f32 / image_width;
      let x_offset = (chr.xoffset + (self.padding[PAD_LEFT] as i32 - DESIRED_PADDING) as f32) * self.horizontal_per_pixel_size;
      let y_offset = (chr.yoffset + (self.padding[PAD_TOP] as i32 - DESIRED_PADDING) as f32) * self.vertical_per_pixel_size;
      let x_advance = (chr.xadvance - self.padding_width as i32) as f32 * self.horizontal_per_pixel_size;
      let rchar = RChar::new(id, x_tex, y_tex, x_tex_size, y_tex_size, x_offset, y_offset, x_size, y_size, x_advance);
      self.metadata.insert(id, rchar);
    }
  }
}
