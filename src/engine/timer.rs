
// use specs::{Component, DenseVecStorage};
use time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
  pub delta: f32,
  pub fps: f32,
  pub last: Instant,
  pub now: Instant,
}

impl Default for Timer {
  fn default() -> Self {
    let tmp = Instant::now();
    let tmp2 = tmp + Duration::milliseconds(50_i64);
    Self {
      delta: 0.0667_f32,
      fps: 60.0,
      last: tmp,
      now: tmp2,
    }
  }
}

impl Timer {
  pub fn new() -> Self {
    let tmp = Instant::now();
    let tmp2 = tmp + Duration::milliseconds(50_i64);
    Timer {
      delta: 0.0667_f32,
      fps: 60_f32,
      last: tmp,
      now: tmp2,
    }
  }
  
  pub fn tick(&mut self) -> &Self {
    self.last = self.now;
    self.now = Instant::now();
    let dur = self.now - self.last;
    self.delta = (dur.whole_microseconds() as f32) / 1_000_000_f32;
    self.fps = if self.delta > 0_f32 { 1_f32 / self.delta } else { 0_f32 };
    self
  }
}
