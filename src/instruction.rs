/*
    Definitions of instructions for the machines.
*/

/// Instructions for the Shenzhen I/O assembly language.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Label,
    Nop,
    Mov,
    Add,
    Sub,
    Mul,
    Not,
    Jmp,
    Teq,
    Tgt,
    Tlt,
    /*
    Empty,
    Comment,
    Slp,
    Slx,
    Dgt,
    Dst
    */
}

/// Types that an argument to an instruction can be.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Arg {
    Number(u64),
    Register(String),
    Label(String),
    BranchTrue,
    BranchFalse,
}
