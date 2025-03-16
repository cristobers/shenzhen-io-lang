mod instruction;
mod machine;
mod parse;
mod register;

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::hash::Hash;

use crate::instruction::Instruction;
use instruction::Arg;
use parse::split_line;
use register::Register;

fn main() {
    let mut registers: HashMap<String, Register> = HashMap::from([
        ("acc".to_owned(), Register { value: 0 }),
        ("x1".to_owned(), Register { value: 0 }),
        ("x2".to_owned(), Register { value: 0 }),
    ]);


    let file = fs::read_to_string("program.asm").unwrap();
    let mut distilled_program: Vec<(Instruction, Vec<Arg>)> = Vec::new();
    let mut labels: HashMap<Arg, usize> = HashMap::new();
    for (i, line) in file.lines().enumerate() {
        let first_pass = parse::split_line(&line);
        let parsed = parse::abstracted(first_pass);
        match parsed {
            Some(v) => {
                let (ref instr, ref args) = v;
                if args.len() > 0 {
                    // We are an instruction that takes in arguments.
                    if let Arg::Label(bob) = &args[0] {
                        if let Instruction::Label = &instr {
                            labels.insert(args[0].to_owned(), i);
                        }
                    }
                    distilled_program.push(v);
                } else {
                    distilled_program.push(v);
                }
            }
            None => (),
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
