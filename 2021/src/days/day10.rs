use crate::registry::{DayCommand, Function};
use crate::utils;

const OPENERS: [char; 4] = ['(', '{', '<', '['];

fn match_pair(a: char, b: char) -> bool {
    if a == '(' && b == ')' || a == '{' && b == '}' || a == '[' && b == ']' || a == '<' && b == '>'
    {
        return true;
    }

    false
}

fn check_corrupted_line(line: &String) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if OPENERS.contains(&c) {
            stack.push(c);
            continue;
        }

        let opener = stack.pop().unwrap();
        if !match_pair(opener, c) {
            return Some(c);
        }
    }
    None
}

fn get_incomplete_line(line: &String) -> Option<Vec<char>> {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if OPENERS.contains(&c) {
            stack.push(c);
            continue;
        }

        let opener = stack.pop().unwrap();
        if !match_pair(opener, c) {
            return None;
        }
    }
    Some(stack)
}

fn get_incomplete_score(stack: &mut Vec<char>) -> usize {
    let mut score: usize = 0;
    while let Some(x) = stack.pop() {
        score *= 5;
        let add: usize = match x {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
        score += add;
    }
    score
}

fn part1(lines: Vec<String>) {
    let bad_chars: Vec<char> = lines.iter().filter_map(check_corrupted_line).collect();
    let score: usize = bad_chars
        .iter()
        .map(|b| match b {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum();
    println!("Score for p1 is {}", score);
}

fn part2(lines: Vec<String>) {
    let mut incomplete_lines: Vec<Vec<char>> =
        lines.iter().filter_map(get_incomplete_line).collect();
    let mut scores: Vec<usize> = incomplete_lines
        .iter_mut()
        .map(get_incomplete_score)
        .collect();
    scores.sort_unstable();
    let median = scores.get(((scores.len() + 1) / 2) - 1).unwrap();
    println!("middle score is {}", median);
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
    DayCommand::new("day10", Function{func: main})
}
