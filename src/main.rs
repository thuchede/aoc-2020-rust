extern crate regex;

mod helpers;
mod day10;


fn main() {
    // println!("Day10 - 1 : {:?}", day10::day10_1());
    println!("Day10 - 2 : {:?}", day10::day10_2());
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