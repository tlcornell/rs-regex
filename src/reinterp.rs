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
use reprog::Instruction::*;
use util::char_at;



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


#[derive(Debug, Clone, Copy)]
pub struct MatchRecord {
    pub len: usize,
    pub rule: usize,
}

impl MatchRecord {
    pub fn new(p: usize, r: usize) -> MatchRecord {
        MatchRecord { len: p, rule: r }
    }
}



pub type TokenizerAction = fn(&str) -> ();


pub struct ThompsonInterpreter {
    pub matches: Vec<MatchRecord>, // string positions where matches ended
    prog: Program,
    actions: Vec<TokenizerAction>,
}

impl ThompsonInterpreter {
    
    pub fn new(p: Program, acts: Vec<TokenizerAction>) -> ThompsonInterpreter {
        ThompsonInterpreter {
            matches: vec![],
            prog: p,
            actions: acts,
        }
    }

    fn best_match(&self) -> Option<MatchRecord> {
        if self.matches.is_empty() {
            return None;
        }
        let mut best = MatchRecord {len: 0, rule: 0};
        for m in &self.matches {
            if m.len > best.len {
                best = m.clone();
            } else if m.len == best.len {
                if m.rule < best.rule {
                    best = m.clone();
                }
            }
            // else m.len < best.len, and we continue
        }
        Some(best)
    }

    /**
     * Loop through clist. Epsilon transitions (Split) add new entries to clist,
     * so this implements epsilon-closure. All other instructions add new 
     * entries to nlist.
     * So this will apply all character tests to the current character, and
     * return when it is done.
     * There is no direct notion of failure here. If nothing is added to nlist,
     * then the whole procedure will terminate. There is a global notion of
     * failure which can be checked then, namely were there any matches. 
     */
    fn advance(
        &mut self, 
        str_pos: usize, 
        ch: char, 
        clist: &mut TaskList, 
        nlist: &mut TaskList
    ) {
        //println!("advance: '{}'", ch);
        let mut i: usize = 0;
        loop {
            if i >= clist.len() {
                //println!("finished with clist, end of match advance");
                return; // really we want to break out of the outer loop here...
            }

            let pc = clist.t.at(i);
            i += 1;

            //println!("Executing instruction at line {}", pc);
            let inst = &self.prog[pc];
            match *inst {
                Char(ref data) => {
                    if data.ch == ch {
                        //println!("Matched '{}' at string pos {}", data.ch, str_pos);
                        //println!("Add task to nlist at {}", pc + 1);
                        nlist.add_task(data.goto);
                    } else if data.nocase {
                        if data.ch.to_lowercase().collect::<String>() == 
                           ch.to_lowercase().collect::<String>() {
                            //println!("i-Matched '{}' at string pos {}", data.ch, str_pos);
                            nlist.add_task(data.goto);
                        }
                    }
                    // otherwise the thread dies here
                }
                AnyChar(ref data) => {
                    nlist.add_task(data.goto);
                }
                CharClass(ref ccd) => {
                    if ccd.data.matches(ch) {
                        //println!("CharClass {} matches {} at {}", ccd.data, ch, str_pos);
                        nlist.add_task(ccd.goto);
                    } else if ccd.nocase {
                        if ccd.data.matches(ch.to_lowercase().next().unwrap()) {
                            //println!("CharClass {} i-matches {} at {}", ccd.data, ch, str_pos);
                            nlist.add_task(ccd.goto);
                        }
                    }
                }
                Match(ref data) => {
                    //println!("Match: {} [{}]", str_pos, data.rule_id);
                    self.matches.push(MatchRecord::new(str_pos, data.rule_id));
                }
                Split(l1, l2) => {
                    //println!("Task at {} added to clist", l1);
                    clist.add_task(l1);
                    //println!("Task at {} added to clist", l2);
                    clist.add_task(l2);
                }
            }
        }

    }



    /**
     * Find a token starting at &text[begin..], if possible.
     * Results are stored in self.matches, and so "failure" is indicated
     * by an empty match list.
     */
    fn all_matches_at(&mut self, text: &str) {

        let plen = self.prog.len();
        let mut clist = TaskList::new(plen);
        let mut nlist = TaskList::new(plen);

        self.matches.clear();

        for start in &self.prog.starts {
            //println!(">> Adding entry point {} to clist", *start);
            clist.add_task(*start);
        }
        let mut pos = 0;
        let mut nxt = 0;
        let mut ch: char;
        while !clist.is_empty() {

            pos += nxt;

            match char_at(&text[pos..]) {
                None => { 
                    if pos == text.len() {
                        ch = 0 as char;
                    } else {
                        panic!("ERROR: Could not decode character at {}", pos);
                    }
                }
                Some((c, byte_len)) => {
                    nxt = byte_len;
                    ch = c;
                    //println!("pos: {}; nxt: {}; ch: '{}'", pos, nxt, ch);
                }
            }

            self.advance(pos, ch, &mut clist, &mut nlist);
            
            // rebind clist and nlist
            swap(&mut clist, &mut nlist);
            nlist.clear();
        }
    }

    /**
     * Loops over the characters in the input, but will exit early if
     * we ever reach a point where nothing in the input matches.
     * Then clist will be empty.
     * Currently, we have no way of knowing what caused termination
     * (out of string? no surviving threads?). It is just a matter of
     * whether there were any matches at that point.
     * 
     * This is not quite correct. There should be an outer loop which 
     * consumes the string, and an inner loop which finds matches.
     * When we are done looking for matches (clist is empty), we bump
     * our string position to either the end of the best match (if there
     * were any) or one position forward (if there were no matches).
     *
     * That latter is assuming that we do not require a match of some kind
     * on every character. Otherwise we have to fail harder in cases where
     * the match list comes back empty.
     */
    pub fn apply(&mut self, text: &str) {

        let mut pos: usize = 0;
        while pos < text.len() {
            self.all_matches_at(&text[pos..]);
            // Now, what is our best match, if any? 
            match self.best_match() {
                None => {
                    // increment pos by 1 and try again
                    println!("No rule matched at pos {}", pos);
                    pos += 1;
                }
                Some(mtch) => {
                    // emit a token
                    //println!("TOKEN: {} -> {} [{}]", pos, pos + mtch.len, mtch.rule);
                    self.actions[mtch.rule](&text[pos..(pos + mtch.len)]);
                    // increment pos by mtch length and continue
                    pos += mtch.len;
                }
            }
        }
    }
}
