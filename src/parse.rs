use instruction::Arg;
use std::collections::VecDeque;

use crate::register;

use crate::instruction;
use crate::instruction::Instruction;

/// Turn the tuple of (String, args) to Option<(instruction::Instruction, args)>
/// Returns None if parsed instruction is something we dont want, (e.g. a comment.)
pub fn abstacted(
    instruction: (String, Vec<String>),
) -> Option<(instruction::Instruction, Vec<Arg>)> {
    let (instr, args) = instruction;
    let first_char = instr.chars().nth(0).unwrap();

    if instr.is_empty() {
        return Some((Instruction::Empty, Vec::new()));
    }

    match first_char {
        // '#' => return (Instruction::Comment, Vec::new()),
        '#' => return None,
        ':' => {
            let label = instr.replace(":", "");
            let label_name = Arg::Label(label);
            return Some((Instruction::Label, Vec::from([label_name])));
        }
        _ => (),
    }

    let parsed_instruction = match instr.as_ref() {
        "nop" => Instruction::Nop,
        "add" => Instruction::Add,
        "mov" => Instruction::Mov,
        "sub" => Instruction::Sub,
        "mul" => Instruction::Mul,
        "not" => Instruction::Not,
        _ => todo!(),
    };

    let mut arguments: Vec<Arg> = Vec::new();
    for el in args {
        if let Ok(v) = el.parse::<u64>() {
            arguments.push(Arg::Number(v));
        } else {
            arguments.push(Arg::Register(el));
        }
    }

    Some((parsed_instruction, arguments))
}

/// Given a line, determine what it is and split it into (instruction, args)
pub fn split_line(line: &str) -> (String, Vec<String>) {
    let first_char = line.chars().nth(0);
    if let Some(_) = first_char {
        match line.chars().nth(0).unwrap() {
            '#' | ':' => (line.to_owned(), Vec::new()),
            _ => parse_instruction(line).unwrap(),
        }
    } else {
        (String::new(), Vec::new())
    }
}

/// Given an instruction line, turn it into a tuple of (instruction, [args])
fn parse_instruction(line: &str) -> Result<(String, Vec<String>), String> {
    let mut split: VecDeque<String> = line
        .split_ascii_whitespace()
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    let instruction: String = split[0].clone();
    if split.len() > 3 {
        return Err(String::from("Too many arguments given"));
    }
    let arguments = match split.len() {
        1 => Vec::new(),
        _ => {
            let _ = split.pop_front();
            Vec::from(split.clone())
        }
    };
    Ok((instruction, arguments))
}

#[cfg(test)]
mod tests {
    use super::parse_instruction;
    use crate::Arg;

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
}
