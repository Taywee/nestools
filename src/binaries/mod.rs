//! The set of binary program entry points.
//!
//! As with most Rust binaries, this one abstracts most of its functionality into the library, and
//! only uses the actual binary entry point to parse the command line and print errors.  Each
//! module within this module describes a full binary program and all of its functionality. The
//! list of submodules here is a good summary of the binaries shipped by this package.

pub mod spritesheetc;
pub mod stagec;
