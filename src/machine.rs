use crate::{instruction::Arg, instruction::Instruction, register::Register};
use std::{
    collections::HashMap,
    mem::{self, Discriminant},
};

/// Given a line, execute it.
pub fn exec(
    line: &(Instruction, Vec<Arg>),
    registers: &mut HashMap<String, Register>,
    labels: &HashMap<Arg, usize>,
    pc: usize,
) -> usize {
    let (instr, args) = line;
    match instr {
        Instruction::Nop | Instruction::Label | Instruction::Empty => {
            return pc + 1;
        }
        Instruction::Add => {
            // add R/I
            let first_arg = args[0].clone();
            let acc = registers.get("acc").unwrap();
            let mut value = acc.value;
            match first_arg {
                Arg::Register(first) => {
                    let reg_value = registers.get(&first).unwrap();
                    value += reg_value.value;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                Arg::Number(i) => {
                    value += i;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                _ => (),
            };
            return pc + 1;
        }
        Instruction::Sub => {
            // sub R/I
            let first_arg = args[0].clone();
            let acc = registers.get("acc").unwrap();
            let mut value = acc.value;
            match first_arg {
                Arg::Register(first) => {
                    let reg_value = registers.get(&first).unwrap();
                    value -= reg_value.value;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                Arg::Number(i) => {
                    value -= i;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                _ => panic!("Incorrect argument given."),
            };
            return pc + 1;
        }
        Instruction::Not => {
            let acc = registers.get("acc").unwrap().value;
            match acc {
                0 => {
                    let _ = registers.insert("acc".to_owned(), Register { value: 100 });
                }
                _ => {
                    let _ = registers.insert("acc".to_owned(), Register { value: 0 });
                }
            }
            return pc + 1;
        }
        Instruction::Mul => {
            // mul R/I
            let first_arg = args[0].clone();
            let acc = registers.get("acc").unwrap();
            let mut value = acc.value;
            match first_arg {
                Arg::Register(first) => {
                    let reg_value = registers.get(&first).unwrap();
                    value *= reg_value.value;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                Arg::Number(i) => {
                    value *= i;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                _ => (),
            };
            return pc + 1;
        }
        Instruction::Mov => {
            // mov R/I R
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    let first_reg = registers.get(&first).unwrap();
                    let secnd_reg = registers.get(&second).unwrap();
                    let _ = registers.insert(
                        second,
                        Register {
                            value: first_reg.value,
                        },
                    );
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    let reg = registers.insert(
                        second.clone(),
                        Register {
                            value: i.to_owned(),
                        },
                    );
                }
                _ => (),
            };
            return pc + 1;
        }
        Instruction::Teq => {
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    // Comparison between registers.
                    let first_reg = registers.get(&first).unwrap();
                    let secnd_reg = registers.get(&second).unwrap();
                    let _ = match first_reg.value == secnd_reg.value {
                        true => println!("TRUE!!!"),
                        _ => println!("FALSE!!!"),
                    };
                    return pc;
                }
                (Arg::Register(first), Arg::Number(i)) => {
                    // Comparison between a register and a number.
                    let first_reg = registers.get(&first).unwrap();
                    let _ = match first_reg.value == i {
                        true => println!("TRUE!!!"),
                        _ => println!("FALSE!!!"),
                    };
                    return pc;
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    // Comparison a number and a register.
                    return pc;
                }
                (Arg::Number(i), Arg::Number(j)) => {
                    // Comparison between a number and a number.
                    return pc;
                }
                _ => return pc
            }
        }
        Instruction::Jmp => {
            // TODO: find a label that shares a name with the location,
            //       move the program counter to it.
            let location = args[0].clone();
            let position = labels.get(&location).unwrap();
            return *position;
        }
        _ => todo!(),
    }
}

fn arg_check(registers: &HashMap<&str, Register>) -> bool {
    return registers.keys().len() <= 2;
}
