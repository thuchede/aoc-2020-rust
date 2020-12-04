extern crate regex;

mod helpers;
// mod day01;
mod day02;
mod day03;


fn main() {
    // println!("Day3 - 1 : {:?}", day03::day3_1());
    println!("Day3 - 2 : {:?}", day03::day3_2());
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