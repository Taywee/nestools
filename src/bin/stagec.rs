use nestools::binaries::stagec::{Config, run};
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

    opts.optopt("i", "input", "input yaml description file.  Defaults to stdin.", "FILE");
    opts.optopt("o", "output", "output NES stage binary file name. Defaults to stdout.", "FILE");
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
        stage: matches.opt_str("o"),
    };

    if let Err(output) = run(config) {
        eprintln!("ERROR: {}", output);
        process::exit(1);
    }
}
