use crate::registry::{DayCommand, Function};
use crate::utils;

fn part1(lines: Vec<String>) {
    let contains: u32 = lines
        .iter()
        .map(|l| {
            let pairs: Vec<u32> = l
                .split(',')
                .flat_map(|p| p.split('-').map(|c| c.parse::<u32>().unwrap()))
                .collect();
            u32::from(
                (pairs[2] >= pairs[0] && pairs[3] <= pairs[1])
                    || (pairs[0] >= pairs[2] && pairs[1] <= pairs[3]),
            )
        })
        .sum();
    println!("{contains:?}")
}

fn part2(lines: Vec<String>) {
    let overlaps: u32 = lines
        .iter()
        .map(|l| {
            let pairs: Vec<u32> = l
                .split(',')
                .flat_map(|p| p.split('-').map(|c| c.parse::<u32>().unwrap()))
                .collect();
            u32::from(pairs[2] <= pairs[1] && pairs[3] >= pairs[0])
        })
        .sum();
    println!("{overlaps:?}")
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    match part {
        "1" => part1(lines),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day4", Function{func: main})
}
