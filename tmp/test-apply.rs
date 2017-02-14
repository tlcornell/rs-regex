/**
 * Parsing functions should take a _parse state_ object and return 
 * a _tree_ and an updated parse-state.
 * The parse state consists of a 'next-char' position, and a reference
 * to the text. Which I think is the same as having it be a _slice_ 
 * representing the remaining unparsed string.
 */

extern crate getopts;
extern crate rs_regex;

use getopts::Options;
use std::env;
use std::process;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


struct AppConfig {
    in_file: Option<String>,
    expr: String
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig { 
            in_file: None,
            expr: "".to_string()
        }
    }
}

fn configure() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this message and exit");
    opts.optopt("f", "file", "match text from file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
    }

    let mut cfg: AppConfig = AppConfig::new();
    cfg.in_file = matches.opt_str("f");
    if matches.free.is_empty() {
        // regex command line argument is required
        print_usage(&args[0], &opts);
    } else {
        cfg.expr = matches.free[0].clone();
    }

    cfg
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] REGEX", program);
    print!("{}", opts.usage(&brief));
    process::exit(1);
}

fn test(regex: &str, text: &str) {
    use rs_regex::reparse::parse;
    use rs_regex::retrans::RegexTranslator;
    use rs_regex::reinterp::ThompsonInterpreter;

    println!("{}", text);

    let tree = parse(regex);
    println!("{}", tree);

    let mut translator = RegexTranslator::new();
    // Maybe the following doesn't have to be a borrow?
    // At this point we should be done with t...
    translator.compile(&tree);
    translator.print_prog();

    let mut interpreter = ThompsonInterpreter::new(&translator.prog);
    interpreter.apply(&text);
    if interpreter.matches.len() == 0 {
        println!("There were no matches");
    } else {
        for m in interpreter.matches {
            println!("There was a match from position 0 to {}", m);
        }
    }

}

fn main() {
    // Command line parsing
    let cfg = configure();

    // Get the text to match against (from file or stdin)
    let mut text = String::new();
    match cfg.in_file {
        None => {
            let stdin = io::stdin();
            stdin.lock().read_to_string(&mut text).unwrap();
        },
        Some(fname) => {
            let fpath = Path::new(&fname);
            let mut f = File::open(fpath).unwrap();
            f.read_to_string(&mut text).unwrap();
        }
    }

    // Test: Apply the given regex to the given text string
    test(&cfg.expr, &text);
}





