use std::fs::File;
use regex::Regex;
use std::str::FromStr;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day8_1() -> i32 {
    return run_part_1_for_file(String::from("src/input/day08.txt"));
}

fn run_part_1_for_file(path: String) -> i32 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();

    println!("{:?}", value);
    let instructions: Vec<Instruction> = value.iter().map(|i| Instruction::from_str(&i).unwrap()).collect();
    println!("{:?}", instructions);
    return find_last_acc_value_before_loop(instructions);
}

fn find_last_acc_value_before_loop(instr: Vec<Instruction>) -> i32 {
    let mut monitored_instr: Vec<(Instruction, bool)> = instr.iter().map(|i| (i.clone(), false)).collect();

    return find_last_acc_value_before_loop_rec(&mut monitored_instr, 0, 0);
}

fn find_last_acc_value_before_loop_rec(instructions: &mut Vec<(Instruction, bool)>, acc: i32, index: usize) -> i32 {

    let (_, read_flag) = instructions.get_mut(index).unwrap();
    if *read_flag {
        return acc;
    } else {
        *read_flag = true;
    }
    let (instruction, _) = instructions.get(index).unwrap();
    let res = match instruction {
        Instruction::Nop(_) => find_last_acc_value_before_loop_rec(instructions, acc, index + 1),
        Instruction::Jmp(v) => find_last_acc_value_before_loop_rec(instructions, acc, (index as i32 + *v) as usize),
        Instruction::Acc(v) => find_last_acc_value_before_loop_rec(instructions, acc + *v, index + 1),
    };

    return res;
}

impl FromStr for Instruction {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Instruction, ()> {
        let inst_regex = Regex::new(r#"(?:acc ([-+]\d+))|(?:jmp ([-+]\d+))|(?:nop ([-+]\d+))$"#).unwrap();
        let caps = inst_regex.captures(s).unwrap();
        if let Some(int_to_parse) = caps.get(1) {
            return Ok(Instruction::Acc(int_to_parse.as_str().parse::<i32>().unwrap()));
        }
        if let Some(int_to_parse) = caps.get(2) {
            return Ok(Instruction::Jmp(int_to_parse.as_str().parse::<i32>().unwrap()));
        }
        if let Some(int_to_parse) = caps.get(3) {
            return Ok(Instruction::Nop(int_to_parse.as_str().parse::<i32>().unwrap()));
        }
        return Err(());
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        match self {
            Instruction::Nop(v) => Instruction::Nop(*v),
            Instruction::Jmp(v) => Instruction::Jmp(*v),
            Instruction::Acc(v) => Instruction::Acc(*v),
        }
    }
}

#[derive(Debug, Copy)]
enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

// ____________________
// Part 2
// ____________________

pub fn day8_2() -> i32 {
    return run_part_2_for_file(String::from("src/input/day08.txt"));
}

fn run_part_2_for_file(path: String) -> i32 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();

    let instructions: Vec<Instruction> = value.iter().map(|i| Instruction::from_str(&i).unwrap()).collect();

    for (index, _) in instructions.iter().enumerate() {
        let mut new_inst = instructions.clone();
        if let Some(i) = new_inst.get_mut(index) {
            match i {
                Instruction::Nop(v) => *i = Instruction::Jmp(*v),
                Instruction::Jmp(v) => *i = Instruction::Nop(*v),
                Instruction::Acc(_) => (),
            }
        }
        if let Ok(res) = exec_until_err(new_inst) {
            return res;
        }
    }
    return 0;
}

fn exec_until_err(instr: Vec<Instruction>) -> Result<i32, i32> {
    let mut monitored_instr: Vec<(Instruction, bool)> = instr.iter().map(|i| (i.clone(), false)).collect();

    return exec_until_err_rec(&mut monitored_instr, 0, 0);
}

fn exec_until_err_rec(instructions: &mut Vec<(Instruction, bool)>, acc: i32, index: usize) -> Result<i32, i32> {
    
    if let Some((_, read_flag)) = instructions.get_mut(index) {
        if *read_flag {
            return Err(acc);
        } else {
            *read_flag = true;
        }
        let (instruction, _) = instructions.get(index).unwrap();
        let res = match instruction {
            Instruction::Nop(_) => exec_until_err_rec(instructions, acc, index + 1),
            Instruction::Jmp(v) => exec_until_err_rec(instructions, acc, (index as i32 + *v) as usize),
            Instruction::Acc(v) => exec_until_err_rec(instructions, acc + *v, index + 1),
        };
        return res;
    } else {
        return Ok(acc);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_1() {
        assert_eq!(1137, day8_1());
    }

    #[test]
    fn test_part_sample1() {
        assert_eq!(5, run_part_1_for_file(String::from("src/input/day08-sample.txt")));
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day8_2() {
        assert_eq!(1125, day8_2());
    }

    #[test]
    fn test_day8_sample2() {
        assert_eq!(8, run_part_2_for_file(String::from("src/input/day08-sample.txt")));
    }
}