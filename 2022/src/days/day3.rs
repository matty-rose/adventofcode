use std::collections::HashSet;

use crate::registry::{DayCommand, Function};
use crate::utils;

fn char_score(c: &char) -> u32 {
    if c.is_uppercase() {
        *c as u32 - 38
    } else {
        *c as u32 - 96
    }
}

fn part1(lines: Vec<String>) {
    let mut result = 0;
    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);
        let set_one: HashSet<char> = first.chars().collect();
        let set_two: HashSet<char> = second.chars().collect();

        result += char_score(set_one.intersection(&set_two).next().unwrap());
    }
    println!("{result}");
}

fn part2(lines: Vec<String>) {
    let mut result = 0;
    for i in (0..lines.len()).step_by(3) {
        let set_one: HashSet<char> = lines[i].chars().collect();
        let set_two: HashSet<char> = lines[i + 1].chars().collect();
        let set_three: HashSet<char> = lines[i + 2].chars().collect();

        let overlap: HashSet<char> = set_one.intersection(&set_two).copied().collect();
        result += char_score(overlap.intersection(&set_three).next().unwrap());
    }
    println!("{result}");
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    match part {
        "1" => part1(lines),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day3", Function{func: main})
}
