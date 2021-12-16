use crate::registry::{DayCommand, Function};
use crate::utils;

fn part1(ps: Vec<isize>) {
    let mut least: isize = isize::MAX;
    for pos in *ps.iter().min().unwrap()..*ps.iter().max().unwrap() + 1 {
        let curr = ps.iter().map(|v| (pos - v).abs()).sum();
        if curr < least {
            least = curr;
        }
    }
    println!("Solution for p1 is current min: {}", least);
}

fn part2(ps: Vec<isize>) {
    let mut least: isize = isize::MAX;
    for pos in *ps.iter().min().unwrap()..*ps.iter().max().unwrap() + 1 {
        let curr = ps
            .iter()
            .map(|v| {
                let n = (pos - v).abs();
                n * (n + 1) / 2 // calculate the nth triangular number for fuel amount
            })
            .sum();
        if curr < least {
            least = curr;
        }
    }
    println!("Solution for p2 is current min: {}", least);
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let mut lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let positions: Vec<isize> = lines
        .pop()
        .unwrap()
        .split(',')
        .map(|p| p.parse::<isize>().unwrap())
        .collect();
    match part {
        "1" => part1(positions),
        "2" => part2(positions),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day7", Function{func: main})
}
