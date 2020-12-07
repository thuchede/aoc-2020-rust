use std::collections::{HashSet, HashMap};
use std::fs::File;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day6_1() -> usize {
    let value = helpers::read(File::open("src/input/day06.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let all_grouped_response = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x)).collect::<Vec<String>>();

    let set_of_responses: Vec<HashSet<char>> = all_grouped_response.iter().map(|responses| responses.chars().collect::<HashSet<char>>()).collect();

    let total: usize = set_of_responses.iter().map(|s| s.len()).fold(0, |acc, s| acc+s);

    return total;
}

fn day6_sample() -> usize {
    let value = helpers::read(File::open("src/input/day06-sample.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let all_grouped_response = iter.map(|x| x.iter().fold(String::from(""), |acc, x| acc + x)).collect::<Vec<String>>();

    let set_of_responses: Vec<HashSet<char>> = all_grouped_response.iter().map(|responses| responses.chars().collect::<HashSet<char>>()).collect();

    let total: usize = set_of_responses.iter().map(|s| s.len()).fold(0, |acc, s| acc+s);

    return total;
}


// ____________________
// Part 2
// ____________________

pub fn day6_2() -> usize {
    let value = helpers::read(File::open("src/input/day06.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let all_grouped_response = iter.map(|x| x.iter().fold(Customs{group_size:0, answers: String::from("")}, |mut acc, x| {acc.group_size+=1;acc.answers+=x; return acc;})).collect::<Vec<Customs>>();

    let set_of_responses: Vec<usize> = all_grouped_response.iter().map(|responses| {
        let mut hash: HashMap<char, usize> = HashMap::new();
        responses.answers.chars().for_each(|c| {
            if let Some(v) = hash.get(&c).cloned() {
                hash.insert(c, v+1);
            } else {
                hash.insert(c, 1);
            }
        });
        let size = hash.iter().filter(|&(_k, v)| *v == responses.group_size).count();
        return size;
    }).collect();

    let total: usize = set_of_responses.iter().fold(0, |acc, s| acc+s);

    return total;
}

struct Customs {
    group_size: usize,
    answers: String,
}
fn day6_s2mple() -> usize {
    let value = helpers::read(File::open("src/input/day06-sample.txt").unwrap()).unwrap();

    let iter = value.split(|v| v == &String::from(""));
    let all_grouped_response = iter.map(|x| x.iter().fold(Customs{group_size:0, answers: String::from("")}, |mut acc, x| {acc.group_size+=1;acc.answers+=x; return acc;})).collect::<Vec<Customs>>();

    let set_of_responses: Vec<usize> = all_grouped_response.iter().map(|responses| {
        let mut hash: HashMap<char, usize> = HashMap::new();
        responses.answers.chars().for_each(|c| {
            if let Some(v) = hash.get(&c).cloned() {
                hash.insert(c, v+1);
            } else {
                hash.insert(c, 1);
            }
        });
        let size = hash.iter().filter(|&(_k, v)| *v == responses.group_size).count();
        println!("{:?}", size);
        return size;
    }).collect();

    let total: usize = set_of_responses.iter().fold(0, |acc, s| acc+s);

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_1() {
        assert_eq!(0, day6_1());
    }

    #[test]
    fn test_day6_sample() {
        assert_eq!(11, day6_sample());
    }

    #[test]
    fn test_day6_2() {
        assert_eq!(6, day6_2());
    }

    #[test]
    fn test_day6_s2mple() {
        assert_eq!(6, day6_s2mple());
    }
}