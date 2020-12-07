extern crate regex;

mod helpers;
mod day07;


fn main() {
    println!("Day7 - 1 : {:?}", day07::day7_1());
    // println!("Day7 - 2 : {:?}", day07::day7_2());
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