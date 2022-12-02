use std::collections::HashMap;

use crate::registry::{DayCommand, Function};
use crate::utils;

fn choice_score(c: &str) -> i32 {
    let scores = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    *scores.get(c).unwrap()
}

#[derive(Eq, Hash, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn outcome_score(o: Outcome) -> i32 {
    let scores = HashMap::from([(Outcome::Win, 6), (Outcome::Draw, 3), (Outcome::Lose, 0)]);

    *scores.get(&o).unwrap()
}

fn part1(lines: Vec<String>) {
    let mut score = 0;
    for line in lines {
        if let Some((opponent, me)) = line.split_once(' ') {
            if (opponent == "A" && me == "X")
                || (opponent == "B" && me == "Y")
                || (opponent == "C" && me == "Z")
            {
                score += choice_score(me) + outcome_score(Outcome::Draw);
            }
            if (opponent == "A" && me == "Y")
                || (opponent == "B" && me == "Z")
                || (opponent == "C" && me == "X")
            {
                score += choice_score(me) + outcome_score(Outcome::Win);
            }
            if (opponent == "A" && me == "Z")
                || (opponent == "B" && me == "X")
                || (opponent == "C" && me == "Y")
            {
                score += choice_score(me) + outcome_score(Outcome::Lose);
            }
        }
    }
    println!("score is {score}")
}

fn part2(lines: Vec<String>) {
    let mut score = 0;
    for line in lines {
        if let Some((opponent, me)) = line.split_once(' ') {
            if me == "Y" {
                score += outcome_score(Outcome::Draw);
                if opponent == "A" {
                    score += choice_score("X")
                } else if opponent == "B" {
                    score += choice_score("Y")
                } else if opponent == "C" {
                    score += choice_score("Z")
                }
            }
            if me == "X" {
                score += outcome_score(Outcome::Lose);
                if opponent == "A" {
                    score += choice_score("Z")
                } else if opponent == "B" {
                    score += choice_score("X")
                } else if opponent == "C" {
                    score += choice_score("Y")
                }
            }
            if me == "Z" {
                score += outcome_score(Outcome::Win);
                if opponent == "A" {
                    score += choice_score("Y")
                } else if opponent == "B" {
                    score += choice_score("Z")
                } else if opponent == "C" {
                    score += choice_score("X")
                }
            }
        }
    }
    println!("score is {score}")
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
    DayCommand::new("day2", Function{func: main})
}
