use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day1";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut buffer: Vec<i32> = Vec::new();
    let length: usize;

    fill_buffers(lines, &mut left, &mut right);

    left.sort();
    right.sort();

    if left.len() == right.len() {
        length = left.len();
    } else {
        panic!("Nooo, column length does not match");
    }

    for n in 0..length {
        let diff = left[n] - right[n];
        buffer.push(diff.abs());
    }

    println!("Day 1 - part 1: {}", buffer.iter().sum::<i32>());
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut buffer: Vec<i32> = Vec::new();
    let length: usize;

    fill_buffers(lines, &mut left, &mut right);

    length = left.len();

    for n in 0..length {
        let target = left.get(n).unwrap();
        let occurences = right.iter().filter(|&x| x == target).count();
        buffer.push(target * (occurences as i32));
    }

    println!("Day 1 - part 2: {}", buffer.iter().sum::<i32>());
}

fn fill_buffers(lines: &Vec<Result<String>>, left: &mut Vec<i32>, right: &mut Vec<i32>) -> () {
    for line in lines {
        let line = line.as_ref().unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }
}
