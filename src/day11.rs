use std::fs::File;
use std::convert::TryFrom;
use std::cmp;
use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn day11_1() -> u64 {
    return run_part_1_for_file(String::from("src/input/day11.txt"));
}

fn run_part_1_for_file(path: String) -> u64 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();
    let seats: Vec<Vec<char>> = value.iter().map(|l| l.chars().collect()).collect();

    let mut opt_next_seats = has_new_seats(seats);
    let mut res = 0;
    while let Some(next_seats) = opt_next_seats {
        res = count_seats(next_seats.clone());
        opt_next_seats = has_new_seats(next_seats);
    }

    res
}

fn count_seats(seats: Vec<Vec<char>>) -> u64 {
    seats.iter().flatten().fold(0, |acc, e| if *e == '#' { acc+1 } else { acc } )
}

fn has_new_seats(seats: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> /*(Vec<Vec<char>>, bool)*/ {
    let mut changed = false;
    let mut new_seats = seats.clone();
    seats.clone().iter().enumerate().for_each(|(i, row)| row.iter().enumerate().for_each(|(j, col)| {
        let next = get_next_seat(*col, adjacent_occupied_seat(seats.clone(), i, j));
        if next != *col {
            changed = true;
        }
        new_seats[i][j] = next;
    }));
    
    // (new_seats.to_vec(), changed)
    if changed {
        Some(new_seats.to_vec())
    } else {
        None
    }
}

fn get_next_seat(seat: char, adjacent_occupied_seat: usize) -> char {
    match seat {
        '#' => if adjacent_occupied_seat >= 4 { 'L' } else { '#' },
        'L' => if adjacent_occupied_seat == 0 { '#' } else { 'L' },
        _ => '.',
    }
}

fn adjacent_occupied_seat(room: Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let min_x = if x == 0 { 0 } else { x-1 };
    let min_y = if y == 0 { 0 } else { y-1 };
    let max_x = cmp::min(room.len()-1, x+1);
    let max_y = cmp::min(room[0].len()-1, y+1);
    let mut occupied_seats = 0;
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if i == x && j == y {
                continue;
            }
            if room[i][j] == '#' {
                occupied_seats+=1;
            }
        }
    }
    occupied_seats
}

// ____________________
// Part 2
// ____________________

pub fn day11_2() -> u64 {
    return run_part_2_for_file(String::from("src/input/day11.txt"));
}

fn run_part_2_for_file(path: String) -> u64 {
    let value = helpers::read(File::open(path).unwrap()).unwrap();
    let seats: Vec<Vec<char>> = value.iter().map(|l| l.chars().collect()).collect();

    let mut opt_next_seats = has_new_seats(seats);
    let mut res = 0;
    while let Some(next_seats) = opt_next_seats {
        res = count_seats(next_seats.clone());
        // println!("__________");
        // for s in next_seats.iter() {
        //     println!("{}", s.iter().collect::<String>());
        // }
        // println!("__________");
        opt_next_seats = has_new_seats_los(next_seats);
    }


    res
}

