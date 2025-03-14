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
    let mut registers: HashMap<String, Register> = HashMap::from([
        ("acc".to_owned(), Register { value: 0 }),
        ("x1".to_owned(), Register { value: 0 }),
        ("x2".to_owned(), Register { value: 0 }),
    ]);

    let file = fs::read_to_string("program.asm").unwrap();
    let mut distilled_program: Vec<(Instruction, Vec<Arg>)> = Vec::new();
    for line in file.lines() {
        let first_pass = parse::split_line(&line);
        let parsed = parse::abstacted(first_pass);
        match parsed {
            Some(v) => distilled_program.push(v),
            None => (),
        }
    }
    dbg!(&distilled_program);
    for line in distilled_program {
        machine::exec(line, &mut registers);
        dbg!(&registers);
    }
    // ok, now actually go through distilled_program and execute it
}
