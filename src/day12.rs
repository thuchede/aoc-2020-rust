use std::fs::File;
use std::convert::TryFrom;
use std::cmp;
use regex::Regex;
use std::str::FromStr;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day12_1() -> isize {
    return run_part_1_for_file(String::from("src/input/day12.txt"));
}

fn run_part_1_for_file(path: String) -> isize {
    let value = helpers::read(File::open(path).unwrap()).unwrap();
    let mut coord: Coordinates = Coordinates{
        facing: Direction::East(0),
        pos_x: 0,
        pos_y: 0,
    };
    value.iter().for_each(|l| apply_action_to_coord(&mut coord, Direction::from_str(l).unwrap()));
    println!("{:?}", coord);
    coord.pos_x.abs() + coord.pos_y.abs()
}

fn apply_action_to_coord(coord: &mut Coordinates, action: Direction) {
    match action {
        Direction::Forward(v) => {
            move_forward(coord, v as isize);
        },
        Direction::Right(v) => {
            coord.facing = turn_right(coord.facing, v);
        },
        Direction::Left(v) => {
            coord.facing = turn_left(coord.facing, v);
        },
        _ => {
            move_dir(coord, action);
        }
    }
}

fn move_dir(coord: &mut Coordinates, dir: Direction) {
    match dir {
        Direction::North(v) => coord.pos_y += v as isize, 
        Direction::South(v) => coord.pos_y -= v as isize, 
        Direction::East(v) => coord.pos_x += v as isize, 
        Direction::West(v) => coord.pos_x -= v as isize, 
        _ => (), 
    }
    ()
}

fn move_forward(coord: &mut Coordinates, value: isize) {
    match coord.facing {
        Direction::North(_) => coord.pos_y += value, 
        Direction::South(_) => coord.pos_y -= value, 
        Direction::East(_) => coord.pos_x += value, 
        Direction::West(_) => coord.pos_x -= value, 
        _ => (), 
    }
}

fn turn_right(dir: Direction, rotation: usize) -> Direction {
    let compass: Vec<Direction> = vec![Direction::North(0), Direction::East(0), Direction::South(0), Direction::West(0)];

    let index = compass.iter().position(|&r| r == dir).unwrap();
    let next = rotation / 90;

    compass[(index+next) % 4]
}

fn turn_left(dir: Direction, rotation: usize) -> Direction {
    let compass: Vec<Direction> = vec![Direction::North(0), Direction::West(0), Direction::South(0), Direction::East(0)];

    let index = compass.iter().position(|&r| r == dir).unwrap();
    let next = rotation / 90;

    compass[(index+next) % 4]
}


impl FromStr for Direction {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Direction, ()> {
        let inst_regex = Regex::new(r#"([NEWSLFR])(\d+)$"#).unwrap();
        let caps = inst_regex.captures(s).unwrap();
        if let Some(action) = caps.get(1) {
            let value = caps.get(2).unwrap();
            return match action.as_str() {
                "N" => Ok(Direction::North(value.as_str().parse::<usize>().unwrap())),
                "E" => Ok(Direction::East(value.as_str().parse::<usize>().unwrap())),
                "W" => Ok(Direction::West(value.as_str().parse::<usize>().unwrap())),
                "S" => Ok(Direction::South(value.as_str().parse::<usize>().unwrap())),
                "L" => Ok(Direction::Left(value.as_str().parse::<usize>().unwrap())),
                "F" => Ok(Direction::Forward(value.as_str().parse::<usize>().unwrap())),
                "R" => Ok(Direction::Right(value.as_str().parse::<usize>().unwrap())),
                _ => Ok(Direction::Forward(0)),
            };
        }
        return Err(());
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::North(v) => Direction::North(*v),
            Direction::South(v) => Direction::South(*v),
            Direction::West(v) => Direction::West(*v),
            Direction::East(v) => Direction::East(*v),
            Direction::Forward(v) => Direction::Forward(*v),
            Direction::Right(v) => Direction::Right(*v),
            Direction::Left(v) => Direction::Left(*v),
        }
    }
}

#[derive(Debug, Copy, PartialEq)]
enum Direction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Forward(usize),
    Right(usize),
    Left(usize),
}

#[derive(Debug)]
struct Coordinates {
    facing: Direction,
    pos_x: isize,
    pos_y: isize,
}

// ____________________
// Part 2
// ____________________

#[derive(Debug)]
struct Coordinates2 {
    facing: Direction,
    pos_x: isize,
    pos_y: isize,
    wp: (isize, isize),
}

fn variant_eq(a: &Direction, b: &Direction) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn day12_2() -> isize {
    return run_part_2_for_file(String::from("src/input/day12.txt"));
}


fn run_part_2_for_file(path: String) -> isize {
    let value = helpers::read(File::open(path).unwrap()).unwrap();
    let mut coord: Coordinates2 = Coordinates2 {
        facing: Direction::East(0),
        pos_x: 0,
        pos_y: 0,
        wp: (10, 1),
    };
    value.iter().for_each(|l| apply_action_to_coord_or_wp(&mut coord, Direction::from_str(l).unwrap()));
    println!("{:?}", coord);
    coord.pos_x.abs() + coord.pos_y.abs()
}

