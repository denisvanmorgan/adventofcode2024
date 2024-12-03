use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};
use regex::Regex;

const EXERCISE_NAME: &str = "day3";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let mut result: u32 = 0;

    for line in lines {
        result += get_result(line.as_ref().unwrap());
    }

    println!("Day 3 - part 1: {}", result);
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let mut result: u32 = 0;
    let mut input: String = "".to_string();
    let mut collect: bool = true;

    for line in lines {
        input.push_str(&line.as_ref().unwrap().to_string());
    }

    loop {
        if collect {
            if let Some(pos) = input.find("don't()") {
                let before = &input[..pos];
                let after = &input[pos..];

                result += get_result(before);

                input = after.to_string();
                collect = false;
            } else {
                break;
            }
        } else {
            if let Some(pos) = input.find("do()") {
                let after = &input[pos..];

                input = after.to_string();
                collect = true;
            } else {
                break;
            }
        }
    }

    println!("Day 3 - part 2: {}", result);
}

fn get_result(sequence: &str) -> u32 {
    let mut result: u32 = 0;
    let regex = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    for hit in regex.captures_iter(sequence) {
        let first_number = &hit[1].parse::<u32>().unwrap();
        let second_number = &hit[2].parse::<u32>().unwrap();

        result += first_number * second_number;
    }

    result
}
