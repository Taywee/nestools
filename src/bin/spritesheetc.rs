//! Compiles spritesheets, using a meta file, into sets of headers and a chr file.

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

    let mut header: Option<File> = match matches.opt_str("c") {
        Some(filename) => Some(File::create(filename).unwrap()),
        None => None,
    };

    let mut asm: Option<File> = match matches.opt_str("a") {
        Some(filename) => Some(File::create(filename).unwrap()),
        None => None,
    };

    let sheet_pattern_table: SheetPatternTable = serde_json::from_reader(input).unwrap();
    let pattern_table = PatternTable::from_sheet_pattern_table(sheet_pattern_table).unwrap();
    pattern_table.write(&mut chr);
}
