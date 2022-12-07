use std::collections::HashMap;

use crate::registry::{DayCommand, Function};
use crate::utils;

fn parse(lines: Vec<String>) -> HashMap<String, usize> {
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut curr: Vec<String> = Vec::new();
    for l in lines {
        match l.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => curr.push("/".to_string()),
            ["$", "cd", ".."] => _ = curr.pop(),
            ["$", "cd", dir] => {
                curr.push(format!("{dir}/"));
            }
            ["$", "ls"] => continue,
            ["dir", _] => continue,
            [size, _] => {
                for (idx, _) in curr.iter().enumerate() {
                    let dir = curr[..idx + 1].join("");
                    *dirs.entry(dir).or_insert(0) += size.parse::<usize>().unwrap();
                }
            }
            _ => continue,
        }
    }

    dirs
}

fn part1(lines: Vec<String>) {
    let dirs = parse(lines);
    println!(
        "total under 100000: {}",
        dirs.values().filter(|&v| v <= &100000).sum::<usize>()
    )
}

fn part2(lines: Vec<String>) {
    let dirs = parse(lines);
    let best_dir_size = dirs
        .values()
        .filter(|&v| v > &(dirs["/"] - (70_000_000 - 30_000_000)))
        .min()
        .unwrap();
    println!("size to remove: {best_dir_size}");
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
    DayCommand::new("day7", Function{func: main})
}
