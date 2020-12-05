extern crate regex;

mod helpers;
// mod day01;
// mod day02;
// mod day03;
// mod day04;
mod day05;


fn main() {
    println!("Day5 - 1 : {:?}", day05::day5_1());
    println!("Day5 - 2 : {:?}", day05::day5_2());
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