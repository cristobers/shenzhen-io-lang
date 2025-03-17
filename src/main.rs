mod instruction;
mod machine;
mod parse;
mod register;

use std::collections::HashMap;
use std::fs;

use crate::instruction::Instruction;
use instruction::Arg;
use register::Register;

fn main() {
    let mut registers: HashMap<String, Register> = HashMap::from([
        ("acc".to_owned(), Register { value: 0 }),
        ("x1".to_owned(), Register { value: 0 }),
        ("x2".to_owned(), Register { value: 0 }),
        ("x3".to_owned(), Register { value: 0 }),
    ]);

    let file = fs::read_to_string("program.asm").unwrap();
    let mut distilled_program: Vec<(Instruction, Vec<Arg>)> = Vec::new();
    let mut labels: HashMap<Arg, usize> = HashMap::new();
    for line in file.lines() {
        let first_pass = parse::split_line(&line);
        let parsed = parse::abstracted(first_pass);
        match parsed {
            Some(v) => {
                let (ref instr, ref args) = v;
                distilled_program.push(v);
            }
            None => (),
        }
    }

    // we want the labels to be based off of the distilled program,
    // instead of being based off of the initial parsing.
    for (i, (instr, args)) in distilled_program.iter().enumerate() {
        if let Instruction::Label = instr {
            labels.insert(args[0].to_owned(), i);
        }
    }

    let mut count = 0;
    let mut program_counter = 0;
    let mut branch: bool = false;
    while program_counter < distilled_program.len() {
        let line = &distilled_program[program_counter];
        println!("{:?}", line);
        let (pc, curr_branch) =
            machine::exec(line, &mut registers, &labels, program_counter, &branch);
        program_counter = pc;
        branch = curr_branch;
        count += 1;
        println!("{:?}", &registers);
    }
    println!("Finished execution in {} steps.", &count);
}
