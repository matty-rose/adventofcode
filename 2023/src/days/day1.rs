use crate::registry::{DayCommand, Function};
use crate::utils;

fn part1(lines: Vec<String>) {
    todo!()
}

fn part2(lines: Vec<String>) {
    todo!()
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
    DayCommand::new("day1", Function{func: main})
}
