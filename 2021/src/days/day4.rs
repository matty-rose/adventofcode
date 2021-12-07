use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::HashMap;

const BOARD_SIDE: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Number {
    x: usize,
    y: usize,
    value: usize,
    marked: bool,
}

impl Number {
    fn set_marked(mut self) {
        self.marked = true;
    }
}

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Number>,
    x_counts: HashMap<usize, usize>,
    y_counts: HashMap<usize, usize>,
}

impl Board {
    fn new() -> Self {
        Board {
            numbers: Vec::new(),
            x_counts: HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]),
            y_counts: HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]),
        }
    }

    fn mark_number(mut self, value: usize) {
        let matched = self.numbers.iter().filter(|&v| v.value == value);
        for mut num in matched {
            num.set_marked();
            if self.x_counts.contains_key(&num.x) {
                *self.x_counts.get_mut(&num.x).unwrap() += 1
            } else {
                *self.x_counts.get_mut(&num.x).unwrap() = 1
            }

            if self.y_counts.contains_key(&num.y) {
                *self.y_counts.get_mut(&num.y).unwrap() += 1
            } else {
                *self.y_counts.get_mut(&num.y).unwrap() = 1
            }
        }
    }

    fn check_win(self) -> bool {
        if self
            .x_counts
            .values()
            .filter(|&v| v == &BOARD_SIDE)
            .collect::<Vec<&usize>>()
            .len()
            > 0
        {
            return true;
        }

        if self
            .y_counts
            .values()
            .filter(|&v| v == &BOARD_SIDE)
            .collect::<Vec<&usize>>()
            .len()
            > 0
        {
            return true;
        }

        return false;
    }

    fn unmarked_sum(self) -> usize {
        self.numbers
            .iter()
            .filter(|n| n.marked)
            .map(|n| n.value)
            .sum()
    }
}

#[derive(Debug)]
struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

fn parse(lines: &Vec<String>) -> Game {
    let numbers: Vec<usize> = lines[0]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut boards = Vec::new();

    let mut board = Board::new();
    let mut row: usize = 0;
    // Skip the first newline
    for line in &lines[2..] {
        if line == "" {
            boards.push(board);
            // Reset
            board = Board::new();
            row = 0;
        }

        for (col, num) in line
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .enumerate()
        {
            board.numbers.push(Number {
                x: col,
                y: row,
                value: num,
                marked: false,
            });
        }

        row += 1;
    }

    Game { numbers, boards }
}

fn part1(mut game: Game) {
    let numbers = game.numbers.clone();
    let boards = game.boards.clone();

    for num in numbers {
        for mut board in &boards {
            board.mark_number(num);
            if board.check_win() {
                println!("Solution to part 1 is {}", num * board.unmarked_sum());
                break;
            }
        }
    }
    println!("Marked Game: {:?}", game);
}

fn part2(bins: Vec<String>) {
    unimplemented!()
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let game = parse(&lines);
    println!("Game: {:?}", game);
    match part {
        "1" => part1(game),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day4", Function{func: main})
}
