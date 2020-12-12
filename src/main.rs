extern crate regex;
use std::time::{Duration, Instant};
mod helpers;
mod day12;


fn main() {
    let now = Instant::now();
    println!("Day12 - 1 : {:?}", day12::day12_1());
    println!("{}", now.elapsed().as_millis());

    let now2 = Instant::now();
    println!("Day12 - 2 : {:?}", day12::day12_2());
    println!("{}", now2.elapsed().as_millis());
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        let value = true;
        assert_eq!(true, value);
    }

    #[test]
    fn test_day1() {
        let value = false;
        assert_eq!(false, value);
    }
}