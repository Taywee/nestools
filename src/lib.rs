//! This is a set of relatively simple tools used to assist with the building of NES games.
//! Currently, its only functionality is in managing sprite sheets.
//!
//! All binaries are individually described in their own binary modules.  [The list of binary
//! modules is namespaced for convenience](binaries/index.html).

#[macro_use]
extern crate serde_derive;
extern crate lodepng;

pub mod sprites;

// Binary entrypoints
pub mod binaries;
