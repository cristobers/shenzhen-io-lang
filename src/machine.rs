use crate::{instruction::Arg, instruction::Instruction, register::Register};
use std::{
    collections::HashMap,
    mem::{self, Discriminant},
};

/// Given a line, execute it.
pub fn exec(line: (Instruction, Vec<Arg>), registers: &mut HashMap<String, Register>) -> () {
    let (instr, args) = line;
    match instr {
        Instruction::Nop | Instruction::Label | Instruction::Empty => (),
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
                _ => (),
            };
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
        }
        _ => todo!(),
    }
}

fn arg_check(registers: &HashMap<&str, Register>) -> bool {
    return registers.keys().len() <= 2;
}
