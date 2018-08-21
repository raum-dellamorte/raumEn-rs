
use time::SteadyTime;

pub struct Timer {
  pub delta: f32,
  pub fps: f32,
  pub last: SteadyTime,
  pub now: SteadyTime,
}

impl Timer {
  pub fn new() -> Self {
    let tmp = SteadyTime::now();
    Timer {
      delta: 0.0667_f32,
      fps: 0_f32,
      last: tmp,
      now: SteadyTime::now(),
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
