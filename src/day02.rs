#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::result::Result;
use regex::Regex;
use fancy_regex::Regex as FRegex;

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

// ____________________
// Part 1
// ____________________

pub fn day2_1() -> usize {
    let value = read(File::open("src/input/day02.txt").unwrap()).unwrap();
    let result = count_valid_password(value);
    return result;
}

struct PasswordPolicy {
    low: i32,
    high: i32,
    letter: String,
}

fn parse(line: String) -> (PasswordPolicy, String) {
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    let groups = re.captures(&line).unwrap();
    
    let policy_low = groups.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let policy_high = groups.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let policy_letter = String::from(groups.get(3).unwrap().as_str());
    let password = String::from(groups.get(4).unwrap().as_str());
    let policy = PasswordPolicy {
        low: policy_low,
        high: policy_high,
        letter: policy_letter,
    };
    (policy, password)
}

fn test_line_fopt((policy, password): (PasswordPolicy, String)) -> Option<(PasswordPolicy, String)> {
    let re = FRegex::new(format!(r#"^(((?!{}).)*?{}((?!{}).)*?){{{},{}}}$"#, policy.letter, policy.letter, policy.letter, policy.low, policy.high).as_str()).unwrap();
    if let Ok(res) = re.is_match(&password) {
        if res {
            return Some((policy, password));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn count_valid_password(list: Vec<String>) -> usize {
    let valid = list.iter().filter_map(|l| test_line_fopt(parse(l.to_string()))).count();
    return valid;
}

// ____________________
// Part 2
// ____________________

pub fn day2_2() -> usize {
    let value = read(File::open("src/input/day02.txt").unwrap()).unwrap();
    let result = count_valid_password_2(value);
    return result;
}

fn count_valid_password_2(list: Vec<String>) -> usize {
    let valid = list.iter().filter(|l| test_line_new_policy(parse(l.to_string()))).count();
    return valid;
}

fn test_line_new_policy((policy, password): (PasswordPolicy, String)) -> bool {
    let first = password.chars().nth(policy.low as usize - 1);
    let second = password.chars().nth(policy.high as usize - 1);
    let mut match_f_value: bool = false;
    let mut match_s_value: bool = false;
    if let Some(fvalue) = first {
        // match_f_value = String::from(fvalue) == policy.letter
        match_f_value = fvalue == *policy.letter.chars().collect::<Vec<char>>().get(0).unwrap()
    }
    if let Some(svalue) = second {
        // match_s_value = String::from(svalue) == policy.letter
        match_s_value = svalue == *policy.letter.chars().collect::<Vec<char>>().get(0).unwrap()
    }
    return match_f_value ^ match_s_value
}

fn test_line_regex_new_policy_opt((policy, password): (PasswordPolicy, String)) -> Option<bool> {
    let re = FRegex::new(format!(r#"((?!^.{{{}}}{})^.{{{}}}{})|((?!^.{{{}}}{})^.{{{}}}{})"#, policy.low-1, policy.letter, policy.high-1, policy.letter, policy.high-1, policy.letter, policy.low-1, policy.letter, ).as_str()).unwrap();
    if let Ok(res) = re.is_match(&password) {
        if res {
            return Some(true);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn count_valid_password_regex_new_policy(list: Vec<String>) -> usize {
    let valid = list.iter().filter_map(|l| test_line_regex_new_policy_opt(parse(l.to_string()))).count();
    return valid;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day2_1_parse_one_line() {
        let (policy, password) = parse(String::from("1-15 f: fffffff"));
        assert_eq!(1, policy.low);
        assert_eq!(15, policy.high);
        assert_eq!("f", policy.letter);
        assert_eq!("fffffff", password);
    }

    #[test]
    fn test_day2_1_test_line_fopt_that_fail() {
        let (policy, password) = parse(String::from("1-15 f: aaaaa"));
        let expect_fail = test_line_fopt((policy, password));
        assert!(expect_fail.is_none());
    }

    #[test]
    fn test_day2_1_test_line_fopt_that_pass() {
        let (policy, password) = parse(String::from("1-15 f: fffffff"));
        let expect_pass = test_line_fopt((policy, password));
        assert!(expect_pass.is_some());
    }

    #[test]
    fn test_day2_1_test_count() {
        let passwords = vec![String::from("1-15 f: aaaaa"), String::from("1-15 f: fffffff")];
        let one = count_valid_password(passwords);
        assert_eq!(1, one);
    }

    #[test]
    fn test_day2_1_test_many_count() {
        let passwords = vec![String::from("1-15 f: aaaaa"), String::from("2-3 b: bbb"), String::from("1-15 f: fffffff")];
        let two = count_valid_password(passwords);
        assert_eq!(2, two);
    }
    
    #[test]
    fn test_day2_1_test_more_count() {
        let test = vec![String::from("2-3 a: abcabcacb"), String::from("2-3 a: aaa"), String::from("2-3 a: aaaa"), String::from("2-3 a: ababbaa"), String::from("2-3 a: aa"), String::from("2-3 a: ahzzzzzza"), String::from("2-3 a: zjakzjaz"), String::from("2-3 a: ffffff")];
        let five = count_valid_password(test);
        assert_eq!(5, five);
    }
    
    #[test]
    fn test_day2_1_result() {
        let result = day2_1();
        assert_eq!(643, result);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day2_1_test_line_new_policy_that_fail() {
        let (policy, password) = parse(String::from("1-15 f: fffffffffffffffffff"));
        let expect_fail = test_line_new_policy((policy, password));
        assert_eq!(false, expect_fail);
    }

    #[test]
    fn test_day2_1_test_line_new_policy_that_maybe_should_fail_but_somehow_pass_because_out_of_bound() {
        let (policy, password) = parse(String::from("1-15 f: fffffff"));
        let expect_fail = test_line_new_policy((policy, password));
        assert_eq!(true, expect_fail);
    }
    
    #[test]
    fn test_day2_1_test_line_new_policy_that_pass() {
        let (policy, password) = parse(String::from("2-5 f: afaaa"));
        let expect_pass = test_line_new_policy((policy, password));
        assert_eq!(true, expect_pass);
    }
    
    #[test]
    fn test_day2_2_result() {
        let result = day2_2();
        assert_eq!(388, result);
    }
    
    #[test]
    fn test_day2_2_regex_result() {    
        let value = read(File::open("src/input/day02.txt").unwrap()).unwrap();
        let result = count_valid_password_regex_new_policy(value);
        assert_eq!(388, result);
    }
}