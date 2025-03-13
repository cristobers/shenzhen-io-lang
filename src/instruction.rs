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
    /*
    Slp,
    Slx,
    Sub,
    Mul,
    Not,
    Dgt,
    Dst
    */
}

#[derive(Debug)]
pub enum Arg {
    Number(u64),
    Register(String),
    Label(String),
}
