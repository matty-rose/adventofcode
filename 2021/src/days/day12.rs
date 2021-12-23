use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;

type Edges = HashMap<String, Vec<String>>;

#[derive(Debug)]
struct Map {
    edges: Edges,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges: Edges = HashMap::new();
        for line in s.lines() {
            let (start, end) = line.split_once("-").unwrap();

            edges
                .entry(start.to_string())
                .or_insert_with(Vec::new)
                .push(end.to_string());
            edges
                .entry(end.to_string())
                .or_insert_with(Vec::new)
                .push(start.to_string());
        }

        Ok(Map { edges })
    }
}

#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Node>,
    visited_small: HashMap<String, usize>,
}

impl Node {
    fn new(value: String, visited_small: HashMap<String, usize>) -> Self {
        Node {
            value,
            children: Vec::new(),
            visited_small,
        }
    }

    fn get_leaves(&self, end_filter: bool) -> Vec<&Node> {
        let mut leaves: Vec<&Node> = Vec::new();
        if self.children.is_empty() {
            leaves.push(self);
            return leaves;
        }

        for child in &self.children {
            leaves.append(&mut child.get_leaves(false));
        }

        return match end_filter {
            false => leaves,
            true => leaves.into_iter().filter(|p| p.value == "end").collect(),
        };
    }
}

impl Map {
    fn populate(&self, node: &mut Node, two_visit_small: bool) {
        for dest in &self.edges[&node.value] {
            // Can't go back to start
            if dest == "start" {
                continue;
            }

            let mut vsmall = node.visited_small.clone();

            match two_visit_small {
                false => {
                    // If small cave has been visited, skip
                    if vsmall.contains_key(dest) {
                        continue;
                    }

                    // Check whether cave is lowercase, and if it is, add to visited smalls
                    if dest == &dest.to_lowercase() {
                        vsmall.insert(dest.to_string(), 1);
                    }
                }
                true => {
                    let is_lower = dest == &dest.to_lowercase();

                    // if dest is already in visited, and some small has already been visited twice
                    // then skip
                    if vsmall.values().any(|&v| v == 2) && vsmall.contains_key(dest) && is_lower {
                        continue;
                    }

                    // Increment with default insert if key doesn't exist (allows a single cave to
                    // increment to 2 visits)
                    if is_lower {
                        *vsmall.entry(dest.to_string()).or_insert(0) += 1;
                    }
                }
            }

            node.children.push(Node::new(dest.to_string(), vsmall));

            if dest != "end" {
                self.populate(node.children.last_mut().unwrap(), two_visit_small);
            }
        }
    }

    fn build_tree(&self, two_visit_small: bool) -> Node {
        let mut root = Node::new("start".to_string(), HashMap::new());
        self.populate(&mut root, two_visit_small);

        root
    }
}

// P1 is start at start, and build a tree of all valid paths, then count number of leaves == end
fn part1(map: Map) {
    let tree = map.build_tree(false);
    let end_leaves = tree.get_leaves(true);
    println!("number of valid paths: {:?}", end_leaves.len());
}

fn part2(map: Map) {
    let tree = map.build_tree(true);
    let end_leaves = tree.get_leaves(true);
    println!("number of valid paths: {:?}", end_leaves.len());
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let map = Map::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(map),
        "2" => part2(map),
        _ => unreachable!(),
    }
    None
}

inventory::submit! {
    DayCommand::new("day12", Function{func: main})
}
