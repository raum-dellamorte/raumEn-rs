
// use specs::{Component, DenseVecStorage};
use time::{Duration, SteadyTime};

#[derive(Debug)]
pub struct Timer {
  pub delta: f32,
  pub fps: f32,
  pub last: SteadyTime,
  pub now: SteadyTime,
}

impl Default for Timer {
  fn default() -> Self {
    let tmp = SteadyTime::now();
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
    let tmp = SteadyTime::now();
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
    self.now = SteadyTime::now();
    let dur = self.now - self.last;
    self.delta = match dur.num_microseconds() {
      Some(t) => (t as f32) / 1000000_f32,
      None => 0_f32,
    };
    self.fps = match self.delta > 0_f32 {
      true => 1_f32 / self.delta,
      false => 0_f32,
    };
    self
  }
}
