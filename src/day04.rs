use std::fs::File;
use fancy_regex::Regex as FRegex;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day4_1() -> usize {
    let value = helpers::read(File::open("src/input/day04.txt").unwrap()).unwrap();
    // println!("{:?}", value)
    // let passports: Vec<Vec<String>> = value.split(|&v| v == String::from("")).into_iter().map(|pass| println!("{:?}", pass)); //.collect::<Vec<Vec<String>>>();
    // let passports = value.split(|v| v == &String::from("")).into_iter().collect::<Vec<Vec<String>>>();

    let iter = value.split(|v| v == &String::from(""));
    let passports = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x + &String::from(" "))).collect::<Vec<String>>();
    // println!("{:?}", passports);
    // return count_tree(pattern);
    let result = count_valid_passport(passports);
    return result;
}



fn test_line_fopt(passport: String) -> Option<bool> {
    let re = FRegex::new(format!(r#"(?=.*byr:)(?=.*iyr:)(?=.*eyr:)(?=.*hgt:)(?=.*hcl:)(?=.*ecl:)(?=.*pid:).*$"#).as_str()).unwrap();
    if let Ok(res) = re.is_match(&passport) {
        if res {
            return Some(res);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn count_valid_passport(list: Vec<String>) -> usize {
    let valid = list.iter().filter_map(|l| test_line_fopt(l.to_string())).count();
    return valid;
}

// ____________________
// Part 2
// ____________________

fn test_line_fopt_stronger(passport: String) -> Option<bool> {
    // let re = FRegex::new(format!(r#"(?=.*iyr:((201\d)|2020))(?=.*hcl:(#[0-9a-f]{{6}}))(?=.*byr:((?:19[2-9]\d)|200[0-2]))(?=.*pid:(\d{{9}}))(?=.*ecl:(amb|blu|brn|gry|grn|hzl|oth))(?=.*byr:(19[2-9]\d|200[0-2]))(?=.*iyr:(201\d|2020))(?=.*eyr:(202\d|2030))(?=.*hgt:((?:(?:59|6\d|7[0-6])in)|(?:(?:1[5-8]\d|19[0-3])cm))).*"#).as_str()).unwrap();
    let re = FRegex::new(format!(r#"(?=.*iyr:((201\d)|2020)(?: |$))(?=.*hcl:(#[0-9a-f]{{6}})(?: |$))(?=.*byr:((?:19[2-9]\d)|200[0-2])(?: |$))(?=.*pid:(\d{{9}}(?:\s|$)))(?=.*ecl:(amb|blu|brn|gry|grn|hzl|oth)(?: |$))(?=.*eyr:(202\d|2030)(?: |$))(?=.*hgt:((?:(?:59|6\d|7[0-6])in)|(?:(?:1[5-8]\d|19[0-3])cm))(?: |$)).*"#).as_str()).unwrap();
    if let Ok(res) = re.is_match(&passport) {
        if res {
            return Some(res);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn count_stronger_valid_passport(list: Vec<String>) -> usize {
    let valid = list.iter().filter_map(|l| test_line_fopt_stronger(l.to_string())).count();
    return valid;
}

pub fn day4_2() -> usize {
    let value = helpers::read(File::open("src/input/day04.txt").unwrap()).unwrap();
    // println!("{:?}", value)
    // let passports: Vec<Vec<String>> = value.split(|&v| v == String::from("")).into_iter().map(|pass| println!("{:?}", pass)); //.collect::<Vec<Vec<String>>>();
    // let passports = value.split(|v| v == &String::from("")).into_iter().collect::<Vec<Vec<String>>>();

    let iter = value.split(|v| v == &String::from(""));
    let passports = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x + &String::from(" "))).collect::<Vec<String>>();
    // println!("{:?}", passports);
    // return count_tree(pattern);
    let result = count_stronger_valid_passport(passports);
    return result;
}

pub fn day4_empty() -> usize {
    let value = helpers::read(File::open("src/input/day04-empty.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let passports = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x + &String::from(" "))).collect::<Vec<String>>();

    let result = count_stronger_valid_passport(passports);
    return result;
}

pub fn day4_full() -> usize {
    let value = helpers::read(File::open("src/input/day04-full.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let passports = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x + &String::from(" "))).collect::<Vec<String>>();

    let result = count_stronger_valid_passport(passports);
    return result;
}

pub fn day4_sample() -> usize {
    let value = helpers::read(File::open("src/input/day04-sample.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let passports = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x + &String::from(" "))).collect::<Vec<String>>();

    let result = count_stronger_valid_passport(passports);
    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_1() {
        assert_eq!(245, day4_1());
    }

    #[test]
    fn test_day4_2() {
        assert_eq!(133, day4_2());
    }

    #[test]
    fn test_day4_sample() {
        assert_eq!(2, day4_sample());
    }

    #[test]
    fn test_day4_emp() {
        assert_eq!(0, day4_empty());
    }

    #[test]
    fn test_day4_full() {
        assert_eq!(4, day4_full());
    }
}