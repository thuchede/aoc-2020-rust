extern crate regex;
use std::time::{Duration, Instant};
mod helpers;
// mod day12;
mod day11;


fn main() {
    let now = Instant::now();
    println!("Day11 - 1 : {:?}", day11::day11_1());
    println!("{}", now.elapsed().as_millis());

    let now2 = Instant::now();
    println!("Day11 - 2 : {:?}", day11::day11_2());
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