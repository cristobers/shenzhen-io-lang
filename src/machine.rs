use crate::{instruction::Arg, instruction::Instruction, register::Register};
use std::collections::HashMap;

/// Given a line, execute it, alter registers if need be, update the program counter.
pub fn exec(
    line: &(Instruction, Vec<Arg>),
    registers: &mut HashMap<String, Register>,
    labels: &HashMap<Arg, usize>,
    pc: usize,
    branch: &bool,
) -> (usize, bool) {
    let unchanged_branch: bool = *branch;
    let (instr, args) = line;

    if args.contains(&Arg::BranchTrue) && *branch == false {
        return (pc + 1, unchanged_branch);
    } else if args.contains(&Arg::BranchFalse) && *branch == true {
        return (pc + 1, unchanged_branch);
    }

    match instr {
        Instruction::Nop | Instruction::Label => {
            return (pc + 1, unchanged_branch);
        }
        Instruction::Add => {
            // add R/I
            let first_arg = args[0].clone();
            let acc = registers.get("acc").unwrap();
            let mut value = acc.value;
            match first_arg {
                Arg::Register(first) => {
                    let reg_value = get_register_value(first, registers);
                    value += reg_value.value;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                Arg::Number(i) => {
                    value += i;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                _ => (),
            };
            return (pc + 1, unchanged_branch);
        }
        Instruction::Sub => {
            // sub R/I
            let first_arg = args[0].clone();
            let acc = registers.get("acc").unwrap();
            let mut value = acc.value;
            match first_arg {
                Arg::Register(first) => {
                    let reg_value = get_register_value(first, registers);
                    //let reg_value = registers.get(&first).unwrap();
                    value -= reg_value.value;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                Arg::Number(i) => {
                    value -= i;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                _ => panic!("Incorrect argument given."),
            };
            return (pc + 1, unchanged_branch);
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
            return (pc + 1, unchanged_branch);
        }
        Instruction::Mul => {
            // mul R/I
            let first_arg = args[0].clone();
            let acc = registers.get("acc").unwrap();
            let mut value = acc.value;
            match first_arg {
                Arg::Register(first) => {
                    let reg_value = get_register_value(first, registers);
                    value *= reg_value.value;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                Arg::Number(i) => {
                    value *= i;
                    let _ = registers.insert("acc".to_owned(), Register { value: value });
                }
                _ => (),
            };
            return (pc + 1, unchanged_branch);
        }
        Instruction::Mov => {
            // mov R/I R
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    let first_reg = get_register_value(first, registers);
                    let _ = registers.insert(
                        second,
                        Register {
                            value: first_reg.value,
                        },
                    );
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    // check that the register exists.
                    let _ = get_register_value(second.to_owned(), registers);
                    let _ = registers.insert(
                        second.clone(),
                        Register {
                            value: i.to_owned(),
                        },
                    );
                }
                _ => (),
            };
            return (pc + 1, unchanged_branch);
        }
        Instruction::Teq => {
            // teq R/I R/I
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            let new_branch_val: bool;
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    // Comparison between registers.
                    let first_reg = get_register_value(first, registers);
                    let secnd_reg = get_register_value(second, registers);
                    new_branch_val = match first_reg.value == secnd_reg.value {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Register(first), Arg::Number(i)) => {
                    // Comparison between a register and a number.
                    let first_reg = get_register_value(first, registers);
                    new_branch_val = match first_reg.value == i {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    // Comparison a number and a register.
                    let secnd_reg = get_register_value(second, registers);
                    new_branch_val = match secnd_reg.value == i {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Number(j)) => {
                    // Comparison between a number and a number.
                    new_branch_val = match j == i {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                _ => panic!("Failed to parse arguments to teq."),
            }
        }
        Instruction::Tgt => {
            // tgt R/I R/I
            // Test to see if the value of the first operand is greater than the value
            // of the second operand.
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            let new_branch_val: bool;
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    let first_reg = get_register_value(first, registers);
                    let secnd_reg = get_register_value(second, registers);
                    new_branch_val = match first_reg.value > secnd_reg.value {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Number(j)) => {
                    new_branch_val = match i > j {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Register(first), Arg::Number(i)) => {
                    let first_reg = get_register_value(first, registers);
                    new_branch_val = match first_reg.value > i {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    let secnd_reg = get_register_value(second, registers);
                    new_branch_val = match i > secnd_reg.value {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                _ => panic!("Failed to parse the arguments to tgt."),
            }
        }
        Instruction::Tlt => {
            // tlt R/I R/I
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            let new_branch_val: bool;
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    let first_reg = get_register_value(first, registers);
                    let secnd_reg = get_register_value(second, registers);
                    new_branch_val = match first_reg.value < secnd_reg.value {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Number(j)) => {
                    new_branch_val = match i < j {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Register(first), Arg::Number(i)) => {
                    let first_reg = get_register_value(first, registers);
                    new_branch_val = match first_reg.value < i {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    let secnd_reg = get_register_value(second, registers);
                    new_branch_val = match i < secnd_reg.value {
                        true => true,
                        _ => false,
                    };
                    return (pc + 1, new_branch_val);
                }
                u => panic!("Unknown arguments given to tlt. {:?}", u),
            }
        }
        Instruction::Jmp => {
            // jmp L
            let location = args[0].clone();
            let _ = match &location {
                Arg::Label(_) => {
                    let position = labels.get(&location).unwrap();
                    return (*position, unchanged_branch);
                }
                _ => panic!("Argument provided to jmp was not a label."),
            };
        }
    }
}

fn get_register_value(arg: String, registers: &HashMap<String, Register>) -> &Register {
    match registers.get(&arg) {
        Some(v) => v,
        None => {
            println!("Failed to find a register called {:?}", arg);
            panic!();
        }
    }
}