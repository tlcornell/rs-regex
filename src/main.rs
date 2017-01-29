/**
 * Parsing functions should take a _parse state_ object and return 
 * a _tree_ and an updated parse-state.
 * The parse state consists of a 'next-char' position, and a reference
 * to the text. Which I think is the same as having it be a _slice_ 
 * representing the remaining unparsed string.
 */

use std::io;
use std::io::prelude::*;

mod reterm;
mod reparse;
mod retrans;

use reparse::*;
use retrans::*;

fn main() {
    // Get stdin into a string
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).unwrap();
    println!("{}", s);

    let t = parse(&s);
    println!("{}", t);

    let mut translator = RegexTranslator::new();

    // Maybe the following doesn't have to be a borrow?
    // At this point we should be done with t...
    translator.compile(&t);
    translator.print_prog();
}





