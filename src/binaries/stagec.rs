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
//!     -c, --c FILE        output C files basename.  Not generated if not specified.
//!     -a, --asm FILE      output asm file basenames.  Not generated if not specified.
//!     -h, --help          print this help menu
//! ```

extern crate serde_yaml;

use std::io::{self, stdin, stdout, Read, Write};
use std::fs::File;
use std::error;
use std::fmt;

use ::sprites::sheet::SheetPatternTable;
use ::sprites::PatternTable;

/// Config type, built from command line or however you'd like.
pub struct Config {
    pub input: Option<String>,
    pub chr: Option<String>,
    pub header: Option<String>,
    pub asm: Option<String>,
    pub prefix: String,
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

/// Write out the C header file.  This is used as an easy grouping mechanism in order to catch all
/// possible IO errors and report them with a helpful error message.
pub fn write_c_header(filename: &String, prefix: &str, pattern_table: &PatternTable) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;

    let guard_string: String = filename.replace(".", "_").replace("/", "_").replace("\\", "_").to_uppercase();
    let guard_string_fixed: String = guard_string.trim_matches('_').to_string();
    writeln!(file, "#ifndef SPRITESHEETC_{}", guard_string_fixed)?;
    writeln!(file, "#define SPRITESHEETC_{}", guard_string_fixed)?;
    for (index, tile) in pattern_table.left.iter().enumerate() {
        if let Some(ref name) = tile.name {
            writeln!(file, "#define {prefix}LEFT_{name} {index}",
                     prefix = prefix,
                     name = name,
                     index = index)?;
        }
    }
    for (index, tile) in pattern_table.right.iter().enumerate() {
        if let Some(ref name) = tile.name {
            writeln!(file, "#define {prefix}RIGHT_{name} {index}",
                     prefix = prefix,
                     name = name,
                     index = index)?;
        }
    }
    writeln!(file, "#endif /* SPRITESHEETC_{} */", guard_string_fixed)?;

    file.sync_all()
}

/// Write out the ASM header file.  This is used as an easy grouping mechanism in order to catch
/// all possible IO errors and report them with a helpful error message.
pub fn write_asm_header(filename: &String, prefix: &str, pattern_table: &PatternTable) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;

    let guard_string: String = filename.replace(".", "_").replace("/", "_").replace("\\", "_").to_uppercase();
    let guard_string_fixed: String = guard_string.trim_matches('_').to_string();
    writeln!(file, ".ifndef SPRITESHEETC_{}", guard_string_fixed)?;
    writeln!(file, "SPRITESHEETC_{} = 1", guard_string_fixed)?;
    for (index, tile) in pattern_table.left.iter().enumerate() {
        if let Some(ref name) = tile.name {
            writeln!(file, "{prefix}LEFT_{name} = {index}",
                     prefix = prefix,
                     name = name,
                     index = index)?;
        }
    }
    for (index, tile) in pattern_table.right.iter().enumerate() {
        if let Some(ref name) = tile.name {
            writeln!(file, "{prefix}RIGHT_{name} = {index}",
                     prefix = prefix,
                     name = name,
                     index = index)?;
        }
    }
    writeln!(file, ".endif ; SPRITESHEETC_{}", guard_string_fixed)?;

    file.sync_all()
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

    let mut chr: Box<Write> = match config.chr {
        Some(filename) => match File::create(filename) {
            Ok(file) => Box::new(file),
            Err(err) => return Err(Error::boxed("Error opening output CHR file", err)),
        },
        None => Box::new(stdout()),
    };

    let prefix = config.prefix;

    let sheet_pattern_table: SheetPatternTable = match serde_yaml::from_reader(input) {
        Ok(table) => table,
        Err(err) => return Err(Error::boxed("Error loading YAML", err)),
    };

    let pattern_table = match PatternTable::from_sheet_pattern_table(sheet_pattern_table) {
        Ok(table) => table,
        Err(err) => return Err(Error::boxed("Error building pattern table", err)),
    };

    if let Err(err) = pattern_table.write(&mut chr) {
        return Err(Error::boxed("Error writing pattern table", err));
    }

    if let Some(filename) = config.asm {
        if let Err(err) = write_asm_header(&filename, &prefix, &pattern_table) {
            return Err(Error::boxed("Error writing ASM header", err));
        }
    }

    if let Some(filename) = config.header {
        if let Err(err) = write_c_header(&filename, &prefix, &pattern_table) {
            return Err(Error::boxed("Error writing C header", err));
        }
    }

    Ok(())
}
