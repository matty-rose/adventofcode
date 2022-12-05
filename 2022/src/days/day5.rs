use std::str::FromStr;
use std::string::ParseError;

use crate::registry::{DayCommand, Function};
use crate::utils;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Game {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stack_str, instructions) = s.split_once("\n\n").unwrap();

        let stack_line = stack_str.split('\n').last().unwrap();
        let num_stacks = stack_line
            .replace(' ', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .max()
            .unwrap();

        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks.try_into().unwrap()];

        for line in stack_str.split('\n').rev().skip(1) {
            let mut chars = line.chars();
            for i in 0..num_stacks {
                match chars.next_chunk::<4>() {
                    Ok(x) => {
                        if x[1] != ' ' {
                            stacks[i as usize].push(x[1])
                        }
                    }
                    Err(mut x) => {
                        let val = x.nth(1).unwrap();
                        if val != ' ' {
                            stacks[i as usize].push(val)
                        }
                    }
                }
            }
        }

        let instructions: Vec<Instruction> = instructions
            .split('\n')
            .map(|l| {
                let parts: Vec<&str> = l.split(' ').collect();
                Instruction {
                    count: parts[1].parse().unwrap(),
                    from: parts[3].parse().unwrap(),
                    to: parts[5].parse().unwrap(),
                }
            })
            .collect();

        Ok(Game {
            stacks,
            instructions,
        })
    }
}

fn part1(mut game: Game) {
    for instr in &game.instructions {
        for _c in 0..instr.count {
            let val = game.stacks[instr.from - 1].pop().unwrap();
            game.stacks[instr.to - 1].push(val)
        }
    }
    let message: String = game.stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("{message:?}");
}

fn part2(mut game: Game) {
    for instr in &game.instructions {
        let mut new_vals = Vec::new();
        for _c in 0..instr.count {
            let val = game.stacks[instr.from - 1].pop().unwrap();
            new_vals.push(val)
        }
        for val in new_vals.iter().rev() {
            game.stacks[instr.to - 1].push(*val)
        }
    }
    let message: String = game.stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("{message:?}");
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let game = Game::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(game),
        "2" => part2(game),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day5", Function{func: main})
}
