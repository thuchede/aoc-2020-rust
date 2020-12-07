use std::fs::File;
use std::collections::{HashSet, HashMap};
use regex::Regex;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day7_1() -> usize {
    let value = helpers::read(File::open("src/input/day07.txt").unwrap()).unwrap();
    
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new(); 

    for rule in value.iter() {
        str_to_hash_map(&mut hashmap, rule.to_string());
    }
    
    let res = find_all_parent_to(&hashmap, String::from("shiny gold"));

    return res.len();
}

fn day7_sample1() -> usize {
    let value = helpers::read(File::open("src/input/day07-sample.txt").unwrap()).unwrap();
    
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new(); 

    for rule in value.iter() {
        str_to_hash_map(&mut hashmap, rule.to_string());
    }
    
    let res = find_all_parent_to(&hashmap, String::from("shiny gold"));

    return res.len();
}

fn find_all_parent_to(hashmap: &HashMap<String, Vec<String>>, input: String) -> HashSet<String> {
    let mut acc: HashSet<String> = HashSet::new();
    find_rec_all_parent_to(hashmap, input, &mut acc);
    return acc;
}

fn find_rec_all_parent_to(hashmap: &HashMap<String, Vec<String>>, input: String, acc: &mut HashSet<String>) {
    if let Some(parents) = hashmap.get(&input) {
        parents.iter().for_each(|p| if !acc.contains(p) {
            acc.insert(p.clone());
            find_rec_all_parent_to(hashmap, p.clone(), acc);
        });
    }
}

