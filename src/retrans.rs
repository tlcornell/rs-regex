use reterm::*;

pub type Label = usize;

pub enum Instruction {
    Char(char),
    Match,
    Jump(Label),
    Split(Label, Label),
    Abort
}

pub type Program = Vec<Instruction>;

pub struct RegexTranslator {
    prog: Program
}

impl RegexTranslator {
    pub fn new() -> RegexTranslator {
        RegexTranslator {
            prog: vec!()
        }
    }

    pub fn compile(&mut self, regex: &Term) {
        use self::Instruction::*;
        self.translate(regex);
        self.prog.push(Match);
    }

    fn translate(&mut self, regex: &Term) {
        use self::Instruction::*;
        use reterm::TermType::*;
        match regex.op {
            Atom(c) => {
                self.prog.push(Char(c));
            },
            Alternation => {
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
            },
            Concatenation => {
                self.translate(&regex.subs[0]);
                self.translate(&regex.subs[1]);
            },
            Iteration => {
                let l1 = self.prog.len();
                self.prog.push(Abort);  // --> split L2,L3 
                let l2 = self.prog.len();
                self.translate(&regex.subs[0]);
                self.prog.push(Jump(l1));
                let l3 = self.prog.len();
                self.prog[l1] = Split(l2, l3);
            },
            Optional => {
                let split_pos = self.prog.len();
                self.prog.push(Abort);  // --> split L1,L2 
                let l1 = split_pos + 1; // == prog.len()
                self.translate(&regex.subs[0]);
                let l2 = self.prog.len();
                self.prog[split_pos] = Split(l1, l2);
            },
            PositiveIteration => {
                let l1 = self.prog.len();
                self.translate(&regex.subs[0]);
                let split_pos = self.prog.len();
                let l3 = split_pos + 1;
                self.prog.push(Split(l1, l3));
            },
        }
    }

    pub fn print_prog(&self) {
        use self::Instruction::*;
        for (pos, inst) in self.prog.iter().enumerate() {
            match *inst {
                Abort => println!("{}: abort", pos),
                Char(c) => println!("{}: char {}", pos, c),
                Match => println!("{}: match", pos),
                Jump(l1) => println!("{}: jmp {}", pos, l1),
                Split(l1, l2) => println!("{}: split {}, {}", pos, l1, l2)
            }
        }
    }
}