//! Compiles spritesheets, using a meta file, into sets of headers and a chr file.
//!
//! The usage can be retrieved exactly how you'd expect:
//!
//! ```sh
//! $ spritesheetc -h
//! Usage: spritesheetc [options]
//! 
//! Options:
//!     -i, --input INPUT   input json description file. Defaults to stdin.
//!     -o, --char INPUT    output NES char file name. Defaults to stdout.
//!     -c, --header INPUT  output C header file name
//!     -a, --asm INPUT     output asm define file name
//!     -h, --help          print this help menu
//! ```

extern crate nestools;
extern crate getopts;
extern crate serde_json;

use getopts::Options;
use std::env;
use std::process;
use std::io::{stdin, stdout, Read, Write};
use std::fs::File;

use nestools::sprites::sheet::SheetPatternTable;
use nestools::sprites::PatternTable;

fn usage(program: &str, opts: Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    opts.usage(&brief)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let program = args[0].clone();

    opts.optopt("i", "input", "input json description file.  Defaults to stdin.", "INPUT");
    opts.optopt("o", "char", "output NES char file name. Defaults to stdout.", "INPUT");
    opts.optopt("c", "header", "output C header file name", "INPUT");
    opts.optopt("a", "asm", "output asm define file name", "INPUT");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            eprintln!("{}", f);
            eprint!("{}", usage(&program, opts));
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print!("{}", usage(&program, opts));
        return;
    }

    let mut input: Box<Read> = match matches.opt_str("i") {
        Some(filename) => Box::new(File::open(filename).unwrap()),
        None => Box::new(stdin()),
    };

    let mut chr: Box<Write> = match matches.opt_str("o") {
        Some(filename) => Box::new(File::create(filename).unwrap()),
        None => Box::new(stdout()),
    };

    let mut header: Option<(String, File)> = match matches.opt_str("c") {
        Some(filename) => Some((filename.clone(), File::create(filename).unwrap())),
        None => None,
    };

    let mut asm: Option<(String, File)> = match matches.opt_str("a") {
        Some(filename) => Some((filename.clone(), File::create(filename).unwrap())),
        None => None,
    };

    let sheet_pattern_table: SheetPatternTable = serde_json::from_reader(input).unwrap();
    let pattern_table = PatternTable::from_sheet_pattern_table(sheet_pattern_table).unwrap();
    pattern_table.write(&mut chr);

    if let Some((filename, mut file)) = header {
        let guard_string: String = filename.replace(".", "_").replace("/", "_").replace("\\", "_").to_uppercase();
        let guard_string_fixed: String = guard_string.trim_matches('_').to_string();
        writeln!(file, "#ifndef {}", guard_string_fixed);
        writeln!(file, "#define {}", guard_string_fixed);
        for (index, tile) in pattern_table.left.iter().enumerate() {
            if let Some(ref name) = tile.name {
                writeln!(file, "#define S_LEFT_{} {}", name, index);
            }
        }
        for (index, tile) in pattern_table.right.iter().enumerate() {
            if let Some(ref name) = tile.name {
                writeln!(file, "#define S_RIGHT_{} {}", name, index);
            }
        }
        writeln!(file, "#endif /* {} */", guard_string_fixed);
        file.sync_all();
    }

    if let Some((filename, mut file)) = asm {
        let guard_string: String = filename.replace(".", "_").replace("/", "_").replace("\\", "_").to_uppercase();
        let guard_string_fixed: String = guard_string.trim_matches('_').to_string();
        writeln!(file, ".ifndef {}", guard_string_fixed);
        writeln!(file, "{} = 1", guard_string_fixed);
        for (index, tile) in pattern_table.left.iter().enumerate() {
            if let Some(ref name) = tile.name {
                writeln!(file, "S_LEFT_{} = {}", name, index);
            }
        }
        for (index, tile) in pattern_table.right.iter().enumerate() {
            if let Some(ref name) = tile.name {
                writeln!(file, "S_RIGHT_{} = {}", name, index);
            }
        }
        writeln!(file, ".endif ; {}", guard_string_fixed);
        file.sync_all();
    }
}
