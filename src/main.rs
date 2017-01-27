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


fn parse(text: &str) -> Node
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
 * <iter> ::= <base> OR <base> '*'
 * <base> ::= <char> OR '\' <char> OR '(' <regex> ')'
 */
fn parse_regex(text: &str) -> Option<(Node, &str)>
{
    //println!("parse_regex '{}'", text);
    parse_alt(text)
}

fn parse_alt(text: &str) -> Option<(Node, &str)> {
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
                        Some((Node::new_alternation(t1, t2), rmdr2))
                }
            }
        }
    }
}

fn parse_conc(text: &str) -> Option<(Node, &str)> {
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
                        Some((Node::new_concatenation(t1, t2), rmdr2))
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
fn parse_iter(text: &str) -> Option<(Node, &str)> {
    //println!("parse_iter '{}'", text);
    match parse_atom(text) {
        None => None,
        Some((mut t1, mut rmdr1)) => {
            while rmdr1.starts_with("*") {
                t1 = Node::new_iteration(t1);
                rmdr1 = &rmdr1[1..];
            }
            Some((t1, rmdr1))
        }
    }
}

fn parse_atom(text: &str) -> Option<(Node, &str)> {
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
    } else {
        Some((Node::new_atom(text.chars().next().unwrap()), &text[1..]))
    }
}

fn is_operator(ch: char) -> bool {
    match ch {
        '|' | '*' | ')'  => true,
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
struct Node {
    op: TermType,
    subs: Vec<Node>,
}

impl Node {

    fn new_epsilon() -> Node {
        Node {
            op: TermType::Epsilon,
            subs: vec!()
        }
    }

    fn new_atom(contents: char) -> Node {
        Node {
            op: TermType::Atom(contents),
            subs: vec!()
        }
    }

    fn new_concatenation(left: Node, right: Node) -> Node {
        Node {
            op: TermType::Concatenation,
            subs: vec!(left, right)
        }
    }

    fn new_alternation(left: Node, right: Node) -> Node {
        Node {
            op: TermType::Alternation,
            subs: vec!(left, right)
        }
    }

    fn new_iteration(sub: Node) -> Node {
        Node {
            op: TermType::Iteration,
            subs: vec!(sub)
        }
    }

    fn new_positive_iteration(sub: Node) -> Node {
        Node {
            op: TermType::PositiveIteration,
            subs: vec!(sub)
        }
    }

    fn new_optional(sub: Node) -> Node {
        Node {
            op: TermType::Optional,
            subs: vec!(sub)
        }
    }



    fn pretty_print(&self, tab: u8) -> fmt::Result {
        tab_over(tab);
        match self.op {
            TermType::Epsilon => { println!("EPSILON"); },
            TermType::Atom(c) => { println!("ATOM '{}'", c); },
            TermType::Concatenation => { println!("CONCATENATION"); },
            TermType::Alternation => { println!("ALTERNATION"); },
            TermType::Iteration => { println!("ITERATION"); },
            TermType::PositiveIteration => { println!("POSITIVE_ITERATION"); },
            TermType::Optional => { println!("OPTIONAL"); },
        }
        for t in &self.subs {
            t.pretty_print(tab + 4);
        }
        Ok(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Node::pretty_print(&self, 0)
    }
}

fn tab_over(n: u8) {
    for _ in 0..n {
        print!(" ");
    }
}
