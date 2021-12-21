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
    visited_small: Vec<String>,
}

impl Node {
    fn new(value: String, visited_small: Vec<String>) -> Self {
        Node {
            value,
            children: Vec::new(),
            visited_small,
        }
    }

    fn get_leaves(&self) -> Vec<&Node> {
        let mut leaves: Vec<&Node> = Vec::new();
        if self.children.is_empty() {
            leaves.push(self);
            return leaves;
        }

        for child in &self.children {
            leaves.append(&mut child.get_leaves());
        }

        return leaves;
    }
}

impl Map {
    fn populate(&self, node: &mut Node) {
        for dest in &self.edges[&node.value] {
            let mut vsmall = node.visited_small.clone();

            // If small cave has been visited, skip
            if vsmall.contains(dest) {
                continue;
            }

            // Check whether cave is lowercase, and if it is, add to visited smalls
            if dest == &dest.to_lowercase() {
                vsmall.push(dest.to_string());
            }

            node.children.push(Node::new(dest.to_string(), vsmall));

            if dest != "end" {
                self.populate(node.children.last_mut().unwrap());
            }
        }
    }

    fn build_tree(&self) -> Node {
        let mut root = Node::new("start".to_string(), Vec::new());
        self.populate(&mut root);

        root
    }
}

// P1 is start at start, and build a tree of all valid paths, then count number of leaves == end
fn part1(map: Map) {
    println!("{:?}", map);
    let tree = map.build_tree();
    let leaves = tree.get_leaves();
    let end_count = leaves.iter().filter(|p| p.value == "end").count();
    println!("number of valid paths: {:?}", end_count);
}

fn part2(lines: Vec<String>) {
    unimplemented!()
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let map = Map::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(map),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day12", Function{func: main})
}
