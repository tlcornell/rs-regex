use std::fmt;

#[derive(Debug)]
pub enum TermType {
    Alternation,
    Concatenation,
    Iteration,
    PositiveIteration,
    Optional,
    Atom(char),
    CharClassTerm(CharClassData),
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
        TermType::CharClassTerm(ref ccd) => { print!("CHAR_CLASS {}", ccd); },
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



#[derive(Debug)]
pub struct CharClassData {
    positive: bool,
    ranges: Vec<CharRange>,
}


/**
 * The implementation of matches() doesn't really belong here.
 * It has to harmonize with other matches() methods used by the interpreter.
 * So probably there needs to be a trait defined somewhere that 
 * allows us to extend CharClassData with what we need to interpret it.
 * This is all because this struct is shared between the char class term
 * and the char class instruction.
 */
impl CharClassData {

    pub fn new(pos: bool, rngs: Vec<CharRange>) -> CharClassData {
        CharClassData {
            positive: pos,
            ranges: rngs,       // take ownership
        }
    }
    
    pub fn matches(&self, ch: &str) -> bool {
        for rng in &self.ranges {
            if ch >= &rng.begin[..] && ch < &rng.end[..] {
                return true;
            }
        }
        false
    }
    
}

impl fmt::Display for CharClassData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.positive {
            try!(write!(f, "NOT "));
        }
        for rng in &self.ranges {
            try!(write!(f, "{} ", rng));
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct CharRange {
    begin: String,
    end: String,
}

impl CharRange {
    pub fn new(b: String, e: String) -> CharRange {
        CharRange {
            begin: b,
            end: e,
        }
    }
}

impl fmt::Display for CharRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.end == self.begin {
            write!(f, "{}", self.begin)
        } else {
            write!(f, "{}-{}", self.begin, self.end)
        }
    }
}
