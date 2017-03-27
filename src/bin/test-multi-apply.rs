//////////////////////////////////////////////////////////////////////////////
// test-multi-apply.rs
//


extern crate getopts;
extern crate rs_regex;

use getopts::Options;
use std::env;
use std::process;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;


struct AppConfig {
    text_file: Option<String>,
    expr_file: String,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig { 
            text_file: None,
            expr_file: "".to_string()
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
    cfg.text_file = matches.opt_str("f");
    if matches.free.is_empty() {
        // regex command line argument is required
        print_usage(&args[0], &opts);
    } else {
        cfg.expr_file = matches.free[0].clone();
    }

    cfg
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] REGEX-FILE", program);
    print!("{}", opts.usage(&brief));
    process::exit(1);
}



struct TextSource {
    text: String,
}

impl TextSource {
    pub fn new(cfg: &AppConfig) -> TextSource {
        // Get the text to match against (from file or stdin)
        let mut txt = String::new();
        match cfg.text_file {
            None => {
                let stdin = io::stdin();
                stdin.lock().read_to_string(&mut txt).unwrap();
            },
            Some(ref fname) => {
                let fpath = Path::new(&fname);
                let mut f = File::open(fpath).unwrap();
                f.read_to_string(&mut txt).unwrap();
            }
        }  

        TextSource { text: txt }      
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}


struct RegexSource {
    regexes: Vec<String>,
}

impl RegexSource {
    pub fn new(cfg: &AppConfig) -> RegexSource {
        let rxfname = &cfg.expr_file;
        let rxfpath = Path::new(rxfname);
        let rxf = File::open(rxfpath).unwrap();
        let rxfile = BufReader::new(&rxf);

        let mut rxs = vec![];

        for line in rxfile.lines() {
            let l1 = line.unwrap();
            if l1.is_empty() {
                continue;
            }
            if l1.chars().next() == Some('#') {
                continue;
            }
            println!("> {}", l1);
            rxs.push(l1);
        }        

        RegexSource { regexes: rxs }
    }
}




fn test(regex_src: &RegexSource, text_src: &TextSource) {
    use rs_regex::reparse::parse;
    use rs_regex::retrans::RegexTranslator;
    use rs_regex::reinterp::ThompsonInterpreter;
    use rs_regex::reinterp::TokenizerAction;

    let mut rule_nbr: usize = 0;
    let mut translator = RegexTranslator::new();
    for regex in &regex_src.regexes {
        let tree = parse(&regex);
        println!("{}", tree);
        translator.compile(&tree, rule_nbr);  // extend current program
        rule_nbr += 1;
    }

    translator.finish();
    translator.print_prog();


    let actions : Vec<TokenizerAction> = vec![];
    let mut interpreter = ThompsonInterpreter::new(translator.prog, actions);
    let text = &text_src.get_text();
    println!("{}", text);
    interpreter.apply(&text);
    if interpreter.matches.len() == 0 {
        println!("There were no matches");
    } else {
        for m in interpreter.matches {
            println!("There was a match from position 0 to {} (rule {})", m.len, m.rule);
        }
    }
}


fn main() {
    // Command line parsing
    let cfg = configure();

    let text_src = TextSource::new(&cfg);
    let regex_src = RegexSource::new(&cfg);
    // Now regex_src should be the owner of the regexes.

    // Test: Apply the given regex to the given text string
    test(&regex_src, &text_src);
}


