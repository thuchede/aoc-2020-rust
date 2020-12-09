extern crate regex;

mod helpers;
mod day09;


fn main() {
    // println!("Day9 - 1 : {:?}", day09::day9_1());
    println!("Day9 - 2 : {:?}", day09::day9_2());
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