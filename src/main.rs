extern crate regex;

mod helpers;
mod day08;


fn main() {
    // println!("Day8 - 1 : {:?}", day08::day8_1());
    println!("Day8 - 2 : {:?}", day08::day8_2());
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