//! This is a set of relatively simple tools used to assist with the building of NES games.
//! Currently, its only functionality is in managing sprite sheets.
//!
//! All binaries are individually described in their own binary modules.  [The list of binary
//! modules is namespaced for convenience](binaries/index.html).
//!
//! To download releases, you can either use a standard `cargo install`, or you can visit the
//! [GitHub releases page](https://github.com/Taywee/nestools/releases).  I'll do my best to
//! support a standard set of targets, but I can't make strong guarantees, as I do all of my
//! development on my Linux machine.

#[macro_use]
extern crate serde_derive;
extern crate lodepng;

pub mod sprites;

// Binary entrypoints
pub mod binaries;
