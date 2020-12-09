use std::fs::File;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day9_1() -> u64 {
    return run_part_1_for_file(String::from("src/input/day09.txt"), 25);
}

fn run_part_1_for_file(path: String, size: usize) -> u64 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();

    let xmas: Vec<u64> = value.iter().map(|v| v.parse::<u64>().unwrap()).collect();

    let index = find_weakness(xmas.clone(), size);

    let result = xmas.get(index).unwrap();
    return *result;
}

fn find_weakness(cypher: Vec<u64>, preamble_size: usize) -> usize {
    return find_weakness_rec(cypher, preamble_size, preamble_size);
}

fn find_weakness_rec(cypher: Vec<u64>, preamble_size: usize, start_index: usize) -> usize {
    if start_index - 1 > cypher.len() {
        return 0;
    }

    let preamble = &cypher[start_index-preamble_size..start_index];

    
    let sums = (0..preamble_size).flat_map(|i| preamble[i+1..preamble_size].iter().map(|a| a + preamble[i]).collect::<Vec<u64>>()).collect::<Vec<u64>>();
    
    let value = cypher.get(start_index).unwrap();

    if sums.contains(value) {
        return find_weakness_rec(cypher, preamble_size, start_index+1)
    } else {
        return start_index;
    }
}

// ____________________
// Part 2
// ____________________

pub fn day9_2() -> u64 {
    return run_part_2_for_file(String::from("src/input/day09.txt"), 25);
}

fn run_part_2_for_file(path: String, size: usize) -> u64 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();

    let xmas: Vec<u64> = value.iter().map(|v| v.parse::<u64>().unwrap()).collect();

    let index = find_weakness(xmas.clone(), size);

    let result = find_weakness_key(xmas, index);
    return result;
}

fn find_weakness_key(cypher: Vec<u64>, weak_key_index: usize) -> u64 {
    let cloned_cypher = cypher.clone();
    let (mut res_i, mut res_j) = (0, 0);

    for start_index in 0..weak_key_index {
        let (i, j) = find_weakness_key_slice(cypher.clone(), weak_key_index, start_index, start_index, 0);
        if i == 0 && j == 0 {
            continue;
        } else {
            res_i = i;
            res_j = j;
            break;
        }
        
    }

    let vec: Vec<u64> = (&cloned_cypher[res_i..res_j]).to_vec();
    return compute_encryption_weakness(vec);
}

fn find_weakness_key_slice_v1(cypher: Vec<u64>, weak_key_index: usize, start_index: usize, index: usize, acc: u64) -> (usize, usize) {
    if start_index >= weak_key_index {
        return (0, 0);
    }
    if index >= weak_key_index {
        return find_weakness_key_slice_v1(cypher, weak_key_index, start_index+1, start_index+1, 0);
    }
    if acc > *cypher.get(weak_key_index).unwrap() {
        return find_weakness_key_slice_v1(cypher, weak_key_index, start_index+1, start_index+1, 0)
    }

    let value = *cypher.get(index).unwrap() + acc;
    if value == *cypher.get(weak_key_index).unwrap() {
        return (start_index, index)
    }
    return find_weakness_key_slice_v1(cypher, weak_key_index, start_index, index+1, value);
}

fn find_weakness_key_slice(cypher: Vec<u64>, weak_key_index: usize, start_index: usize, index: usize, acc: u64) -> (usize, usize) {
    if start_index >= weak_key_index {
        return (0, 0);
    }
    if index >= weak_key_index {
        return (0, 0);
    }
    if acc > *cypher.get(weak_key_index).unwrap() {
        return (0, 0);
    }

    let value = *cypher.get(index).unwrap() + acc;
    if value == *cypher.get(weak_key_index).unwrap() {
        return (start_index, index)
    }
    return find_weakness_key_slice(cypher, weak_key_index, start_index, index+1, value);
}

fn compute_encryption_weakness(vec: Vec<u64>) -> u64 {
    *vec.iter().min().unwrap() + vec.iter().max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_1() {
        assert_eq!(14360655, day9_1());
    }

    #[test]
    fn test_day9_sample1() {
        assert_eq!(127, run_part_1_for_file(String::from("src/input/day09-sample.txt"), 5));
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day9_2() {
        assert_eq!(1962331, day9_2());
    }

    #[test]
    fn test_run_part_2_for_file() {
        assert_eq!(62, run_part_2_for_file(String::from("src/input/day09-sample.txt"), 5));
    }

    #[test]
    fn test_weakness_key_slice_revised() {
        assert_eq!((0, 1), find_weakness_key_slice(vec![55, 45, 100], 2, 0, 0, 0));
        assert_eq!((0, 0), find_weakness_key_slice(vec![5, 7, 2, 3, 9], 4, 0, 0, 0));
        assert_eq!((1, 2), find_weakness_key_slice(vec![5, 7, 2, 3, 9], 4, 1, 1, 0));
    }

    #[test]
    fn test_weakness_key_slicev1() {
        assert_eq!((1, 2), find_weakness_key_slice_v1(vec![5, 7, 2, 3, 9], 4, 0, 0, 0));
        assert_eq!((0, 1), find_weakness_key_slice_v1(vec![55, 45, 100], 2, 0, 0, 0));
    }

    #[test]
    fn test_compute_encryption_weakness() {
        assert_eq!(9, compute_encryption_weakness(vec![5, 7, 2, 3]));
        assert_eq!(117, compute_encryption_weakness(vec![55, 17, 100]));
    }
}