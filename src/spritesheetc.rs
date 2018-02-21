extern crate serde_json;

use std::io::{stdin, stdout, Read, Write};
use std::fs::File;
use std::error;

use super::sprites::sheet::SheetPatternTable;
use super::sprites::PatternTable;

pub struct Config {
    pub input: Option<String>,
    pub chr: Option<String>,
    pub header: Option<String>,
    pub asm: Option<String>,
    pub prefix: String,
}

pub fn run(config: Config) -> Result<(), Box<error::Error>> {
    let input: Box<Read> = match config.input {
        Some(filename) => Box::new(File::open(filename)?),
        None => Box::new(stdin()),
    };

    let mut chr: Box<Write> = match config.chr {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(stdout()),
    };

    let header: Option<(String, File)> = match config.header {
        Some(filename) => Some((filename.clone(), File::create(filename)?)),
        None => None,
    };

    let asm: Option<(String, File)> = match config.asm {
        Some(filename) => Some((filename.clone(), File::create(filename)?)),
        None => None,
    };

    let prefix = config.prefix;

    let sheet_pattern_table: SheetPatternTable = serde_json::from_reader(input)?;
    let pattern_table = PatternTable::from_sheet_pattern_table(sheet_pattern_table)?;
    pattern_table.write(&mut chr)?;

    if let Some((filename, mut file)) = header {
        let guard_string: String = filename.replace(".", "_").replace("/", "_").replace("\\", "_").to_uppercase();
        let guard_string_fixed: String = guard_string.trim_matches('_').to_string();
        writeln!(file, "#ifndef {}", guard_string_fixed)?;
        writeln!(file, "#define {}", guard_string_fixed)?;
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
        writeln!(file, "#endif /* {} */", guard_string_fixed)?;
        file.sync_all()?;
    }

    if let Some((filename, mut file)) = asm {
        let guard_string: String = filename.replace(".", "_").replace("/", "_").replace("\\", "_").to_uppercase();
        let guard_string_fixed: String = guard_string.trim_matches('_').to_string();
        writeln!(file, ".ifndef {}", guard_string_fixed)?;
        writeln!(file, "{} = 1", guard_string_fixed)?;
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
        writeln!(file, ".endif ; {}", guard_string_fixed)?;
        file.sync_all()?;
    }
    Ok(())
}
