use crate::registry::{DayCommand, Function};
use crate::utils;

fn get_less(values: &Vec<u32>) -> Vec<u32> {
    let less = values[1..]
        .into_iter()
        .enumerate()
        .filter(|&(idx, v)| v < &values[idx])
        .map(|(_, v)| *v)
        .collect();

    less
}

fn part1(measurements: Vec<u32>) {
    let less = get_less(&measurements);
    println!(
        "Solution to part 1 is {:?}",
        measurements.len() - less.len()
    )
}

fn part2(measurements: Vec<u32>) {
    let mut windows: Vec<u32> = Vec::new();
    for idx in 0..measurements.len() - 2 {
        windows.push(measurements[idx..idx + 2].iter().sum())
    }

    let less = get_less(&windows);
    println!("Solution to part 2 is {:?}", windows.len() - less.len())
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<u32> = utils::read_lines(filename)
        .expect("could not load lines")
        .iter()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    match part {
        "1" => part1(lines),
        "2" => part2(lines),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day1", Function{func: main})
}
