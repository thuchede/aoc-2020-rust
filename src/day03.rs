use std::fs::File;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day3_1() -> usize {
    let value = helpers::read(File::open("src/input/day03.txt").unwrap()).unwrap();
    let pattern: Vec<Vec<String>> = value.iter().map(|v| v.chars().map(|c| String::from(c)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    return count_tree(pattern);
}

fn count_tree(pattern: Vec<Vec<String>>) -> usize {
    let slope = 3;
    let result = pattern.iter().enumerate().filter_map(|(i, l)| if *l.get(i*slope % l.len()).unwrap() == String::from("#") { Some("#") } else { None }).count();
    return result;
}
// ____________________
// Part 2
// ____________________

pub fn day3_1_5() -> usize {
    let value = helpers::read(File::open("src/input/day03.txt").unwrap()).unwrap();
    let pattern: Vec<Vec<String>> = value.iter().map(|v| v.chars().map(|c| String::from(c)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    return count_tree_w_xslope(pattern, 1, 3);
}

pub fn day3_2_test() -> usize {
    let value = helpers::read(File::open("src/input/day03-sample.txt").unwrap()).unwrap();
    let pattern: Vec<Vec<String>> = value.iter().map(|v| v.chars().map(|c| String::from(c)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let results: Vec<usize> = slopes.iter().map(|slope| count_tree_w_xslope(pattern.clone(), slope.1, slope.0)).collect();
    results.iter().for_each(|r| println!("{}", r));

    return results.iter().fold(1, |a, b| a * b)
}

pub fn day3_2() -> usize {
    let value = helpers::read(File::open("src/input/day03.txt").unwrap()).unwrap();
    let pattern: Vec<Vec<String>> = value.iter().map(|v| v.chars().map(|c| String::from(c)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let results: Vec<usize> = slopes.iter().map(|slope| count_tree_w_xslope(pattern.clone(), slope.1, slope.0)).collect();
    results.iter().for_each(|r| println!("{}", r));
    return results.iter().fold(1, |a, b| a * b)
}

fn count_tree_w_xslope(pattern: Vec<Vec<String>>, xslope: usize, yslope: usize) -> usize {
    let result = pattern.iter().enumerate().filter_map(|(i, l)| {
        let value = if i%xslope == 0 && *l.get(i*yslope/xslope % l.len()).unwrap() == String::from("#") {
            Some("#")
        } else {
            None
        };
        return value;
    }).count();
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_1() {
        assert_eq!(280, day3_1());
    }

    #[test]
    fn test_day3_2_with_previous_sample() {
        assert_eq!(280, day3_1_5());
    }

    #[test]
    fn test_day3_2_sample() {
        let value = helpers::read(File::open("src/input/day03-sample.txt").unwrap()).unwrap();
        let pattern: Vec<Vec<String>> = value.iter().map(|v| v.chars().map(|c| String::from(c)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
        assert_eq!(2, count_tree_w_xslope(pattern.clone(), 1, 1));
        assert_eq!(7, count_tree_w_xslope(pattern.clone(), 1, 3));
        assert_eq!(3, count_tree_w_xslope(pattern.clone(), 1, 5));
        assert_eq!(4, count_tree_w_xslope(pattern.clone(), 1, 7));
        assert_eq!(2, count_tree_w_xslope(pattern.clone(), 2, 1));
    }

    #[test]
    fn test_day3_2_probsample() {
        let value = helpers::read(File::open("src/input/day03-sample.txt").unwrap()).unwrap();
        let pattern: Vec<Vec<String>> = value.iter().map(|v| v.chars().map(|c| String::from(c)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
        assert_eq!(2, count_tree_w_xslope(pattern, 2, 1));
    }

    #[test]
    fn test_day3_2_complete_sample() {
        assert_eq!(336, day3_2_test());
    }

    #[test]
    fn test_day3_2() {
        assert_eq!(4355551200, day3_2());
    }
}