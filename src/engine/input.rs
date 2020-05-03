
use glutin::event::WindowEvent as WEvent;
use glutin::event::DeviceEvent as DEvent;
use glutin::event::KeyboardInput as KB;
use glutin::event::MouseButton as MB;
use glutin::event::ElementState::{Pressed, Released};
use glutin::event::VirtualKeyCode as VKC;
use glutin::event::ModifiersState as MKS;

use engine::timer::Timer;
use util::HashMap;

pub struct Handler {
  pub timer: Timer,
  pub kb: HashMap<String, bool>,
  pub mouse: HashMap<MB, bool>,
  pub cursor_pos: Option<(f64, f64)>,
  pub cursor_delta: Option<(f64, f64)>,
}
impl Default for Handler {
  fn default() -> Self {
    let mut timer = Timer::new();
    timer.tick();
    Handler { timer, kb: HashMap::<String, bool>::new(), mouse: HashMap::<MB, bool>::new(), cursor_pos: None, cursor_delta: None }
  }
}
impl Handler {
  pub fn fps_and_delta(&self) -> (f32, f32) {
    (self.timer.fps, self.timer.delta)
  }
  pub fn reset_delta(&mut self) {
    self.cursor_delta = None;
  }
  pub fn window_event(&mut self, event: &WEvent) {
    match event {
      WEvent::CursorMoved { position: pos, ..} => {
        self.cursor_pos = Some((pos.x,pos.y));
      }
      WEvent::MouseInput { state: Pressed, button: bttn, ..} => {
        self.mouse.insert(*bttn, true);
      }
      WEvent::MouseInput { state: Released, button: bttn, ..} => {
        self.mouse.insert(*bttn, false);
      }
      WEvent::KeyboardInput { input: KB { virtual_keycode: Some(bttn), state: Pressed, ..}, ..} => { // , modifiers: modkey deprecated
        // print!("{:?}-{}-{}-{}-{}", bttn, modkey.shift, modkey.ctrl, modkey.alt, modkey.logo);
        self.kb.insert(key_code(*bttn), true); // , *modkey
      }
      WEvent::KeyboardInput { input: KB { virtual_keycode: Some(bttn), state: Released, ..}, ..} => { // , modifiers: modkey deprecated
        self.kb.insert(key_code(*bttn), false); // , *modkey
      }
      WEvent::AxisMotion { axis: _axis, value: _val, .. } => {} // DeviceId(X(DeviceId(2)))
      // e => println!("Window Event:\n  {:?}", e)
      _ => ()
    }
  }
  pub fn device_event(&mut self, event: &DEvent) {
    match event {
      DEvent::MouseMotion { delta: dxdy} => {
        self.cursor_delta = Some((dxdy.0, dxdy.1));
      }
      DEvent::Button { button: _bttn, state: Pressed } => {
        // println!("Button pressed: {}", bttn);
      }
      DEvent::Button { button: _bttn, state: Released } => {
        // println!("Button released: {}", bttn);
      }
      // DEvent::ModifiersChanged(_mod_state) => {
        
      // }
      DEvent::Motion {..} => {}
      // e => println!("Device Event:\n{:?}", e)
      _ => ()
    }
  }
  pub fn read_kb_single(&mut self, kc: KeyCode) -> bool {
    match self.kb.insert(key_code(kc.key), false) { // , kc.modkey
      Some(tf) => { tf },
      None     => { false },
    }
  }
  pub fn read_kb_multi(&self, kc: KeyCode) -> bool {
    match self.kb.get(&key_code(kc.key)) { // , kc.modkey
      Some(&tf) => { tf },
      None      => { false },
    }
  }
  pub fn read_kb_single_any_of(&mut self, kcs: KeyCodes) -> bool {
    let mut out = false;
    for kc in kcs.keys {
      if self.read_kb_single(kc) && !out { out = true; }
    }
    out
  }
  pub fn read_kb_multi_any_of(&self, kcs: KeyCodes) -> bool {
    for kc in kcs.keys {
      if self.read_kb_multi(kc) { return true }
    }
    false
  }
  pub fn clear_kb_bttns(&mut self) {
    self.kb.clear();
  }
  pub fn read_mouse_single(&mut self, key: MB) -> bool {
    match self.mouse.insert(key, false) {
      Some(tf) => { tf },
      None     => { false },
    }
  }
  pub fn read_mouse_multi(&self, key: MB) -> bool {
    match self.mouse.get(&key) {
      Some(&tf) => { tf },
      None      => { false },
    }
  }
  pub fn clear_mouse_bttns(&mut self) {
    self.mouse.clear();
  }
}
pub struct KeyCode {
  pub key: VKC,
  pub modkey: MKS,
}
impl KeyCode {
  pub fn new(key: VKC) -> Self {
    KeyCode {
      key,
      modkey: MKS::empty(),
    }
  }
  pub fn to_str(&self) -> String {
    key_code(self.key) // , self.modkey
  }
  pub fn shift(&mut self) -> &mut Self {
    self.modkey.set(MKS::SHIFT, true);
    self
  }
  pub fn ctrl(&mut self) -> &mut Self {
    self.modkey.set(MKS::CTRL, true);
    self
  }
  pub fn alt(&mut self) -> &mut Self {
    self.modkey.set(MKS::ALT, true);
    self
  }
  pub fn logo(&mut self) -> &mut Self {
    self.modkey.set(MKS::LOGO, true);
    self
  }
}
pub struct KeyCodes {
  keys: Vec<KeyCode>,
}
impl KeyCodes {
  pub fn new(keys: &[VKC]) -> Self {
    let mut out = KeyCodes { keys: Vec::new() };
    for key in keys {
      out.keys.push(KeyCode::new(*key));
    }
    out
  }
  pub fn to_str_vec(&self) -> Vec<String> {
    let mut out = Vec::new();
    for key in &self.keys {
      out.push(key_code(key.key)); // , key.modkey
    }
    out
  }
  pub fn shift(&mut self, i: usize) -> &mut Self {
    if i < self.keys.len() {
      self.keys[i].shift();
    }
    self
  }
  pub fn shift_all(&mut self) -> &mut Self {
    for key in &mut self.keys {
      key.shift();
    }
    self
  }
  pub fn ctrl(&mut self, i: usize) -> &mut Self {
    if i < self.keys.len() {
      self.keys[i].ctrl();
    }
    self
  }
  pub fn ctrl_all(&mut self) -> &mut Self {
    for key in &mut self.keys {
      key.ctrl();
    }
    self
  }
  pub fn alt(&mut self, i: usize) -> &mut Self {
    if i < self.keys.len() {
      self.keys[i].alt();
    }
    self
  }
  pub fn alt_all(&mut self) -> &mut Self {
    for key in &mut self.keys {
      key.alt();
    }
    self
  }
  pub fn logo(&mut self, i: usize) -> &mut Self {
    if i < self.keys.len() {
      self.keys[i].logo();
    }
    self
  }
  pub fn logo_all(&mut self) -> &mut Self {
    for key in &mut self.keys {
      key.logo();
    }
    self
  }
}
pub fn key_code(bttn: VKC) -> String { // , modkey: MKS
  format!("{:?}", bttn) // -{}-{}-{}-{} , modkey.shift(), modkey.ctrl(), modkey.alt(), modkey.logo())
}
