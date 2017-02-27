/**
 * Thompson style "breadth first" NFA interpreter.
 * Add dynamic programming, and you get a "just in time" DFA compiler.
 *
 * Multiple patterns:
 * Append all the programs? Each one has 1 start instruction and 1 match.
 * Ideally we want to keep track of which Match instructions we encounter,
 * not just which string positions we are in when we hit a Match.
 * Appending all programs means we still just have one clist and one nlist.
 */


use std::mem::swap;
use reprog::*;
use sparse::SparseSet; // cribbed from regex crate, and from its ancestors



struct TaskList {
    t: SparseSet,
}

impl TaskList {
    pub fn new(len: usize) -> TaskList {
        TaskList { t: SparseSet::new(len) }
    }

    pub fn clear(&mut self) {
        self.t.clear();
    }

    pub fn len(&self) -> usize {
        self.t.len()
    }

    pub fn is_empty(&self) -> bool {
        self.t.is_empty()
    }

    pub fn add_task(&mut self, pc: Label) {
        //println!("Adding task with pc = {}", pc);
        if !self.t.contains(pc) {
            self.t.insert(pc);
        }
    }
}


#[derive(Debug)]
pub struct MatchRecord {
    pub pos: usize,
    pub rule: usize,
}

impl MatchRecord {
    pub fn new(p: usize, r: usize) -> MatchRecord {
        MatchRecord { pos: p, rule: r }
    }
}



pub struct ThompsonInterpreter<'a> {
    pub matches: Vec<MatchRecord>, // string positions where matches ended
    prog: &'a Program,
}

impl<'a> ThompsonInterpreter<'a> {
    pub fn new(p: &Program) -> ThompsonInterpreter {
        ThompsonInterpreter {
            matches: vec![],
            prog: p,
        }
    }

    fn step(
        &mut self, 
        str_pos: usize, 
        ch: char, 
        clist: &mut TaskList, 
        nlist: &mut TaskList
    ) {

        use reprog::Instruction::*;

        //println!("str_pos = {}", str_pos);
        let mut i: usize = 0;
        loop {
            if i >= clist.len() {
                return; // really we want to break out of the outer loop here...
            }

            let pc = clist.t.at(i);
            i += 1;

            //println!("Executing instruction at line {}", pc);
            let inst = &self.prog[pc];
            match *inst {
                Char(ic) => {
                    if ic == ch {
                        //println!("Add task to nlist at {}", pc + 1);
                        nlist.add_task(pc + 1);
                    }
                    // otherwise the thread dies here
                }
                AnyChar => {
                    nlist.add_task(pc + 1);
                }
                CharClass(_) => {

                }
                Match(r) => {
                    //println!("Match");
                    self.matches.push(MatchRecord::new(str_pos, r));
                }
                Jump(lbl) => {
                    //println!("Task at {} added to clist", lbl);
                    clist.add_task(lbl);
                }
                Split(l1, l2) => {
                    //println!("Task at {} added to clist", l1);
                    clist.add_task(l1);
                    //println!("Task at {} added to clist", l2);
                    clist.add_task(l2);
                }
                Abort => {
                    panic!("Encountered Abort at {}", pc);
                }
            }
        }

    }


    /**
     * Returns?
     * Success/Failure (Option?)
     * On Success, something including the match span...
     */
    pub fn apply(&mut self, text: &str) {

        let plen = self.prog.len();
        let mut clist = TaskList::new(plen);
        let mut nlist = TaskList::new(plen);

        for start in &self.prog.starts {
            println!(">> Adding entry point {} to clist", *start);
            clist.add_task(*start);
        }
        for (str_pos, ch) in text.char_indices() {
            if clist.is_empty() {
                println!(">> clist empty -- bailing out");
                break;
            }
            self.step(str_pos, ch, &mut clist, &mut nlist);
            // rebind clist and nlist
            swap(&mut clist, &mut nlist);
            nlist.clear();
        }
    }
}