fn str_to_hash_map(hashmap: &mut HashMap<String, Vec<String>>, input: String) {
    let re = Regex::new(r#"(\w+ \w+) bags contain (?:(?:(\d) (\w+ \w+)) bags?(?:, |\.)|(no other bags?))(?:(?:(\d) (\w+ \w+)) bags?(?:, |\.))?(?:(?:(\d) (\w+ \w+)) bags?(?:, |\.))?(?:(?:(\d) (\w+ \w+)) bags?(?:, |\.))?"#).unwrap();
    // let s = String::from("striped black bags contain 1 plaid salmon bag, 2 plaid beige bags, 4 dotted teal bags, 2 posh chartreuse bags.");

    let caps = re.captures(&input).unwrap();
    let edge = String::from(caps.get(1).unwrap().as_str());
    if let Some(value) = caps.get(3) {
        // let vertex_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let vertex_target = String::from(value.as_str());
        if let Some(previous) = hashmap.get(&vertex_target) {
            let mut next = previous.clone();
            next.push(edge.clone());
            hashmap.insert(vertex_target, next);
        } else {
            hashmap.insert(vertex_target, vec![edge.clone()]);
        }
    }
    if None == caps.get(3) {
        // no entry
    }
    if let Some(value) = caps.get(6) {
        let vertex_target = String::from(value.as_str());
        if let Some(previous) = hashmap.get(&vertex_target) {
            let mut next = previous.clone();
            next.push(edge.clone());
            hashmap.insert(vertex_target, next);
        } else {
            hashmap.insert(vertex_target, vec![edge.clone()]);
        }
    }
    if let Some(value) = caps.get(8) {
        let vertex_target = String::from(value.as_str());
        if let Some(previous) = hashmap.get(&vertex_target) {
            let mut next = previous.clone();
            next.push(edge.clone());
            hashmap.insert(vertex_target, next);
        } else {
            hashmap.insert(vertex_target, vec![edge.clone()]);
        }
    }
    if let Some(value) = caps.get(10) {
        let vertex_target = String::from(value.as_str());
        if let Some(previous) = hashmap.get(&vertex_target) {
            let mut next = previous.clone();
            next.push(edge.clone());
            hashmap.insert(vertex_target, next);
        } else {
            hashmap.insert(vertex_target, vec![edge.clone()]);
        }
    }
}

// ____________________
// Part 2
// ____________________

pub fn day7_2() -> u32 {
    let value = helpers::read(File::open("src/input/day07.txt").unwrap()).unwrap();

    let mut hashmap: HashMap<String, Vec<(u32, String)>> = HashMap::new(); 

    for rule in value.iter() {
        str_to_hash_map_part2(&mut hashmap, rule.to_string());
    }

    let res = find_all_child_in(&mut hashmap, String::from("shiny gold"));

    return res;
}

fn day7_firstsample2() -> u32 {
    let value = helpers::read(File::open("src/input/day07-sample.txt").unwrap()).unwrap();

    let mut hashmap: HashMap<String, Vec<(u32, String)>> = HashMap::new(); 

    for rule in value.iter() {
        str_to_hash_map_part2(&mut hashmap, rule.to_string());
    }

    let res = find_all_child_in(&mut hashmap, String::from("shiny gold"));

    return res;
}

fn day7_sample2() -> u32 {
    let value = helpers::read(File::open("src/input/day07-sample2.txt").unwrap()).unwrap();

    let mut hashmap: HashMap<String, Vec<(u32, String)>> = HashMap::new(); 

    for rule in value.iter() {
        str_to_hash_map_part2(&mut hashmap, rule.to_string());
    }

    let res = find_all_child_in(&mut hashmap, String::from("shiny gold"));

    return res;
}



fn str_to_hash_map_part2(hashmap: &mut HashMap<String, Vec<(u32, String)>>, input: String) {
    let re = Regex::new(r#"(\w+ \w+) bags contain (?:(?:(\d) (\w+ \w+)) bags?(?:, |\.)|(no other bags?))(?:(?:(\d) (\w+ \w+)) bags?(?:, |\.))?(?:(?:(\d) (\w+ \w+)) bags?(?:, |\.))?(?:(?:(\d) (\w+ \w+)) bags?(?:, |\.))?"#).unwrap();

    let caps = re.captures(&input).unwrap();
    let edge = String::from(caps.get(1).unwrap().as_str());
    let mut vector: Vec<(u32, String)> = vec![];
    if let Some(value) = caps.get(3) {
        let vertex_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let vertex_target = String::from(value.as_str());
        vector.push((vertex_value, vertex_target));
    }
    if None == caps.get(3) {
        // no entry
    }
    if let Some(value) = caps.get(6) {
        let vertex_value = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
        let vertex_target = String::from(value.as_str());
        vector.push((vertex_value, vertex_target));
    }
    if let Some(value) = caps.get(8) {
        let vertex_value = caps.get(7).unwrap().as_str().parse::<u32>().unwrap();
        let vertex_target = String::from(value.as_str());
        vector.push((vertex_value, vertex_target));
    }
    if let Some(value) = caps.get(10) {
        let vertex_value = caps.get(9).unwrap().as_str().parse::<u32>().unwrap();
        let vertex_target = String::from(value.as_str());
        vector.push((vertex_value, vertex_target));
    }


    if let Some(previous) = hashmap.get(&edge) {
        let mut next = previous.clone();
        next.append(&mut vector);
        hashmap.insert(edge.clone(), next);
    } else {
        hashmap.insert(edge.clone(), vector);
    }
}

fn find_all_child_in(hashmap: &HashMap<String, Vec<(u32, String)>>, input: String) -> u32 {
    let mut sum = 0;
    if let Some(childrens) = hashmap.get(&input) {
        childrens.iter().for_each(|(k, v)| {
            sum = sum + k;
            sum = sum + (k * find_all_child_in(hashmap, v.clone()));
        });
    }
    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_day7_1() {
        assert_eq!(0, day7_1());
    }

    #[test]
    fn test_day7_sample1() {
        assert_eq!(4, day7_sample1());
    }

    #[test]
    fn test_find_parent() {
        let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
        hashmap.insert(String::from("green"), vec![String::from("blue"), String::from("red")]);
        hashmap.insert(String::from("blue"), vec![String::from("yellow"), String::from("red")]);
        hashmap.insert(String::from("yellow"), vec![]);
        hashmap.insert(String::from("red"), vec![]);
        let res = find_all_parent_to(&hashmap, String::from("red"));
        let res2 = find_all_parent_to(&hashmap, String::from("blue"));
        let res3 = find_all_parent_to(&hashmap, String::from("green"));
        println!("{:?}", res3);
        assert_eq!(0, res.len());
        assert_eq!(2, res2.len());
        assert_eq!(3, res3.len());
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day7_2() {
        assert_eq!(0, day7_1());
    }

    #[test]
    fn test_day7_first_sample2() {
        assert_eq!(32, day7_firstsample2());
    }

    #[test]
    fn test_day7_sample2() {
        assert_eq!(126, day7_sample2());
    }
}