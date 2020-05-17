
// use specs::{Component, DenseVecStorage};
use time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
  pub delta: f32,
  pub fps: f32,
  pub last: Instant,
  pub now: Instant,
  sec_passed: bool,
  tick_count: u32,
  sec: Duration,
}

impl Default for Timer {
  fn default() -> Self {
    let now = Instant::now();
    let last = now - Duration::milliseconds(50_i64);
    Self {
      delta: 0.0667_f32,
      fps: 60.0,
      last,
      now,
      sec_passed: false,
      tick_count: 0,
      sec: Duration::zero(),
    }
  }
}

impl Timer {
  pub fn new() -> Self {
    Self::default()
  }
  
  pub fn tick(&mut self) -> &Self {
    self.tick_count += 1;
    self.last = self.now.clone();
    self.now = Instant::now();
    let dur = self.now - self.last;
    self.sec += dur;
    self.delta = (dur.whole_microseconds() as f32) / 1_000_000_f32;
    let ms = self.sec.whole_microseconds();
    if ms > 1_000_000 {
      self.sec_passed = true;
      self.sec -= Duration::second();
      // println!("tick count {}", self.tick_count);
      self.fps = self.tick_count as f32 / (ms as f32 / 1_000_000_f32 );
      self.tick_count = 0;
    }
    self
  }
  
  pub fn once_per_sec(&mut self) -> bool {
    if self.sec_passed {
      self.sec_passed = false;
      return true;
    }
    false
  }
}
