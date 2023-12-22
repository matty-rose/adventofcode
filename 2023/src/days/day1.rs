use std::collections::HashMap;

use crate::registry::{DayCommand, Function};
use crate::utils;

fn part1(lines: Vec<String>) {
    let res: u32 = lines
        .iter()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
        .map(|s| {
            let mut r = String::new();
            r.push(s.chars().next().unwrap());
            r.push(s.chars().last().unwrap());
            r.parse::<u32>().unwrap()
        })
        .sum();

    println!("Sum is {res}");
}

fn part2(lines: Vec<String>) {
    let nums = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let res: i32 = lines
        .iter()
        .map(|l| {
            println!("{l}");
            let mut forward_pos = HashMap::new();
            for num in nums.keys() {
                if let Some(position) = l.find(num) {
                    forward_pos.insert(num, position);
                }
            }
            let min = forward_pos
                .iter()
                .min_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _v)| nums.get(*k).unwrap())
                .unwrap();

            // why does this fix overlap
            let mut backward_pos = HashMap::new();
            let reverse_line = l.chars().rev().collect::<String>();
            for num in nums.keys() {
                let reverse_num = num.chars().rev().collect::<String>();
                if let Some(position) = reverse_line.find(&reverse_num) {
                    backward_pos.insert(num, position);
                }
            }
            let max = backward_pos
                .iter()
                .min_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _v)| nums.get(*k).unwrap())
                .unwrap();

            let mut r = String::new();
            r.push_str(&min.to_string());
            r.push_str(&max.to_string());
            println!("{r}");
            r.parse::<i32>().unwrap()
        })
        .sum();

    println!("Sum is {res}");
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
    DayCommand::new("day1", Function{func: main})
}
