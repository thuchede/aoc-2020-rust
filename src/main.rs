extern crate regex;

mod helpers;
// mod day01;
// mod day02;
// mod day03;
mod day04;


fn main() {
    // println!("Day4 - 1 : {:?}", day04::day4_1());
    println!("Day4 - 2 : {:?}", day04::day4_2());
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