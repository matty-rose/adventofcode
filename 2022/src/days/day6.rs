use std::collections::HashSet;

use crate::registry::{DayCommand, Function};
use crate::utils;

fn solve(line: &str, window_size: usize) -> Result<usize, &str> {
    let chars = line.chars().collect::<Vec<char>>();
    for (idx, win) in chars.windows(window_size).enumerate() {
        if win.iter().copied().collect::<HashSet<char>>().len() == window_size {
            return Ok(idx + window_size);
        };
    }

    Err("No solution found")
}

fn part1(line: &str) {
    let res = solve(line, 4);
    println!("Bytes processed: {}", res.unwrap());
}

fn part2(line: &str) {
    let res = solve(line, 14);
    println!("Bytes processed: {}", res.unwrap());
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    match part {
        "1" => part1(lines.first().unwrap()),
        "2" => part2(lines.first().unwrap()),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day6", Function{func: main})
}
