//! Library for dealing with collision detection that uses glam instead of glam or nalgebra.
//!
//! This crate is still a work in progress and has mostly just focuses on 3d use.
//! It is made to be used with the macroquad game framework and is based on collision-rs.
//!
extern crate bit_set;
extern crate glam;
extern crate rand;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate smallvec;

pub use bound::*;
pub use contact::*;
pub use frustum::*;
pub use line::*;
pub use plane::Plane;
pub use ray::*;
pub use traits::*;
pub use volume::*;

pub mod algorithm;
pub mod dbvt;
pub mod prelude;
pub mod primitive;

// Modules

mod bound;
mod contact;
mod frustum;
mod line;
mod plane;
mod ray;
mod traits;
mod volume;
