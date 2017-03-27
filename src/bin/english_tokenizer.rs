
extern crate getopts;
extern crate rs_regex;

use getopts::Options;
use std::env;
use std::process;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use rs_regex::reinterp::ThompsonInterpreter;
use rs_regex::reinterp::TokenizerAction;

/**
 * The interpreter is actually a Thompson VM partially applied to a
 * given program. That is, the regex program is like "virtual firmware"
 * in the machine. Short version: it looks like a generic interpreter, 
 * but it is not; the program it interprets is fixed during construction.
 */
struct RegexTokenizer {

    interpreter: rs_regex::reinterp::ThompsonInterpreter,

}

impl RegexTokenizer {

    fn run(&mut self, text: &str) {
        self.interpreter.apply(text);
    }

}


struct TokenizerBuilder {
    compiler: rs_regex::retrans::RegexTranslator,
    rule_nbr: usize,
    actions: Vec<TokenizerAction>,
}

impl TokenizerBuilder {

    fn new() -> TokenizerBuilder {
        TokenizerBuilder {
            compiler: rs_regex::retrans::RegexTranslator::new(),
            rule_nbr: 0,
            actions: vec![],
        }
    }

    /**
     * This should compile the pattern and add to the current program.
     */
    fn add_rule(
        mut self, 
        pattern: &str, 
        action: TokenizerAction,
    ) -> TokenizerBuilder {
        let tree = rs_regex::reparse::parse(pattern);
        println!("{}", tree);
        self.compiler.compile(&tree, self.rule_nbr);
        // TODO: The action should be added to an action list here.
        self.actions.push(action);

        self.rule_nbr += 1;
        self
    }

    fn done(mut self) -> RegexTokenizer {
        self.compiler.finish();       // ground instruction labels
        self.compiler.print_prog();
        RegexTokenizer {
            interpreter: ThompsonInterpreter::new(self.compiler.prog,
                                                  self.actions),
        }
    }

}






struct AppConfig {
    text_file: Option<String>,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig { 
            text_file: None,
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

    cfg
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
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

fn word_action (tok: &str) {
    println!("WORD [{}]", tok);
}

fn num_action(tok: &str) {
    println!("NUMBER [{}]", tok);
}

fn punct_action(tok: &str) {
    println!("PUNCT [{}]", tok);
}

fn main() {
    let cfg = configure();
    let text_src = TextSource::new(&cfg);

    let mut english_tokenizer = TokenizerBuilder::new()
        .add_rule(r"(?i)[a-z]+", word_action)           // [0] words
        .add_rule(r"[0-9,.]*[0-9]+", num_action)        // [1] numbers
        .add_rule(r"[.,?!]", punct_action)              // [2] punctuation
        .done();

    println!("\n{}", text_src.get_text());
    english_tokenizer.run(text_src.get_text());
}
