
pub mod coords;
pub mod gen;
pub mod node;
pub mod platform;

pub use terrain::coords::*;
pub use terrain::gen::*;
pub use terrain::platform::*;
pub use terrain::node::*;

#[derive(Debug)]
pub struct PlayerLoc(pub i32,pub i32);
impl Default for PlayerLoc { fn default() -> Self { Self(0,0) } }

