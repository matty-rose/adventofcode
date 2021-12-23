use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::VecDeque;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Point>,
    folds: VecDeque<Fold>,
}

impl FromStr for Paper {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dotstr, foldstr) = s.split_once("\n\n").unwrap();

        let dots: Vec<Point> = dotstr
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                Point {
                    x: x.parse::<usize>().unwrap(),
                    y: y.parse::<usize>().unwrap(),
                }
            })
            .collect();

        let folds: VecDeque<Fold> = foldstr
            .lines()
            .map(|l| {
                let (axis, value) = l.rsplit_once(" ").unwrap().1.split_once("=").unwrap();
                let value: usize = value.parse().unwrap();
                match axis {
                    "x" => Fold::X(value),
                    "y" => Fold::Y(value),
                    _ => unreachable!(),
                }
            })
            .collect();

        Ok(Paper { dots, folds })
    }
}

impl Paper {
    fn fold(&mut self) -> Option<(Paper, usize)> {
        let f = self.folds.pop_front();
        let new_folds = self.folds.clone();
        let mut overlap_count = 0;
        match f {
            Some(Fold::X(x)) => {
                println!("folding along x axis at {}", x);
                let mut root_dots: Vec<Point> =
                    self.dots.iter().cloned().filter(|p| p.x < x).collect();
                let mut new_dots: Vec<Point> = Vec::new();
                for p in &root_dots {
                    println!("point to the left of fold line {:?}", p);
                    let mirror_x = (x - p.x) + x;
                    if let Some(np) = self.dots.iter().find(|fp| fp.x == mirror_x && fp.y == p.y) {
                        overlap_count += 1;
                        new_dots.push(np.clone());
                    };
                }
                root_dots.append(&mut new_dots);
                Some((
                    Paper {
                        dots: root_dots,
                        folds: new_folds,
                    },
                    overlap_count,
                ))
            }
            Some(Fold::Y(y)) => {
                println!("folding along y axis at {}", y);
                let mut root_dots: Vec<Point> =
                    self.dots.iter().cloned().filter(|p| p.y < y).collect();
                let mut new_dots: Vec<Point> = Vec::new();
                for p in &root_dots {
                    println!("point above the fold line {:?}", p);
                    let mirror_y = (y - p.y) + y;
                    if let Some(np) = self.dots.iter().find(|fp| fp.y == mirror_y && fp.x == p.x) {
                        overlap_count += 1;
                        new_dots.push(np.clone())
                    };
                }
                root_dots.append(&mut new_dots);
                Some((
                    Paper {
                        dots: root_dots,
                        folds: new_folds,
                    },
                    overlap_count,
                ))
            }
            _ => None,
        }
    }
}

fn part1(mut paper: Paper) {
    println!("{:?}", paper);
    // New count after one fold is total number of dots - number that overlap
    let total_count = paper.dots.len();
    let (_new_paper, overlap_count) = paper.fold().unwrap();
    println!(
        "Total initial dots: {}, overlapping dots: {}, Visible dots after first fold: {}",
        total_count,
        overlap_count,
        total_count - overlap_count
    );
}

fn part2(paper: Paper) {
    unimplemented!()
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let paper = Paper::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(paper),
        "2" => part2(paper),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day13", Function{func: main})
}
