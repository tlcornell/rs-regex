/**
 * Parsing functions should take a _parse state_ object and return 
 * a _tree_ and an updated parse-state.
 * The parse state consists of a 'next-char' position, and a reference
 * to the text. Which I think is the same as having it be a _slice_ 
 * representing the remaining unparsed string.
 */

use std::fmt;
use std::io;
use std::io::prelude::*;


fn main() {
    // Get stdin into a string
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).unwrap();
    println!("{}", s);

    let t = parse(&s);
    println!("{}", t);
}


fn parse(text: &str) -> Term
{
    match parse_regex(text) {
        Some((t, s)) => {
            if !s.is_empty() {
                println!("Did not parse whole string. Remainder: '{}'", s);
            }
            t
        },
        None => panic!("Parse failed!")
    }
}

/**
 * <regex> ::= <alt>
 * <alt> ::= <conc> OR <conc> '|' <alt>
 * <conc> ::= <iter> OR <iter> <conc>
 * <iter> ::= <base> OR <iter> '*' OR <iter> '+' OR <iter> '?'
 * <base> ::= <char> OR '(' <regex> ')' OR '\' <char> OR '.'
 */
fn parse_regex(text: &str) -> Option<(Term, &str)>
{
    //println!("parse_regex '{}'", text);
    parse_alt(text)
}

fn parse_alt(text: &str) -> Option<(Term, &str)> {
    //println!("parse_alt '{}'", text);
    match parse_conc(text) {
        None => None,
        Some((t1, rmdr1)) => {
            if !rmdr1.starts_with("|") {
                Some((t1, rmdr1))
            } else {
                match parse_alt(&rmdr1[1..]) {
                    None => None,
                    Some((t2, rmdr2)) => 
                        Some((Term::new(TermType::Alternation, vec!(t1, t2)), rmdr2))
                }
            }
        }
    }
}

fn parse_conc(text: &str) -> Option<(Term, &str)> {
    //println!("parse_conc '{}'", text);
    match parse_iter(text) {
        None => None,
        Some((t1, rmdr1)) => {
            if rmdr1.is_empty() || is_operator(rmdr1.chars().next().unwrap()) {
                Some((t1, rmdr1))
            } else {
                match parse_conc(rmdr1) {
                    None => None,
                    Some((t2, rmdr2)) =>
                        Some((Term::new(TermType::Concatenation, vec!(t1, t2)), rmdr2))
                }
            }
        }
    }
}

/**
 * Because of expressions like 'b**', the rule has to be:
 *    <iter> -> <iter> '*'
 * But this is left-recursive.
 */
fn parse_iter(text: &str) -> Option<(Term, &str)> {
    //println!("parse_iter '{}'", text);
    match parse_atom(text) {
        None => None,
        Some((mut t1, mut rmdr1)) => {
            loop {
                match rmdr1.chars().next() {
                    None => break,
                    Some(c1) => match c1 {
                        '*' => t1 = Term::new(TermType::Iteration, vec!(t1)),
                        '+' => t1 = Term::new(TermType::PositiveIteration, vec!(t1)),
                        '?' => t1 = Term::new(TermType::Optional, vec!(t1)),
                        _ => break
                    }
                }
                rmdr1 = &rmdr1[1..];
            }
            Some((t1, rmdr1))
        }
    }
}

fn parse_atom(text: &str) -> Option<(Term, &str)> {
    //println!("parse_atom '{}'", text);
    if text.starts_with("(") {
        match parse_regex(&text[1..]) {
            None => None,
            Some((t, rmdr)) => {
                if !rmdr.starts_with(")") {
                    None
                } else {
                    Some((t, &rmdr[1..]))
                }
            }
        }
    } else if text.starts_with("\\") {
        let optc = text.chars().nth(1);
        match optc {
            None => panic!("String ends in a backslash"),
            Some('e') => Some((Term::new(TermType::Epsilon, vec!()), &text[2..])),
            Some(c) => Some((Term::new(TermType::Atom(c), vec!()), &text[2..]))
        }
    } else {
        let c = text.chars().next().unwrap();
        Some((Term::new(TermType::Atom(c), vec!()), &text[1..]))
    }
}

/**
 * Used to tell when something is a boundary for concatenation.
 * No string that starts with one of these can be concatenated
 * with the preceding term.
 */
fn is_operator(ch: char) -> bool {
    match ch {
        '|' | '*' | '+' | '?' | ')'  => true,
        _ => false
    }
}


#[derive(Debug)]
enum TermType {
    Alternation,
    Concatenation,
    Iteration,
    PositiveIteration,
    Optional,
    Atom(char),
    Epsilon,
}

#[derive(Debug)]
struct Term {
    op: TermType,
    subs: Vec<Term>,
}

impl Term {

    /**
     * Note that there's no arity checking between the op and the
     * sub-term array. So far all our operators have strict arity 
     * requirements, so such a check should probably be added.
     */
    fn new(op: TermType, subs: Vec<Term>) -> Term {
        Term {
            op: op,
            subs: subs
        }
    }



}

impl fmt::Display for Term {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        pretty_print(&self, 0)
    }
}

fn tab_over(n: u8) {
    for _ in 0..n {
        print!(" ");
    }
}


fn pretty_print(t: &Term, tab: u8) -> fmt::Result {
    tab_over(tab);
    match t.op {
        TermType::Epsilon => { println!("EPSILON"); },
        TermType::Atom(c) => { println!("ATOM '{}'", c); },
        TermType::Concatenation => { println!("CONCATENATION"); },
        TermType::Alternation => { println!("ALTERNATION"); },
        TermType::Iteration => { println!("FREE_ITERATION"); },
        TermType::PositiveIteration => { println!("POSITIVE_ITERATION"); },
        TermType::Optional => { println!("OPTIONAL"); },
    }
    for t in &t.subs {
        pretty_print(t, tab + 4).unwrap();
    }
    Ok(())
}
