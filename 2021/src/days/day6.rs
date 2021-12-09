use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::HashMap;

fn part1(fish: Vec<usize>) {
    let mut f = fish.clone();
    for _day in 0..18 {
        let num_zeros = f.iter().filter(|&c| *c == 0).count();
        f = f.iter().map(|&c| if c == 0 { 6 } else { c - 1 }).collect();
        f.append(&mut vec![8 as usize; num_zeros]);
    }
    println!("Solution to p1 is {:?}", f.len());
}

fn get_zero_days(timer: usize, current_day: usize, days_left: usize) -> Vec<usize> {
    let mut resets: Vec<usize> = Vec::new();
    let mut curr = current_day + timer;
    while curr < days_left {
        resets.push(curr);
        curr += 7;
    }
    resets
}

fn part2(fish: Vec<usize>) {
    const MAX_DAYS: usize = 256;

    let mut days: HashMap<usize, usize> = HashMap::new();
    let f = fish.clone();
    let mut total_fish = f.len();
    for init_fish in f.iter() {
        let zdays = get_zero_days(*init_fish, 0, MAX_DAYS);
        for zday in zdays.iter() {
            *days.entry(*zday + 1).or_insert(0) += 1;
        }
    }
    for day in 0..MAX_DAYS + 1 {
        if !days.contains_key(&day) {
            continue;
        }

        let new_fish_count = days[&day];
        total_fish += new_fish_count;

        let new_reset_days = get_zero_days(8, day, MAX_DAYS);
        for reset_day in new_reset_days.iter() {
            *days.entry(*reset_day + 1).or_insert(0) += new_fish_count;
        }
    }

    println!("Solution to p1 is {:?}", total_fish);
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let inp: Vec<usize> = lines
        .iter()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    match part {
        "1" => part1(inp),
        "2" => part2(inp),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day6", Function{func: main})
}
