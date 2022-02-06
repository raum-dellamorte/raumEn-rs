
// use {
//   std::something::Debug,
// };

pub struct Pool<T> {
  storage: Vec<T>,
}
impl<T> Pool<T> where T: Default {
  pub fn new(buf_size: isize) -> Self {
    let mut storage = Vec::new();
    for _ in 0..buf_size {
      storage.push(T::default());
    }
    Self { storage, }
  }
  pub fn take(&mut self) -> T {
    if !self.storage.is_empty() {
      // format!("Failed to pop from vec in Pool<{:?}>", T)
      self.storage.pop().expect("Failed to pop from vec in Pool")
    } else {
      T::default()
    }
  }
  pub fn push(&mut self, item: T) {
    self.storage.push(item);
  }
}
