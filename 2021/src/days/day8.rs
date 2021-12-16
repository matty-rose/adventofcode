use crate::registry::{DayCommand, Function};
use crate::utils;

const GOOD_LENS: [usize; 4] = [2, 3, 4, 7];

fn part1(output_digits: Vec<&str>) {
    let total: usize = output_digits
        .iter()
        .map(|o| {
            let digits: Vec<&str> = o.split_whitespace().collect();
            digits
                .iter()
                .copied()
                .filter(|d| GOOD_LENS.contains(&d.len()))
                .count()
        })
        .sum();
    println!("Solution to p1 is {:?}", total);
}

fn part2(lines: Vec<String>) {
    unimplemented!()
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let output_digits: Vec<&str> = lines
        .iter()
        .map(|l| l.split('|').last().unwrap().trim())
        .collect();
    match part {
        "1" => part1(output_digits),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day8", Function{func: main})
}
