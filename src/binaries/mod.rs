//! The set of binary program entry points.
//!
//! As with most Rust binaries, this one abstracts most of its functionality into the library, and
//! only uses the actual binary entry point to parse the command line and print errors.  Each
//! module within this module describes a full binary program and all of its functionality. The
//! list of submodules here is a good summary of the binaries shipped by this package.

pub mod spritesheetc;
pub mod stagec;

use std::error;
use std::fmt;
use std::convert::From;
use std::io;

/// Simple centralized error type for easier handling.
#[derive(Debug)]
pub struct Error {
    description: String,
}

impl Error {
    /// Allow converting another error type into this error type
    pub fn new<T: error::Error>(description: &str, error: T) -> Error {
        Error {
            description: format!("{}: {}", description, error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new("IO Error", error)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
