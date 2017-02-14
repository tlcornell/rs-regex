use reterm::*;
use std::ops::{Index, IndexMut};

pub type Label = usize;

pub enum Instruction {
    Char(char),
    Match(usize),             // arg: rule#
    Jump(Label),
    Split(Label, Label),
    Abort
}

pub struct Program {
    code: Vec<Instruction>,
    starts: Vec<usize>,         // entry points
    // Need some way of mapping Match instructions to rule #'s.
}

impl Program {
    pub fn new() -> Program {
        Program {
            code: vec![],
            starts: vec![],
        }
    }
    pub fn len(&self) -> usize {
        self.code.len()
    }
    pub fn push(&mut self, instr: Instruction) {
        self.code.push(instr);
    }
    pub fn print(&self) {
        use self::Instruction::*;
        for (pos, inst) in self.code.iter().enumerate() {
            let ref i: Instruction = *inst;
            match *i {
                Abort => println!("{}: abort", pos),
                Char(c) => println!("{}: char {}", pos, c),
                Match(r) => println!("{}: match {}", pos, r),
                Jump(l1) => println!("{}: jmp {}", pos, l1),
                Split(l1, l2) => println!("{}: split {}, {}", pos, l1, l2)
            }
        }
    }
}

impl Index<usize> for Program {
    type Output = Instruction;
    fn index(&self, index: usize) -> &Instruction {
        &self.code[index]
    }
}

impl IndexMut<usize> for Program {
    //type Output = Instruction;
    fn index_mut(&mut self, index: usize) -> &mut Instruction {
        &mut self.code[index]
    }
}




pub struct RegexTranslator {
    pub prog: Program
}

impl RegexTranslator {
    pub fn new() -> RegexTranslator {
        RegexTranslator {
            prog: Program::new()
        }
    }

    pub fn get_program(&self) -> &Program {
        &self.prog
    }

    pub fn compile(&mut self, regex: &Term) {
        self.translate(regex);
        let pc = self.prog.len();
        self.prog.push(Instruction::Match(pc));
    }

    fn translate(&mut self, regex: &Term) {
        use self::Instruction::*;
        use reterm::TermType::*;
        match regex.op {
            Atom(c) => {
                self.prog.push(Char(c));
            },
            Alternation => self.trans_alt(regex),
            Concatenation => self.trans_conc(regex),
            Iteration => self.trans_iter(regex),
            Optional => self.trans_opt(regex),
            PositiveIteration => self.trans_pos(regex),
        }
    }

    fn trans_alt(&mut self, regex: &Term) {
        use self::Instruction::*;
        let split_pos = self.prog.len();  // location of next available slot
        let l1 = split_pos + 1;
        self.prog.push(Abort);  // placeholder for eventual split L1,L2
        // L1:
        self.translate(&regex.subs[0]);
        // jmp L3
        let jump_pos = self.prog.len();
        self.prog.push(Abort);  // placeholder for eventual jmp L3
        // L2:
        let l2 = self.prog.len();  
        self.translate(&regex.subs[1]);
        let l3 = self.prog.len();
        self.prog[split_pos] = Split(l1, l2);
        self.prog[jump_pos] = Jump(l3);   
    }

    fn trans_conc(&mut self, regex: &Term) {
        self.translate(&regex.subs[0]);
        self.translate(&regex.subs[1]);
    }

    fn trans_iter(&mut self, regex: &Term) {
        use self::Instruction::*;
        let l1 = self.prog.len();
        self.prog.push(Abort);  // --> split L2,L3 
        let l2 = self.prog.len();
        self.translate(&regex.subs[0]);
        self.prog.push(Jump(l1));
        let l3 = self.prog.len();
        self.prog[l1] = Split(l2, l3);        
    }

    fn trans_opt(&mut self, regex: &Term) {
        use self::Instruction::*;
        let split_pos = self.prog.len();
        self.prog.push(Abort);  // --> split L1,L2 
        let l1 = split_pos + 1; // == prog.len()
        self.translate(&regex.subs[0]);
        let l2 = self.prog.len();
        self.prog[split_pos] = Split(l1, l2);
    }

    fn trans_pos(&mut self, regex: &Term) {
        use self::Instruction::*;
        let l1 = self.prog.len();
        self.translate(&regex.subs[0]);
        let split_pos = self.prog.len();
        let l3 = split_pos + 1;
        self.prog.push(Split(l1, l3));
    }

    pub fn print_prog(&self) {
        self.prog.print();
    }
}
