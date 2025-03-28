use instruction::Arg;
use std::collections::VecDeque;

use crate::instruction;
use crate::instruction::Instruction;

/// Turn the tuple of (String, args) to Option<(instruction::Instruction, args)>
/// 
/// Returns None if parsed instruction is something we dont want, (e.g. a comment.)
pub fn abstracted(
    instruction: (String, Vec<String>),
) -> Option<(instruction::Instruction, Vec<Arg>)> {
    let (instr, args) = instruction;

    if instr.is_empty() {
        return None;
    }

    if instr.contains(":") {
        let label = instr.replace(":", "");
        let label_name = Arg::Label(label);
        return Some((Instruction::Label, Vec::from([label_name])));
    }

    let first_char = instr.chars().nth(0).unwrap();
    match first_char {
        '#' => return None,
        _ => (),
    }

    let parsed_instruction = match instr.as_ref() {
        "nop" => Instruction::Nop,
        "add" => Instruction::Add,
        "mov" => Instruction::Mov,
        "sub" => Instruction::Sub,
        "mul" => Instruction::Mul,
        "not" => Instruction::Not,
        "jmp" => Instruction::Jmp,
        "teq" => Instruction::Teq,
        "tgt" => Instruction::Tgt,
        "tlt" => Instruction::Tlt,
        _ => todo!(),
    };

    let mut should_branch_true: bool = false;
    let mut should_branch_false: bool = false;
    let mut arguments: Vec<Arg> = Vec::new();
    for el in args {
        if let Ok(v) = el.parse::<u64>() {
            arguments.push(Arg::Number(v));
        } else {
            if el.len() == 1 {
                let character = el.chars().nth(0).unwrap();
                let _ = match character {
                    '+' => should_branch_true = true,
                    '-' => should_branch_false = true,
                    _ => (),
                };
            } else {
                if instr == "jmp" {
                    arguments.push(Arg::Label(el));
                } else {
                    arguments.push(Arg::Register(el));
                }
            }
        }
    }
    if should_branch_true {
        arguments.push(Arg::BranchTrue);
    } else if should_branch_false {
        arguments.push(Arg::BranchFalse);
    }
    Some((parsed_instruction, arguments))
}

/// Given a line, determine what it is and split it into (instruction, args)
pub fn split_line(line: &str) -> (String, Vec<String>) {
    let first_char = line.chars().nth(0);
    if let Some(_) = first_char {
        match line.chars().nth(0).unwrap() {
            '#' | ':' => (line.to_owned(), Vec::new()),
            _ => {
                if let Ok(v) = parse_instruction(line) {
                    return v;
                } else {
                    (String::new(), Vec::new())
                }
            }
        }
    } else {
        (String::new(), Vec::new())
    }
}

/// Given an instruction line, turn it into a tuple of (instruction, [args])
pub fn parse_instruction(line: &str) -> Result<(String, Vec<String>), String> {
    let mut split: VecDeque<String> = line
        .split_ascii_whitespace()
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    let instruction: String;
    if split.len() == 0 {
        return Err(String::from("Empty string"));
    }
    if split[0] == "+" || split[0] == "-" {
        instruction = split[1].clone();
        // dbg!(&split);
        let arguments = match split.len() {
            1 => Vec::new(),
            _ => {
                let instr_arg = split.pop_front().unwrap();
                let _ = split.pop_front().unwrap();
                split.push_front(instr_arg);
                Vec::from(split.clone())
            }
        };
        Ok((instruction, arguments))
    } else {
        instruction = split[0].clone();
        let arguments = match split.len() {
            1 => Vec::new(),
            _ => {
                let _ = split.pop_front();
                Vec::from(split.clone())
            }
        };
        Ok((instruction, arguments))
    }
}

#[cfg(test)]
mod tests {
    use super::abstracted;
    use super::parse_instruction;
    use super::split_line;
    use crate::Arg;
    use crate::instruction::Instruction;

    #[test]
    fn parse_test() {
        let res = parse_instruction("nop").unwrap();
        let empty: Vec<String> = Vec::new();
        let op = String::from("nop");
        assert_eq!(res, (op, empty));
    }

    #[test]
    fn multi_arg_parse_test() {
        let res = parse_instruction("mov 3 x1").unwrap();
        let args: Vec<String> = Vec::from([String::from("3"), String::from("x1")]);
        let op = String::from("mov");
        assert_eq!(res, (op, args));
    }

    #[test]
    fn branch_arg_parse_test() {
        let res = parse_instruction("+ jmp end").unwrap();
        let args: Vec<String> = Vec::from([String::from("+"), String::from("end")]);
        let op = String::from("jmp");
        assert_eq!(res, (op, args));
    }

    #[test]
    fn abstracted_test() {
        assert_eq!(
            abstracted(parse_instruction("nop").unwrap()).unwrap(),
            (Instruction::Nop, vec![])
        );

        assert_eq!(
            abstracted(parse_instruction("add 1").unwrap()).unwrap(),
            (Instruction::Add, vec![Arg::Number(1)])
        );

        assert_eq!(
            abstracted(parse_instruction("add x1").unwrap()).unwrap(),
            (Instruction::Add, vec![Arg::Register(String::from("x1"))])
        );

        assert_eq!(
            abstracted(parse_instruction("jmp end").unwrap()).unwrap(),
            (Instruction::Jmp, vec![Arg::Label(String::from("end"))])
        );

        assert_eq!(
            abstracted(parse_instruction("teq acc 5").unwrap()).unwrap(),
            (
                Instruction::Teq,
                vec![Arg::Register(String::from("acc")), Arg::Number(5)]
            )
        );

        assert_eq!(
            abstracted(parse_instruction("mov acc x1").unwrap()).unwrap(),
            (
                Instruction::Mov,
                vec![
                    Arg::Register(String::from("acc")),
                    Arg::Register(String::from("x1"))
                ]
            )
        );

        assert_eq!(
            abstracted(parse_instruction("+ add 3").unwrap()).unwrap(),
            (Instruction::Add, vec![Arg::Number(3), Arg::BranchTrue,])
        );
    }
}
