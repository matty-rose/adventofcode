use crate::registry::{DayCommand, Function};
use crate::utils;
use pathfinding::directed::dijkstra::dijkstra;
use std::fmt::Display;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    entry_cost: usize,
    visited: bool,
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
                    visited: false,
                })
            }
        }

        Ok(Cavern { nodes })
    }
}

impl Cavern {
    fn draw(&self) {
        let max_x = self.nodes.iter().map(|p| p.x).max().unwrap();
        let max_y = self.nodes.iter().map(|p| p.y).max().unwrap();

        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                if let Some(n) = self.nodes.iter().find(|n| n.x == x && n.y == y) {
                    print!("{}", n.entry_cost);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<Node> {
        self.nodes
            .iter()
            .copied()
            .filter(|n| (n.x == x && (n.y == y + 1)) || (n.y == y && (n.x == x + 1)))
            .collect()
    }

    fn expand(&mut self) {
        let original_nodes = self.nodes.clone();
        let max_x = original_nodes.iter().map(|n| n.x).max().unwrap() + 1;
        let max_y = original_nodes.iter().map(|n| n.y).max().unwrap() + 1;

        let mut expansions = Vec::new();
        for x in 0..5 {
            for y in 0..5 {
                if x == 0 && y == 0 {
                    continue;
                }
                expansions.push((x, y));
            }
        }
        for (x_mult, y_mult) in expansions {
            let mut new_nodes: Vec<Node> = original_nodes
                .iter()
                .map(|n| Node {
                    x: n.x + x_mult * max_x,
                    y: n.y + y_mult * max_y,
                    entry_cost: {
                        let x = n.entry_cost + x_mult + y_mult;
                        if x > 9 {
                            x - 9
                        } else {
                            x
                        }
                    },
                    visited: false,
                })
                .collect();
            self.nodes.append(&mut new_nodes);
        }
    }
}

fn part1(cavern: Cavern) {
    let res = dijkstra(
        cavern.nodes.get(0).unwrap(),
        |n| {
            cavern
                .neighbours(n.x, n.y)
                .iter()
                .map(|nb| (*nb, nb.entry_cost))
                .collect::<Vec<(Node, usize)>>()
        },
        |n| *n == *cavern.nodes.last().unwrap(),
    );
    println!("cost of shortest path is {:?}", res.unwrap().1);
}

fn part2(mut cavern: Cavern) {
    // cavern.draw();
    println!("Expanding cavern");
    cavern.expand();
    let goal = *cavern.nodes.last().unwrap();
    // cavern.draw();
    println!("Solving shortest path");
    let res = dijkstra(
        cavern.nodes.get(0).unwrap(),
        |n| {
            cavern
                .neighbours(n.x, n.y)
                .iter()
                .map(|nb| (*nb, nb.entry_cost))
                .collect::<Vec<(Node, usize)>>()
        },
        |n| *n == goal,
    );
    println!("cost of shortest path is {:?}", res.unwrap().1);
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let cavern = Cavern::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(cavern),
        "2" => part2(cavern),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day15", Function{func: main})
}
