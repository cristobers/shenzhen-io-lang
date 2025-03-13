mod instruction;
mod machine;
mod parse;
mod register;

use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::instruction::Instruction;
use instruction::Arg;
use register::Register;

fn main() {
    let mut registers: HashMap<&str, Register> = HashMap::from([
        ("acc", Register { value: 0 }),
        ("x1", Register { value: 0 }),
        ("x2", Register { value: 0 }),
    ]);

    let file = fs::read_to_string("program.asm").unwrap();
    let mut parsed_lines: Vec<(Instruction, Vec<Arg>)> = Vec::new();
    for line in file.lines() {
        let first_pass = parse::split_line(&line);
        let parsed = parse::abstacted(first_pass);
        match parsed {
            Some(v) => parsed_lines.push(v),
            None => (),
        }
    }
    dbg!(parsed_lines);
}
