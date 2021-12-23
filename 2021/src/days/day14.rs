use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::HashMap;

fn get_pair_replacement_map(insertions: &[String]) -> HashMap<String, Vec<(char, String)>> {
    let mut insertion_map: HashMap<String, Vec<(char, String)>> = HashMap::new();
    for insertion in insertions {
        let (k, v) = insertion.split_once(" -> ").unwrap();
        let v_char = v.parse::<char>().unwrap();

        let mut front = k.chars().next().unwrap().to_string();
        front.push(v_char);
        let mut back = v.to_string();
        back.push(k.chars().last().unwrap());

        insertion_map
            .entry(k.to_string())
            .or_default()
            .push((v_char, front));
        insertion_map
            .entry(k.to_string())
            .or_default()
            .push((v_char, back));
    }
    insertion_map
}

fn solve(lines: Vec<String>, steps: usize) {
    let mut pair_counts: HashMap<String, usize> = HashMap::new();

    let template = lines[0].to_string();
    let template_chars = template.chars().collect::<Vec<char>>();
    let template_pairs = template_chars.windows(2);

    let mut char_counts = template.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    println!("char count: {:?}", char_counts);

    for p in template_pairs {
        *pair_counts.entry(p.iter().collect::<String>()).or_insert(0) += 1;
    }

    println!("pair count: {:?}", pair_counts);

    let pair_replace = get_pair_replacement_map(&lines[2..]);

    println!("pair replacement: {:?}", pair_replace);

    for _step in 1..steps + 1 {
        for (pair, count) in pair_counts.clone().iter_mut() {
            for (idx, (new_char, new_pair)) in pair_replace.get(pair).unwrap().iter().enumerate() {
                if idx == 0 {
                    *char_counts.entry(*new_char).or_insert(0) += *count;
                }
                *pair_counts.entry(new_pair.to_string()).or_insert(0) += *count;
            }
            *pair_counts.get_mut(pair).unwrap() -= *count;
        }
        println!(
            "Step: {}, Total character count: {}, Diff: {}",
            _step,
            char_counts.values().copied().sum::<usize>(),
            char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
        )
    }
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    match part {
        "1" => solve(lines, 10),
        "2" => solve(lines, 40),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day14", Function{func: main})
}
