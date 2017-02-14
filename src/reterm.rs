use std::fmt;

#[derive(Debug)]
pub enum TermType {
    Alternation,
    Concatenation,
    Iteration,
    PositiveIteration,
    Optional,
    Atom(char),
}

#[derive(Debug)]
pub struct Term {
    pub op: TermType,
    pub subs: Vec<Term>,
}

impl Term {

    /**
     * Note that there's no arity checking between the op and the
     * sub-term array. So far all our operators have strict arity 
     * requirements, so such a check should probably be added.
     */
    pub fn new(op: TermType, subs: Vec<Term>) -> Term {
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

/**
 * There has to be a better way to do this, but for the life of me,
 * I can't find it.
 */
fn tab_over(n: usize) {
    for _ in 0..n {
        print!(" ");
    }
}

fn print_label(t: &Term) {
    match t.op {
        TermType::Atom(c) => { print!("ATOM '{}'", c); },
        TermType::Concatenation => { print!("CONCATENATION"); },
        TermType::Alternation => { print!("ALTERNATION"); },
        TermType::Iteration => { print!("FREE_ITERATION"); },
        TermType::PositiveIteration => { print!("POSITIVE_ITERATION"); },
        TermType::Optional => { print!("OPTIONAL"); },
    }
}


fn pretty_print(t: &Term, tab: usize) -> fmt::Result {
    tab_over(tab);
    print_label(t);
    println!("");
    for sb in &t.subs {
        pretty_print(sb, tab + 4).unwrap();
    }
    Ok(())
}
