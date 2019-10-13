pub mod maths;
pub mod rgl;
pub mod rvector;
pub mod rmatrix;
pub mod rvertex;

pub use {
  util::{
    maths::*, 
    // rgl::*, 
    rmatrix::Matrix4f, 
    rvector::{
      RVec, Vector2f, Vector3f, Quaternion, 
      XVEC, YVEC, ZVEC, XVEC64, YVEC64, ZVEC64, 
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
