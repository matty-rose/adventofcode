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
    fn draw(&self) {
        let max_x = self.dots.iter().map(|p| p.x).max().unwrap();
        let max_y = self.dots.iter().map(|p| p.y).max().unwrap();

        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                if self.dots.iter().any(|p| p.x == x && p.y == y) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn fold(&mut self) -> Option<(Paper, usize)> {
        let f = self.folds.pop_front();
        let new_folds = self.folds.clone();
        let mut overlap_count = 0;
        match f {
            Some(Fold::X(x)) => {
                let mut root_dots: Vec<Point> =
                    self.dots.iter().cloned().filter(|p| p.x < x).collect();
                let fold_dots: Vec<Point> = self.dots.iter().cloned().filter(|p| p.x > x).collect();
                let mut new_dots: Vec<Point> = Vec::new();
                for p in &fold_dots {
                    let mirror_x = x - (p.x - x);
                    if let Some(_np) = root_dots.iter().find(|fp| fp.x == mirror_x && fp.y == p.y) {
                        overlap_count += 1;
                    } else {
                        new_dots.push(Point {
                            x: mirror_x,
                            y: p.y,
                        });
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
                let mut root_dots: Vec<Point> =
                    self.dots.iter().cloned().filter(|p| p.y < y).collect();
                let fold_dots: Vec<Point> = self.dots.iter().cloned().filter(|p| p.y > y).collect();
                let mut new_dots: Vec<Point> = Vec::new();
                for p in &fold_dots {
                    let mirror_y = y - (p.y - y);
                    if let Some(_np) = self.dots.iter().find(|fp| fp.y == mirror_y && fp.x == p.x) {
                        overlap_count += 1;
                    } else {
                        new_dots.push(Point {
                            x: p.x,
                            y: mirror_y,
                        })
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

fn part2(mut paper: Paper) {
    while let Some((new_paper, _oc)) = paper.fold() {
        paper = new_paper;
    }
    paper.draw();
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
