/**
 * Thompson style "breadth first" NFA interpreter.
 * Add dynamic programming, and you get a "just in time" DFA compiler.
 */


use retrans::*;
use sparse::SparseSet;  // cribbed from regex crate, and from its ancestors



struct TaskList {
    t: SparseSet //VecDeque<Task>
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



pub struct ThompsonInterpreter {
    pub matches: Vec<usize>,    // string positions where matches ended
}

impl ThompsonInterpreter {

    pub fn new() -> ThompsonInterpreter {
        ThompsonInterpreter {
            matches: vec!(),
        }
    }

    /**
     * Returns? 
     * Success/Failure (Option?)
     * On Success, something including the match span...
     */
    pub fn apply(&mut self, prog: &Program, text: &str) {
        use retrans::Instruction::*;

        let plen = prog.len();
        let mut clist = TaskList::new(plen);
        let mut nlist = TaskList::new(plen);

        clist.add_task(0);  // start of program 
        for (str_pos, ch) in text.char_indices() {
            if clist.is_empty() {
                println!("clist empty -- bailing out");
                break;
            }
            println!("str_pos = {}", str_pos);
            let mut i: usize = 0;
            loop {
                if i >= clist.len() {
                    break;
                }

                let pc = clist.t.at(i);
                i += 1;

                println!("Executing instruction at line {}", pc);
                let inst = &prog[pc];
                match *inst {
                    Char(ic) => {
                        if ic == ch {
                            println!("Add task to nlist at {}", pc + 1);
                            nlist.add_task(pc + 1);
                        }
                        // otherwise the thread dies here
                    },
                    Match => {
                        println!("Match");
                        self.matches.push(str_pos);
                    },
                    Jump(lbl) => {
                        println!("Task at {} added to clist", lbl);
                        clist.add_task(lbl);
                    },
                    Split(l1, l2) => {
                        println!("Task at {} added to clist", l1);
                        clist.add_task(l1);
                        println!("Task at {} added to clist", l2);
                        clist.add_task(l2);
                    },
                    Abort => {
                        panic!("Encountered Abort at {}", pc);
                    }
                }
            }
            // rebind clist and nlist
            let tmp = clist;
            clist = nlist;
            nlist = tmp;
            nlist.clear();
        }
    }
}