use std::fs::File;
use regex::Regex;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day5_1() -> u32 {
    let value = helpers::read(File::open("src/input/day05.txt").unwrap()).unwrap();
    let max: Option<u32> = value.iter().map(|b| compute_seat_id_from_boarding_pass(parse_boarding_pass(b.clone()))).max();
    if let Some(result) = max {
        return result;
    } else {
        return 0;
    }
}

struct BoardingPass {
    row: u32,
    col: u32,
}

fn parse_boarding_pass(s: String) -> BoardingPass {
    let re = Regex::new(r"([FB]{7})([LR]{3})").unwrap();
    let caps = re.captures(&s).unwrap();
    let row_code = caps.get(1).unwrap().as_str();
    let col_code = caps.get(2).unwrap().as_str();
    let row_string: String = row_code.chars().map(|e| match e {
        'F' => '0',
        'B' => '1',
        _ => '0',
    }).collect();
    let col_string: String = col_code.chars().map(|e| match e {
        'L' => '0',
        'R' => '1',
        _ => '0',
    }).collect();

    let row = u32::from_str_radix(row_string.as_str(), 2).unwrap();
    let col = u32::from_str_radix(col_string.as_str(), 2).unwrap();

    return BoardingPass{ row: row, col: col };
}

fn compute_seat_id_from_boarding_pass(boarding_pass: BoardingPass) -> u32 {
    return boarding_pass.row * 8 + boarding_pass.col;
}

// ____________________
// Part 2
// ____________________

pub fn day5_2() -> u32 {
    let value = helpers::read(File::open("src/input/day05.txt").unwrap()).unwrap();

    let all_seats_ids: Vec<u32> = value.iter().map(|b| compute_seat_id_from_boarding_pass(parse_boarding_pass(b.clone()))).collect();

    let mut sorted_seat = all_seats_ids.clone();
    sorted_seat.sort();
    let mut prev_seat = 0;
    for seat in sorted_seat {
        if seat == prev_seat + 2 {
            return prev_seat + 1
        } else {
            prev_seat = seat;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_1() {
        assert_eq!(848, day5_1());
    }

    #[test]
    fn test_boarding_pass_first_seat() {
        let bp = parse_boarding_pass(String::from("FFFFFFFLLL"));
        assert_eq!(0, bp.row);
        assert_eq!(0, bp.col);
    }

    #[test]
    fn test_boarding_pass_example_seat() {
        let bp = parse_boarding_pass(String::from("FBFBBFFRLR"));
        assert_eq!(44, bp.row);
        assert_eq!(5, bp.col);
    }

    #[test]
    fn test_compute_seat_id_from_boarding_pass() {
        let bp = BoardingPass{ row: 44, col: 5 };
        let seat = compute_seat_id_from_boarding_pass(bp);
        assert_eq!(357, seat);
    }

    #[test]
    fn test_day5_2() {
        assert_eq!(682, day5_2());
    }

}