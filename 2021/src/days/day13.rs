use crate::registry::{DayCommand, Function};
use crate::utils;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Point>,
    folds: Vec<Fold>,
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

        let folds: Vec<Fold> = foldstr
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
    fn get_overlapping_dot_count(&self, fold: &Fold) -> usize {
        match fold {
            Fold::X(x) => {
                println!("folding along x axis at {}", x);
                let mut overlap_count = 0;
                for p in self.dots.iter().filter(|p| p.x < *x) {
                    println!("point to the left of fold line {:?}", p);
                    let mirror_x = (x - p.x) + x;
                    if self
                        .dots
                        .iter()
                        .filter(|fp| fp.x == mirror_x && fp.y == p.y)
                        .count()
                        != 0
                    {
                        overlap_count += 1;
                    }
                }
                overlap_count
            }
            Fold::Y(y) => {
                println!("folding along y axis at {}", y);
                let mut overlap_count = 0;
                for p in self.dots.iter().filter(|p| p.y < *y) {
                    println!("point above the fold line {:?}", p);
                    let mirror_y = (y - p.y) + y;
                    if self
                        .dots
                        .iter()
                        .filter(|fp| fp.y == mirror_y && fp.x == p.x)
                        .count()
                        != 0
                    {
                        overlap_count += 1;
                    }
                }
                overlap_count
            }
        }
    }
}

fn part1(paper: Paper) {
    println!("{:?}", paper);
    // New count after one fold is total number of dots - number that overlap
    let first_fold = paper.folds.get(0);
    let count = paper.get_overlapping_dot_count(first_fold.unwrap());
    println!(
        "Visible dots after first fold: {}",
        paper.dots.len() - count
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
