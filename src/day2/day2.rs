use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day2";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let mut safe_occurences: u32 = 0;

    for line in lines {
        let parts: Vec<i32> = line
            .as_ref().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if !is_safe(&parts) {
            continue;
        }

        safe_occurences += 1;
    }

    println!("Day 2 - part 1: {}", safe_occurences);
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let mut safe_occurences: u32 = 0;

    for line in lines {
        let parts: Vec<i32> = line
            .as_ref().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&parts) {
            safe_occurences += 1;
            continue;
        }

        let mut parts_copy = parts.clone();

        for n in 0..parts.len() {
            parts_copy.remove(n);

            if is_safe(&parts_copy) {
                safe_occurences += 1;
                break;
            }

            parts_copy = parts.clone();
        }
    }

    println!("Day 2 - part 2: {}", safe_occurences);
}

fn is_safe(parts: &Vec<i32>) -> bool {
    let mut safe_occurences: u32 = 0;
    let len = parts.len();
    let mut direction_change: bool = false;

    for n in 0..len - 1 {
        let diff = parts[n] - parts[n + 1];

        if n == 0 {
            direction_change = parts[n] > parts[n + 1];
        }

        if n > 0 {
            let previous_direction = direction_change;
            direction_change = parts[n] > parts[n + 1];

            if previous_direction != direction_change {
                break;
            }
        }

        if diff == 0 || diff.abs() > 3 {
            break;
        }

        if n == len - 2 {
            safe_occurences += 1;
        }
    }

    safe_occurences == 1
}
