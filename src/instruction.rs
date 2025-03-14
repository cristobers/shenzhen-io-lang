/*
    Definitions of instructions for the machines.
*/

#[derive(Debug)]
pub enum Instruction {
    Empty,
    Comment,
    Label,
    Nop,
    Mov,
    Add,
    Sub,
    Mul,
    /*
    Slp,
    Slx,
    Not,
    Dgt,
    Dst
    */
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Arg {
    Number(u64),
    Register(String),
    Label(String),
}
