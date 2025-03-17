use crate::{instruction::Arg, instruction::Instruction, register::Register};
use std::collections::HashMap;

/// Given a line, execute it.
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
        println!("Ignoring instruction.");
        return (pc + 1, unchanged_branch);
    } else if args.contains(&Arg::BranchFalse) && *branch == true {
        println!("Ignoring instruction.");
        return (pc + 1, unchanged_branch);
    }

    match instr {
        Instruction::Nop | Instruction::Label | Instruction::Empty => {
            return (pc + 1, unchanged_branch);
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
            return (pc + 1, unchanged_branch);
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
            return (pc + 1, unchanged_branch);
        }
        Instruction::Mov => {
            // mov R/I R
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    let first_reg = registers.get(&first).unwrap();
                    // let _ = registers.get(&second).unwrap();
                    let _ = registers.insert(
                        second,
                        Register {
                            value: first_reg.value,
                        },
                    );
                }
                (Arg::Number(i), Arg::Register(second)) => {
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
            let first_arg = args[0].clone();
            let second_arg = args[1].clone();
            let new_branch_val: bool;
            match (first_arg, second_arg) {
                (Arg::Register(first), Arg::Register(second)) => {
                    // Comparison between registers.
                    let first_reg = registers.get(&first).unwrap();
                    let secnd_reg = registers.get(&second).unwrap();
                    let _ = match first_reg.value == secnd_reg.value {
                        true => {
                            new_branch_val = true
                        }
                        _ => new_branch_val = false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Register(first), Arg::Number(i)) => {
                    // Comparison between a register and a number.
                    let first_reg = registers.get(&first).unwrap();
                    let _ = match first_reg.value == i {
                        true => new_branch_val = true,
                        _ => new_branch_val = false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    // Comparison a number and a register.
                    let second_reg = registers.get(&second).unwrap();
                    let _ = match second_reg.value == i {
                        true => new_branch_val = true,
                        _ => new_branch_val = false,
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Number(j)) => {
                    // Comparison between a number and a number.
                    let _ = match j == i {
                        true => new_branch_val = true,
                        _ => new_branch_val = false,
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
                    let first_reg = registers.get(&first).unwrap();
                    let secnd_reg = registers.get(&second).unwrap();
                    new_branch_val = match first_reg.value == secnd_reg.value {
                        true => true,
                        _    => false
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Number(j)) => {
                    new_branch_val = match i > j {
                        true => true,
                        _    => false
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Register(first), Arg::Number(i)) => {
                    let first_reg = registers.get(&first).unwrap();
                    new_branch_val = match first_reg.value > i {
                        true => true,
                        _    => false
                    };
                    return (pc + 1, new_branch_val);
                }
                (Arg::Number(i), Arg::Register(second)) => {
                    let secnd_reg = registers.get(&second).unwrap();
                    new_branch_val = match i > secnd_reg.value {
                        true => true,
                        _    => false
                    };
                    return (pc + 1, new_branch_val);
                }
                _ => panic!("Failed to parse the arguments to tgt."),
            }
        }
        Instruction::Jmp => {
            let location = args[0].clone();
            println!("Trying to jump to: {:?}", &location);
            let position = labels.get(&location).unwrap();
            return (*position, unchanged_branch);
        }
        _ => todo!(),
    }
}