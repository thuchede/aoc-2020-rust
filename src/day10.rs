use std::fs::File;
use std::collections::HashMap;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day10_1() -> u64 {
    return run_part_1_for_file(String::from("src/input/day10.txt"));
}

fn run_part_1_for_file(path: String) -> u64 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();

    let mut adapters: Vec<u64> = value.iter().map(|v| v.parse::<u64>().unwrap()).collect();
    adapters.sort();
    let mut hashmap: HashMap<u64, Vec<u64>> = HashMap::new();
    let (last_adp, output) = adapters.iter().fold((0, &mut hashmap), |(prev_adp, acc_by_jolt_diff), adp| {
        let jolt_diff = adp-prev_adp;
        if let Some(adapter_for_joltage) = acc_by_jolt_diff.get_mut(&jolt_diff) {
            adapter_for_joltage.push(*adp);
        } else {
            acc_by_jolt_diff.insert(jolt_diff, vec![*adp]);
        }
        (*adp, acc_by_jolt_diff)
    });

    println!("last {:?}", last_adp);
    println!("adps {:?}", output);
    output.get(&1).unwrap().len() as u64 * (output.get(&3).unwrap().len() + 1) as u64
}

// ____________________
// Part 2
// ____________________

pub fn day10_2() -> u64 {
    return run_part_2_for_file(String::from("src/input/day10.txt"));
}

fn run_part_2_for_file(path: String) -> u64 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();

    let mut adapters: Vec<u64> = value.iter().map(|v| v.parse::<u64>().unwrap()).collect();
    adapters.sort();
    let mut hashmap: HashMap<u64, Vec<u64>> = HashMap::new();
    let (last_adp, output) = adapters.iter().fold((0, &mut hashmap), |(prev_adp, acc_by_jolt_diff), adp| {
        let jolt_diff = adp-prev_adp;
        if let Some(adapter_for_joltage) = acc_by_jolt_diff.get_mut(&jolt_diff) {
            adapter_for_joltage.push(*adp);
        } else {
            acc_by_jolt_diff.insert(jolt_diff, vec![*adp]);
        }
        (*adp, acc_by_jolt_diff)
    });

    let mut prev = 0;
    let mut current: Vec<u64> = vec![];

    let mut count: Vec<Vec<u64>> = output.get(&1).unwrap().iter().fold(vec![], |mut acc, el| {
        if prev + 1 == *el {
            current.push(*el);
        } else {
            acc.push(current.clone());
            current = vec![*el];
        }
        prev = *el;
        acc
    });

    count.push(current);
    
    let product = count.iter().map(|a| match a.len() {
        4 => 7,
        3 => 4,
        2 => 2,
        _ => 1,
    }).fold(1, |acc, el| acc*el);
    
    return product;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_1() {
        assert_eq!(2070, day10_1());
    }

    #[test]
    fn test_day10_sample1() {
        assert_eq!(220, run_part_1_for_file(String::from("src/input/day10-sample.txt")));
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day10_2() {
        assert_eq!(24179327893504, day10_2());
    }

    #[test]
    fn test_run_part_2_for_file() {
        assert_eq!(19208, run_part_2_for_file(String::from("src/input/day10-sample.txt")));
    }
}