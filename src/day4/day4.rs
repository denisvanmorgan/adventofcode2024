use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day4";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let word = "XMAS";
    let grid = &get_grid(lines);
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let x_char_weight = 88;
    let mut occurrences = 0;

    fn check_direction(grid: &[Vec<char>], word: &str, r: usize, c: usize, dr: isize, dc: isize) -> bool {
        let mut row = r as isize;
        let mut col = c as isize;

        for (_i, ch) in word.chars().enumerate() {
            if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
                return false;
            }

            if grid[row as usize][col as usize] != ch {
                return false;
            }

            row += dr;
            col += dc;
        }

        true
    }

    for r in 0..rows {
        for c in 0..cols {
            if (grid[r][c] as u32) != x_char_weight {
                continue;
            }

            // horizontal (left - right)
            if c + word_len <= cols {
                if check_direction(grid, word, r, c, 0, 1) {
                    occurrences += 1;
                }
            }

            // horizontal (right - left)
            if c >= word_len - 1 {
                if check_direction(grid, word, r, c, 0, -1) {
                    occurrences += 1;
                }
            }

            // vertical (top - bottom)
            if r + word_len <= rows {
                if check_direction(grid, word, r, c, 1, 0) {
                    occurrences += 1;
                }
            }

            // vertical (bottom - top)
            if r >= word_len - 1 {
                if check_direction(grid, word, r, c, -1, 0) {
                    occurrences += 1;
                }
            }

            // diagonal (topleft - bottomright)
            if r + word_len <= rows && c + word_len <= cols {
                if check_direction(grid, word, r, c, 1, 1) {
                    occurrences += 1;
                }
            }

            // diagonal (bottomleft - topright)
            if r >= word_len - 1 && c + word_len <= cols {
                if check_direction(grid, word, r, c, -1, 1) {
                    occurrences += 1;
                }
            }

            // diagonal (topright - bottomleft)
            if r + word_len <= rows && c >= word_len - 1 {
                if check_direction(grid, word, r, c, 1, -1) {
                    occurrences += 1;
                }
            }

            // diagonal (bottomright - topleft)
            if r >= word_len - 1 && c >= word_len - 1 {
                if check_direction(grid, word, r, c, -1, -1) {
                    occurrences += 1;
                }
            }
        }
    }

    println!("Day 4 - part 1: {}", occurrences);
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let grid = get_grid(lines);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut occurrences = 0;

    let a_char_weight = 65;
    let m_char_weight = 77;
    let s_char_weight = 83;
    let needle = a_char_weight + m_char_weight * 2 + s_char_weight * 2;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if (grid[r][c] as usize) != a_char_weight {
                continue;
            }

            let top_left = grid[r - 1][c - 1] as usize;
            let bottom_right = grid[r + 1][c + 1] as usize;
            let top_right = grid[r - 1][c + 1] as usize;
            let bottom_left = grid[r + 1][c - 1] as usize;

            if bottom_left == top_right && top_left == bottom_right {
                continue;
            }

            let count = top_left + bottom_right + top_right + bottom_left + a_char_weight;

            if count == needle {
                occurrences += 1;
            }
        }
    }

    println!("Day 4 - part 2: {}", occurrences);
}

fn get_grid(lines: &Vec<Result<String>>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.as_ref().unwrap().chars().collect()).collect()
}
