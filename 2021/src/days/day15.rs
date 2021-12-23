use crate::registry::{DayCommand, Function};
use crate::utils;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Copy, Clone)]
struct Node {
    x: usize,
    y: usize,
    entry_cost: usize,
}

#[derive(Debug)]
struct Cavern {
    nodes: Vec<Node>,
}

impl FromStr for Cavern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();
        for (y, line) in s.lines().enumerate() {
            for (x, cost) in line.chars().enumerate() {
                nodes.push(Node {
                    x,
                    y,
                    entry_cost: cost.to_digit(10).unwrap() as usize,
                })
            }
        }

        Ok(Cavern { nodes })
    }
}

impl Cavern {
    fn neighbours(&self, x: usize, y: usize) -> Vec<Node> {
        self.nodes
            .iter()
            .copied()
            .filter(|n| {
                (n.x == x && (n.y == y.wrapping_sub(1) || n.y == y + 1))
                    || (n.y == y && (n.x == x.wrapping_sub(1) || n.x == x + 1))
            })
            .collect()
    }
}

// Just implement dijkstra - node neighbours are euclidean neighbours, "distances" are the values
// of entering a node
fn part1(cavern: Cavern) {
    println!("{:?}", cavern);
    println!("neighbours {:?}", cavern.neighbours(0, 0));
}

fn part2(lines: Vec<String>) {
    unimplemented!()
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let cavern = Cavern::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(cavern),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day15", Function{func: main})
}
