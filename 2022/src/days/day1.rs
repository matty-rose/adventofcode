use crate::registry::{DayCommand, Function};
use crate::utils;

fn get_elf_sum_iter(s: &str) -> impl Iterator<Item = u32> + '_ {
    let res = s.split("\n\n").map(|l| {
        l.split('\n')
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    });
    res
}

fn part1(lines: Vec<String>) {
    let join = lines.join("\n");
    let res = get_elf_sum_iter(&join).max().unwrap();
    println!("Solution to part 1 is {res:?}")
}

fn part2(lines: Vec<String>) {
    let join = lines.join("\n");
    let mut all: Vec<u32> = get_elf_sum_iter(&join).collect::<Vec<u32>>();
    all.sort();
    let top_three_sum: u32 = all[all.len() - 3..].iter().sum();
    println!("Solution to part 2 is {top_three_sum:?}")
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
