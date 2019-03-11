pub mod maths;
pub mod rgl;
pub mod rvector;
pub mod rmatrix;
pub mod rvertex;

pub use util::maths::*;
// pub use util::rgl::*;
pub use util::rmatrix::Matrix4f;
pub use util::rvector::{RVec, Vector2f, Vector3f, Vector4f, XVEC, YVEC, ZVEC};

// Attach favourite standard library stuff here! #pub_use_abuse
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use std::sync::{Arc, Mutex};
pub use std::collections::{HashMap, HashSet};

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
