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

use std::io::{stdin, Read};
use std::fs::File;

use super::Error;

use crate::stage::serialize;

/// Config type, built from command line or however you'd like.
pub struct Config {
    pub input: Option<String>,
    pub stage: Option<String>,
}

/// Entry point for actual running.  Propagates all errors upward.
pub fn run(config: Config) -> Result<(), Error> {
    let input: Box<Read> = match config.input {
        Some(filename) => match File::open(filename) {
            Ok(file) => Box::new(file),
            Err(err) => return Err(Error::new("Error opening input YAML file", err)),
        },
        None => Box::new(stdin()),
    };

    let stage: serialize::Stage = match serde_yaml::from_reader(input) {
        Ok(stage) => stage,
        Err(err) => return Err(Error::new("Error loading YAML", err)),
    };

    stage.write_binary(&mut std::io::stdout())?;

    Ok(())
}