fn has_new_seats_los(seats: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> /*(Vec<Vec<char>>, bool)*/ {
    let mut changed = false;
    let mut new_seats = seats.clone();
    seats.clone().iter().enumerate().for_each(|(i, row)| row.iter().enumerate().for_each(|(j, col)| {
        let next = get_next_seat2(*col, line_of_sight_occupied_seat(seats.clone(), i, j));
        if next != *col {
            changed = true;
        }
        new_seats[i][j] = next;
    }));
    
    // (new_seats.to_vec(), changed)
    if changed {
        Some(new_seats.to_vec())
    } else {
        None
    }
}

fn get_next_seat2(seat: char, adjacent_occupied_seat: usize) -> char {
    match seat {
        '#' => if adjacent_occupied_seat > 4 { 'L' } else { '#' },
        'L' => if adjacent_occupied_seat == 0 { '#' } else { 'L' },
        _ => '.',
    }
}

fn line_of_sight_occupied_seat(room: Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let min_x = if x == 0 { 0 } else { x-1 };
    let min_y = if y == 0 { 0 } else { y-1 };
    let max_x = cmp::min(room.len()-1, x+1);
    let max_y = cmp::min(room[0].len()-1, y+1);
    let mut occupied_seats = 0;
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if i == x && j == y {
                continue;
            }
            let direction_x: isize = if i > x { 1 } else if i == x { 0 } else { -1 };
            let direction_y: isize = if j > y { 1 } else if j == y { 0 } else { -1 };
            let mut sight_i = 0;
            let mut sight_j = 0;

            // let mut iidx: isize = i + sight_i*direction_x;
            let mut iidx: isize = isize::try_from(i as isize + sight_i*direction_x).unwrap();
            let mut jidx: isize = isize::try_from(j as isize + sight_j*direction_y).unwrap();
            
            // println!("start {},{} : {}", iidx, jidx, room[iidx as usize][jidx as usize]);
            while iidx >= 0 &&
                  iidx as usize <= room.len()-1 &&
                  jidx >= 0 &&
                  jidx as usize <= room[0].len()-1 &&
                  room[iidx as usize][jidx as usize] == '.' {
                // println!("lf {},{} : {}", iidx, jidx, room[iidx as usize][jidx as usize]);
                sight_i+=1;
                sight_j+=1;
                iidx = isize::try_from(i as isize + sight_i*direction_x).unwrap();
                jidx = isize::try_from(j as isize + sight_j*direction_y).unwrap();
            }

            if iidx >= 0 &&
            iidx as usize <= room.len()-1 &&
            jidx >= 0 &&
            jidx as usize <= room[0].len()-1 &&
            room[iidx as usize][jidx as usize] == '#' {
                // println!("seat# {},{} : {}", iidx, jidx, room[iidx as usize][jidx as usize]);
                occupied_seats+=1;
            }
        }
    }
    occupied_seats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_1() {
        assert_eq!(2126, day11_1());
    }

    #[test]
    fn test_day11_sample1() {
        assert_eq!(37, run_part_1_for_file(String::from("src/input/day11-sample.txt")));
    }

    #[test]
    fn test_count_seats() {
        let empty = vec![
            vec!['L', 'L', 'L'],
            vec!['L', 'L', 'L'],
            vec!['L', 'L', 'L'],
        ];
        let full = vec![
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
        ];
        assert_eq!(0, count_seats(empty));
        assert_eq!(9, count_seats(full));
    }

    #[test]
    fn test_new_seats() {
        let start = vec![
            vec!['L', 'L', 'L'],
            vec!['L', 'L', 'L'],
            vec!['L', 'L', 'L'],
        ];
        let next = vec![
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
        ];
        let res = has_new_seats(start);
        assert_eq!(Some(next), res);
    }

    #[test]
    fn test_next_seat() {
        assert_eq!('L', get_next_seat('#', 4));
        assert_eq!('L', get_next_seat('#', 5));
        assert_eq!('#', get_next_seat('#', 3));
        assert_eq!('#', get_next_seat('L', 0));
        assert_eq!('L', get_next_seat('L', 1));
        assert_eq!('.', get_next_seat('.', 0));
        assert_eq!('.', get_next_seat('.', 8));
    }

    #[test]
    fn test_adjacent_occupied_seat() {
        let room = vec![
            vec!['#', '#', '#'],
            vec!['#', 'L', '#'],
            vec!['#', '#', '#'],
        ];
        assert_eq!(2, adjacent_occupied_seat(room.clone(), 0, 0));
        assert_eq!(8, adjacent_occupied_seat(room, 1, 1));
        let room_2 = vec![
            vec!['#', '#', '#'],
            vec!['#', 'L', '#'],
            vec!['#', '.', 'L'],
        ];
        assert_eq!(1, adjacent_occupied_seat(room_2, 2, 2));
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day11_2() {
        assert_eq!(1914, day11_2());
    }

    #[test]
    fn test_day11_sample2() {
        assert_eq!(26, run_part_2_for_file(String::from("src/input/day11-sample.txt")));
    }

    #[test]
    fn test_los_occupied_seat() {
        let room = vec![
            vec!['#', '#', '#'],
            vec!['#', 'L', '#'],
            vec!['#', '#', '#'],
        ];
        assert_eq!(2, line_of_sight_occupied_seat(room.clone(), 0, 0));
        assert_eq!(8, line_of_sight_occupied_seat(room, 1, 1));
        let room_2 = vec![
            vec!['#', '#', '#'],
            vec!['#', '.', 'L'],
            vec!['#', '.', 'L'],
        ];
        assert_eq!(2, line_of_sight_occupied_seat(room_2, 2, 2));
        let room_3 = vec![
            vec!['#', '#', '#', '.'],
            vec!['#', '.', 'L', '#'],
            vec!['#', '.', 'L', 'L'],
            vec!['#', '.', 'L', '#'],
            vec!['#', '.', 'L', '#'],
        ];
        assert_eq!(5, line_of_sight_occupied_seat(room_3.clone(), 1, 2));
        assert_eq!(3, line_of_sight_occupied_seat(room_3, 0, 1));
        let room_4 = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
        ];
        assert_eq!(0, line_of_sight_occupied_seat(room_4.clone(), 3, 3));
        assert_eq!(0, line_of_sight_occupied_seat(room_4, 0, 0));

        let no = vec![
            vec!['.','#','#','.','#','#','.',],
            vec!['#','.','#','.','#','.','#',],
            vec!['#','#','.','.','.','#','#',],
            vec!['.','.','.','L','.','.','.',],
            vec!['#','#','.','.','.','#','#',],
            vec!['#','.','#','.','#','.','#',],
            vec!['.','#','#','.','#','#','.',],
        ];
        assert_eq!(0, line_of_sight_occupied_seat(no, 3, 3));
        
        let pb = vec![
            vec!['#','.','#','#','.','#','#','.','#','#'],
            vec!['#','#','#','#','#','#','#','.','#','#'],
            vec!['#','.','#','.','#','.','.','#','.','.'],
            vec!['#','#','#','#','.','#','#','.','#','#'],
            vec!['#','.','#','#','.','#','#','.','#','#'],
            vec!['#','.','#','#','#','#','#','.','#','#'],
            vec!['.','.','#','.','#','.','.','.','.','.'],
            vec!['#','#','#','#','#','#','#','#','#','#'],
            vec!['#','.','#','#','#','#','#','#','.','#'],
            vec!['#','.','#','#','#','#','#','.','#','#'],
        ];

        assert_eq!(3, line_of_sight_occupied_seat(pb, 1, 0));
    }
}