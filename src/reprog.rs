////////////////////////////////////////////////////////////////////////////
// reprog.rs

use std::ops::{Index, IndexMut};
use std::fmt;
use reterm::CharClassData;

pub type Label = usize;

pub enum Instruction {
    Char(char),
    AnyChar,
    CharClass(CharClassData),
    Match(usize),             // arg: rule#
    Jump(Label),
    Split(Label, Label),
    Abort
}





pub struct Program {
    code: Vec<Instruction>,
    pub starts: Vec<usize>,         // entry points
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
                AnyChar => println!("{}: any_char", pos),
                CharClass(ref cc) => println!("{}: {}", pos, cc),
                Match(r) => println!("{}: match {}", pos, r),
                Jump(l1) => println!("{}: jmp {}", pos, l1),
                Split(l1, l2) => println!("{}: split {}, {}", pos, l1, l2)
            }
        }
    }
    pub fn add_start(&mut self, start: usize) {
        self.starts.push(start);
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


