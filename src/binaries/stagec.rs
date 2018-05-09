//! The stage file compiler,  as used by evy.  May be used in a build system to compile ascii stage
//! descriptions (in YAML format) into stage stubs, with an automatically-inserted stub library.
//! 
//! The usage can be retrieved exactly how you'd expect:
//! 
//! ```sh
//! $ spritesheetc -h
//! Usage: spritesheetc [options]
//! 
//! Options:
//!     -i, --input FILE    input yaml description file. Defaults to stdin.
//!     -o, --char FILE     output NES char file name. Defaults to stdout.
//!     -h, --help          print this help menu
//! ```

extern crate serde_yaml;

use std::io::{stdin, Read};
use std::fs::File;
use std::error;
use std::fmt;

/// Config type, built from command line or however you'd like.
pub struct Config {
    pub input: Option<String>,
    pub stage: Option<String>,
}

/// Simple boxing error type for easier handling.
#[derive(Debug)]
struct Error {
    description: String,
}

impl Error {
    pub fn boxed<T: error::Error>(description: &str, error: T) -> Box<Error> {
        Box::new(Error {
            description: format!("{}: {}", description, error),
        })
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

/// Entry point for actual running.  Propagates all errors upward.
pub fn run(config: Config) -> Result<(), Box<error::Error>> {
    let input: Box<Read> = match config.input {
        Some(filename) => match File::open(filename) {
            Ok(file) => Box::new(file),
            Err(err) => return Err(Error::boxed("Error opening input YAML file", err)),
        },
        None => Box::new(stdin()),
    };

    Ok(())
}
