mod day01;


fn main() {
    println!("Day1 - 1 : {}", day01::day1_1());
    println!("Day1 - 2 : {}", day01::day1_2());
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