extern crate nestools;
extern crate getopts;

use nestools::binaries::spritesheetc::{Config, run};
use getopts::Options;
use std::env;
use std::process;

fn usage(program: &str, opts: Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    opts.usage(&brief)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let program = args[0].clone();

    opts.optopt("i", "input", "input json description file.  Defaults to stdin.", "FILE");
    opts.optopt("o", "char", "output NES char file name. Defaults to stdout.", "FILE");
    opts.optopt("c", "header", "output C header file name", "FILE");
    opts.optopt("a", "asm", "output asm header file name", "FILE");
    opts.optopt("p", "prefix", "the prefix for the header defines", "PREFIX");
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

    let config = Config {
        input:  matches.opt_str("i"),
        chr: matches.opt_str("o"),
        header: matches.opt_str("c"),
        asm: matches.opt_str("a"),
        prefix: match matches.opt_str("p") {
            Some(prefix) => prefix,
            None => String::new(),
        },
    };

    if let Err(output) = run(config) {
        eprintln!("ERROR: {}", output);
        process::exit(1);
    }
}
