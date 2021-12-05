use crate::registry::{DayCommand, Function};
use crate::utils;
use std::str::FromStr;

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    value: u32,
}

impl Instruction {
    fn new(direction: &str, value: u32) -> Self {
        let dir = Direction::from_str(direction).unwrap();
        Instruction {
            direction: dir,
            value,
        }
    }
}

fn part1(instructions: Vec<Instruction>) {
    let (mut h, mut d): (u32, u32) = (0, 0);

    for i in instructions.iter() {
        match i.direction {
            Direction::Forward => h += i.value,
            Direction::Up => d -= i.value,
            Direction::Down => d += i.value,
        }
    }

    println!("Solution to part 1 is {:?}", h * d)
}

fn part2(instructions: Vec<Instruction>) {
    let (mut h, mut d, mut a): (u32, u32, u32) = (0, 0, 0);

    for i in instructions.iter() {
        match i.direction {
            Direction::Forward => {
                h += i.value;
                d += i.value * a;
            }
            Direction::Up => a -= i.value,
            Direction::Down => a += i.value,
        }
    }

    println!("Solution to part 1 is {:?}", h * d)
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let instructions: Vec<Instruction> = utils::read_lines(filename)
        .expect("could not load lines")
        .iter()
        .map(|l| -> Instruction {
            let split: Vec<&str> = l.split(" ").collect();
            Instruction::new(split[0], split[1].parse::<u32>().unwrap())
        })
        .collect();
    match part {
        "1" => part1(instructions),
        "2" => part2(instructions),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day2", Function{func: main})
}
