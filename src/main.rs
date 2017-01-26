use std::fmt;
use std::io;
use std::io::prelude::*;


fn main() {
    let n1 = Node::new_atom('H');
    println!("{:?}", n1);
    let n2 = Node::new_atom('e');
    println!("{:?}", n2);
    let star = Node::new_iteration(n2);
    let he = Node::new_concatenation(n1, star);
    println!("{}", he);

    // Get stdin into a string
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).unwrap();
    println!("{}", s);

    let mut chs = s.chars();
    let ep = Node::new_epsilon();
    chs
        .fold(ep, |t, c| Node::new_concatenation(t, Node::new_atom(c)))
        .pretty_print(0);

}

#[derive(Debug)]
enum TermType {
    Alternation,
    Concatenation,
    Iteration(u8, u8),
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
            op: TermType::Iteration(0,std::u8::MAX),
            subs: vec!(sub)
        }
    }



    fn pretty_print(&self, tab: u8) -> fmt::Result {
        tab_over(tab);
        match self.op {
            TermType::Epsilon => { println!("EPSILON"); },
            TermType::Atom(c) => { println!("ATOM {}", c); },
            TermType::Concatenation => { println!("CONCATENATION"); },
            TermType::Alternation => { println!("ALTERNATION"); },
            TermType::Iteration(lo,hi) => { println!("ITERATION {}..{}", lo, hi); },
        }
        for t in &self.subs {
            t.pretty_print(tab + 4);
        }
        Ok(())
    }
}

fn tab_over(n: u8) {
    for _ in 0..n {
        print!(" ");
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Node::pretty_print(&self, 0)
    }
}
