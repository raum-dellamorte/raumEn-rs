pub mod maths;
pub mod rgl;
pub mod rotator;
pub mod rvector;
pub mod rmatrix;
pub mod rvertex;
pub mod specs;

pub use {
  util::{
    maths::*, 
    // rgl::*, 
    rmatrix::Matrix4f, 
    rotator::{
      Rotator, Rotators,
    },
    rvector::{
      RVec, Vector2f, Vector3f, Quaternion, 
    },
  },
  num::{ Float, NumCast, Zero, },
  std::{ // Attach favourite standard library stuff here! #pub_use_abuse
    rc::Rc,
    cell::RefCell,
    sync::{ Arc, Mutex, },
    collections::{ HashMap, HashSet, },
  },
};