fn apply_action_to_coord_or_wp(coord: &mut Coordinates2, action: Direction) {
    match action {
        Direction::Forward(v) => {
            move_forward_to_wp(coord, v as isize);
        },
        Direction::Right(v) => {
            coord.wp = turn_wp_right(coord.wp, v);
        },
        Direction::Left(v) => {
            coord.wp = turn_wp_left(coord.wp, v);
        },
        _ => {
            move_wp_dir(coord, action);
        }
    }
}

fn move_wp_dir(coord: &mut Coordinates2, dir: Direction) {
    match dir {
        Direction::North(v) => coord.wp.1 += v as isize, 
        Direction::South(v) => coord.wp.1 -= v as isize, 
        Direction::East(v) => coord.wp.0 += v as isize, 
        Direction::West(v) => coord.wp.0 -= v as isize, 
        _ => (), 
    }
    ()
}

fn move_forward_to_wp(coord: &mut Coordinates2, value: isize) {
    coord.pos_x += value*coord.wp.0;
    coord.pos_y += value*coord.wp.1;
    ()
}

fn turn_wp_right((x, y): (isize, isize), rotation: usize) -> (isize, isize) {
    let next = rotation / 90;
    match next {
        3 => (-y, x),
        2 => (-x, -y),
        1 => (y, -x),
        _ => (x, y),
    }
}

fn turn_wp_left((x, y): (isize, isize), rotation: usize) -> (isize, isize) {
    let next = rotation / 90;
    match next {
        3 => (y, -x),
        2 => (-x, -y),
        1 => (-y, x),
        _ => (x, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_1() {
        assert_eq!(1133, day12_1());
    }

    #[test]
    fn test_day12_sample1() {
        assert_eq!(25, run_part_1_for_file(String::from("src/input/day12-sample.txt")));
    }

    #[test]
    fn test_turn_right() {
        let res = turn_right(Direction::East(0), 270);
        assert_eq!(Direction::North(0), res);
    }

    #[test]
    fn test_turn_left() {
        let res = turn_left(Direction::East(0), 90);
        let res2 = turn_left(Direction::West(0), 270);
        assert_eq!(Direction::North(0), res);
        assert_eq!(Direction::North(0), res2);
    }

    #[test]
    fn test_move_forward() {
        let mut coord = Coordinates {
            facing: Direction::North(0),
            pos_x: 24,
            pos_y: -12,
        };
        move_forward(&mut coord, 30);
        coord.facing = Direction::West(0);
        move_forward(&mut coord, 26);
        assert_eq!(-2, coord.pos_x);
        assert_eq!(18, coord.pos_y);
    }

    #[test]
    fn test_move_direction() {
        let mut coord = Coordinates {
            facing: Direction::South(0),
            pos_x: 24,
            pos_y: -12,
        };
        move_dir(&mut coord, Direction::North(30));
        move_dir(&mut coord, Direction::West(26));
        assert_eq!(-2, coord.pos_x);
        assert_eq!(18, coord.pos_y);
        assert_eq!(Direction::South(0), coord.facing);
    }

    #[test]
    fn test_apply_action() {
        let mut coord = Coordinates {
            facing: Direction::South(0),
            pos_x: 24,
            pos_y: -12,
        };
        apply_action_to_coord(&mut coord, Direction::North(30));
        apply_action_to_coord(&mut coord, Direction::West(26));
        assert_eq!(-2, coord.pos_x);
        assert_eq!(18, coord.pos_y);
        assert_eq!(Direction::South(0), coord.facing);
        apply_action_to_coord(&mut coord, Direction::Right(90));
        apply_action_to_coord(&mut coord, Direction::Forward(12));
        apply_action_to_coord(&mut coord, Direction::Left(180));
        apply_action_to_coord(&mut coord, Direction::South(100));
        assert_eq!(-14, coord.pos_x);
        assert_eq!(-82, coord.pos_y);
        assert_eq!(Direction::East(0), coord.facing);
    }
    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day12_2() {
        assert_eq!(61053, day12_2());
    }

    #[test]
    fn test_day12_sample2() {
        assert_eq!(286, run_part_2_for_file(String::from("src/input/day12-sample.txt")));
    }

    #[test]
    fn test_move_forward_to_wp() {
        let mut coord = Coordinates2 {
            facing: Direction::North(0),
            pos_x: 0, 
            pos_y: 0,
            wp: (10, 1),
        };
        move_forward_to_wp(&mut coord, 10);
        assert_eq!(100, coord.pos_x);
        assert_eq!(10, coord.pos_y);
        coord.wp = (10, 4);
        move_forward_to_wp(&mut coord, 7);
        assert_eq!(170, coord.pos_x);
        assert_eq!(38, coord.pos_y);
        coord.wp = (4, -10);
        move_forward_to_wp(&mut coord, 11);
        assert_eq!(214, coord.pos_x);
        assert_eq!(-72, coord.pos_y);
    }

    #[test]
    fn test_turn_wp_right() {
        let res = turn_wp_right((1, 2), 90);
        let res2 = turn_wp_right((1, 2), 180);
        assert_eq!((2, -1), res);
        assert_eq!((-1, -2), res2);
    }

    #[test]
    fn test_turn_wp_left() {
        let res = turn_wp_left((2, 1), 90);
        let res2 = turn_wp_left((2, 1), 270);
        assert_eq!((-1, 2), res);
        assert_eq!((1, -2), res2);
    }

}