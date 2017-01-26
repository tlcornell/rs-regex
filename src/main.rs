use std::fmt;


fn main() {
    let n1 = Node::new_atom('H');
    println!("{:?}", n1);
    let n2 = Node::new_atom('e');
    println!("{:?}", n2);
    let he = Node::new_concatenation(n1, n2);
    println!("{}", he);
}

#[derive(Debug)]
enum TermType {
    Alternation,
    Concatenation,
    Iteration,
    Atom(char),
}

#[derive(Debug)]
struct Node {
    op: TermType,
    subs: Vec<Node>,
}

impl Node {

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


    fn pretty_print(&self) -> fmt::Result {
        self.pprint_aux(0)
    }

    fn pprint_aux(&self, tab: u8) -> fmt::Result {
        tab_over(tab);
        match self.op {
            TermType::Atom(c) => { println!("ATOM {}", c); },
            TermType::Concatenation => { println!("CONCATENATION"); },
            TermType::Alternation => { println!("ALTERNATION"); },
            TermType::Iteration => { println!("ITERATION"); },
        }
        for t in &self.subs {
            t.pprint_aux(tab + 4);
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
        Node::pretty_print(&self)
    }
}
