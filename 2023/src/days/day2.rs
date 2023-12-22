use std::collections::HashMap;
use std::str::FromStr;

use crate::registry::{DayCommand, Function};
use crate::utils;

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    colors: HashMap<String, Vec<i32>>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, observations) = s.split_once(':').unwrap();

        let id = game_id
            .split_whitespace()
            .last()
            .expect("there should be a space between 'Game' and the id")
            .parse::<i32>()
            .unwrap();

        let mut colors: HashMap<String, Vec<i32>> = HashMap::new();
        let observations = observations.split(';').collect::<Vec<&str>>();

        for o in observations {
            let entries = o
                .trim()
                .split(',')
                .map(|e| e.trim().split_once(' ').unwrap())
                .collect::<Vec<(&str, &str)>>();

            for (count, color) in &entries {
                let color_entry = colors.entry(color.to_string()).or_default();
                color_entry.push(count.parse::<i32>().unwrap());
            }
        }

        Ok(Self { id, colors })
    }
}

impl Game {
    fn power(self) -> i32 {
        let power = self
            .colors
            .values()
            .map(|c| c.iter().max().unwrap())
            .product();
        power
    }
}

fn part1(lines: Vec<String>) {
    let games = lines
        .iter()
        .map(|l| Game::from_str(l).expect("all games should be valid"))
        .collect::<Vec<Game>>();

    for g in &games {
        println!("{:?}", g)
    }

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let possible_games: i32 = games
        .iter()
        .filter(|g| {
            (g.colors["red"].iter().max().unwrap() <= &max_red)
                && (g.colors["green"].iter().max().unwrap() <= &max_green)
                && (g.colors["blue"].iter().max().unwrap() <= &max_blue)
        })
        .map(|g| g.id)
        .sum();

    println!("Sum of possible game ids: {possible_games}");
}

fn part2(lines: Vec<String>) {
    let games = lines
        .iter()
        .map(|l| Game::from_str(l).expect("all games should be valid"))
        .collect::<Vec<Game>>();

    let power_sum = games.iter().cloned().map(|g| g.power()).sum::<i32>();
    println!("{:?}", power_sum)
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
