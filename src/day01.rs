#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::result::Result;

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

// ____________________
// Part 1
// ____________________

pub fn day1_1() -> i32 {
    let value = read(File::open("src/input/day01.txt").unwrap()).unwrap();
    let res = find2020(value);
    return res;
}

pub fn day1_2() -> i32 {
    let value = read(File::open("src/input/day01.txt").unwrap()).unwrap();
    let res = recurse(value, 2020, 3);
    return res;
}

fn find2020(mut arr: Vec<i32>) -> i32 {
    if arr.len() == 0 || arr.len() == 1 {
        return 0;
    }
    let head = arr.remove(0);
    if let Some(value) = rec_finc(head, arr.clone()) {
        return value;
    } else {
        return find2020(arr);
    }
}

fn rec_finc(num: i32, arr: Vec<i32>) -> Option<i32> {
    if let Some(value) = arr.iter().find(|el| (*el + num) == 2020) {
        return Some(value*num)
    }
    return None;
}

// ____________________
// Part 2
// ____________________

// pub fn day1_2() -> u32 {
//     let value = read(File::open("src/input/day01.txt").unwrap()).unwrap();
//     let res = find_x_with_n_value(value, 2020, 3);
//     return res;
// }


fn recurse(arr: Vec<i32>, x: i32, n: i32) -> i32 {
    if arr.len() == 0 {
        return 0;
    } else {
        if let Some(value) = recurse_acc(arr, x, n, 1) {
            return value
        } else {
            return 0;
        }
    }
}

fn recurse_acc(mut arr: Vec<i32>, x: i32, n: i32, acc: i32) -> Option<i32> {
    if x == 0 && n == 0 {
        return Some(acc);
    } else if n == 0 || arr.len() == 0 {
        return None;
    } else {
        let head = arr.remove(0);
        if head > x {
            return recurse_acc(arr.clone(), x, n, acc);
        } else if let Some(value) = recurse_acc(arr.clone(), x-head, n-1, acc*head) {
            return Some(value);
        } else {
            return recurse_acc(arr.clone(), x, n, acc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day1_1_rec_finc() {
        assert_eq!(Some(4036), rec_finc(2, vec![2018]));
        assert_eq!(None, rec_finc(2, vec![2013]));
    }

    #[test]
    fn test_day1_1_zero_length_array() {
        assert_eq!(0, find2020(vec![]));
        assert_eq!(0, find2020(vec![1]));
        assert_eq!(0, find2020(vec![2020]));
    }

    #[test]
    fn test_day1_1_small_array_no_rec() {
        let value = vec![2, 2017, 2018, 2019];
        let res = find2020(value);
        assert_eq!(4036, res);
    }

    #[test]
    fn test_day1_1_small_array_rec() {
        let value = vec![2034, 2017, 3, 2019];
        let res = find2020(value);
        assert_eq!(6051, res);
    }

    #[test]
    fn test_day1_1_input() {
        let value = read(File::open("src/input/day01.txt").unwrap()).unwrap();
        let res = find2020(value);
        assert_eq!(1016131, res);
    }

    // ____________________
    // Part 2
    // ____________________
    #[test]
    fn test_day1_2_recurse_acc_find_one() {
        let value = vec![2020];
        let res = recurse_acc(value, 2020, 1, 1);
        assert_eq!(Some(2020), res);
    }

    #[test]
    fn test_day1_2_recurse_acc_find_ttwo() {
        let value = vec![1010, 1010];
        let res = recurse_acc(value, 2020, 2, 1);
        assert_eq!(Some(1020100), res);
    }

    #[test]
    fn test_day1_2_recurse_acc_find_two_in_three() {
        let value = vec![1010, 2, 1010];
        let res = recurse_acc(value, 2020, 2, 1);
        assert_eq!(Some(1020100), res);
    }

    #[test]
    fn test_day1_2_recurse_acc_sample() {
        let value = vec![1721, 979, 366, 299, 675, 1456];
        let res = recurse_acc(value, 2020, 3, 1);
        assert_eq!(Some(241861950), res);
    }

    #[test]
    fn test_day1_2_norecurse() {
        let value = vec![1721, 979, 366, 299, 675, 1456];
        let res = recurse(value, 3, 3);
        assert_eq!(0, res);
    }

    #[test]
    fn test_day1_2_recurse() {
        let value = vec![1721, 979, 366, 299, 675, 1456];
        let res = recurse(value, 2020, 3);
        assert_eq!(241861950, res);
    }

    #[test]
    fn test_day1_2_input() {
        let value = read(File::open("src/input/day01.txt").unwrap()).unwrap();
        let res = find2020(value);
        assert_eq!(1016131, res);
    }
}