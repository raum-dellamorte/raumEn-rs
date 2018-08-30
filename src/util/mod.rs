pub mod maths;
pub mod rvector;
pub mod rmatrix;
pub mod rvertex;

pub use util::rmatrix::Matrix4f;
pub use util::rvector::{Vector2f, Vector3f};

// #[derive(Debug, Copy, Clone)]
// pub struct TransMat {
//   pub transform: [f32; 16],
// }

// impl TransMat {
//   pub fn new() -> Self {
//     TransMat {
//       transform: [1.0, 0.0, 0.0, 0.0,
//                   0.0, 1.0, 0.0, 0.0,
//                   0.0, 0.0, 1.0, 0.0,
//                   0.0, 0.0, 0.0, 1.0_f32]
//     }
//   }
//   pub fn len(&self) -> usize {
//     self.transform.len()
//   }
// }
