use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::HashMap;

type Point = (usize, usize);

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn interpolate_hv_points(&self) -> Vec<Point> {
        if !((self.start.0 == self.end.0) || (self.start.1 == self.end.1)) {
            println!("not a horizontal or vertical line");
            return Vec::new();
        }

        println!("Line: {:?}", self);
        let mut pts: Vec<Point> = vec![self.start, self.end];
        let mut pt = self.start;
        if self.start.0 < self.end.0 {
            for x in (self.start.0 + 1)..self.end.0 {
                pt = (x, pt.1);
                println!("Point: {:?}", pt);
                pts.push(pt);
            }
        } else if self.start.0 > self.end.0 {
            for x in (self.end.0 + 1)..self.start.0 {
                pt = (x, pt.1);
                println!("Point: {:?}", pt);
                pts.push(pt);
            }
        } else if self.start.1 < self.end.1 {
            for y in (self.start.1 + 1)..self.end.1 {
                pt = (pt.0, y);
                println!("Point: {:?}", pt);
                pts.push(pt);
            }
        } else if self.start.1 > self.end.1 {
            for y in (self.end.1 + 1)..self.start.1 {
                pt = (pt.0, y);
                println!("Point: {:?}", pt);
                pts.push(pt);
            }
        }

        return pts;
    }
}

fn parse(lines: &Vec<String>) -> Vec<Line> {
    lines
        .iter()
        .map(|l| {
            str::replace(l, " -> ", ",")
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|v| Line {
            start: (v[0], v[1]),
            end: (v[2], v[3]),
        })
        .collect()
}

fn part1(lines: Vec<Line>) {
    let mut pts: HashMap<(usize, usize), usize> = HashMap::new();

    for l in lines.iter() {
        for p in l.interpolate_hv_points() {
            let counter = pts.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    let dangerous = pts.values().filter(|&v| *v >= 2).count();
    println!("{:?}", dangerous)
}

fn part2(lines: &Vec<Line>) {
    unimplemented!()
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let mut lns: Vec<Line> = parse(&lines);
    println!("{:?}", lns);
    match part {
        "1" => part1(lns),
        "2" => part2(&lns),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day5", Function{func: main})
}
