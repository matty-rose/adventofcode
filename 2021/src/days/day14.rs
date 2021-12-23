use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::HashMap;

fn get_insertion_map(insertions: &[String]) -> HashMap<String, String> {
    let mut insertion_map: HashMap<String, String> = HashMap::new();
    for insertion in insertions {
        let (k, v) = insertion.split_once(" -> ").unwrap();
        insertion_map.insert(k.to_string(), v.to_string());
    }
    insertion_map
}

fn get_pair_replacement_map(insertions: &[String]) -> HashMap<String, Vec<String>> {
    let mut insertion_map: HashMap<String, Vec<String>> = HashMap::new();
    for insertion in insertions {
        let (k, v) = insertion.split_once(" -> ").unwrap();

        let mut front = k.chars().next().unwrap().to_string();
        front.push(v.parse::<char>().unwrap());
        let mut back = v.to_string();
        back.push(k.chars().last().unwrap());

        insertion_map.entry(k.to_string()).or_default().push(front);
        insertion_map.entry(k.to_string()).or_default().push(back);
    }
    insertion_map
}

fn replace(template: &str, insertion_map: &HashMap<String, String>) -> String {
    let chars = template.chars().collect::<Vec<char>>();
    let pairs = chars.windows(2);
    let mut new_chars: Vec<char> = vec![chars[0]];
    for p in pairs {
        let key = p.iter().collect::<String>();
        if insertion_map.contains_key(&key) {
            new_chars.push(insertion_map[&key].parse::<char>().unwrap());
        }
        new_chars.push(*p.iter().last().unwrap());
    }
    new_chars.iter().collect::<String>()
}

fn part1(lines: Vec<String>) {
    let mut template = lines[0].to_string();
    let insertion_map = get_insertion_map(&lines[2..]);
    for _step in 0..10 {
        let new_template = replace(&template.to_string(), &insertion_map);
        template = new_template;
    }

    let char_count = template.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    println!("char count: {:?}", char_count);
    println!(
        "diff: {}",
        char_count.values().max().unwrap() - char_count.values().min().unwrap()
    );
}

fn part2(lines: Vec<String>) {
    let mut pair_counts: HashMap<String, usize> = HashMap::new();

    let template = lines[0].to_string();
    let template_chars = template.chars().collect::<Vec<char>>();
    let template_pairs = template_chars.windows(2);

    for p in template_pairs {
        *pair_counts.entry(p.iter().collect::<String>()).or_insert(0) += 1;
    }

    println!("pair count: {:?}", pair_counts);

    let pair_replace = get_pair_replacement_map(&lines[2..]);

    println!("pair replacement: {:?}", pair_replace);

    for _step in 0..10 {
        println!("step: {}, pair count: {:?}", _step, pair_counts);
        for (pair, count) in pair_counts.clone().iter_mut() {
            for new_pair in pair_replace.get(pair).unwrap() {
                println!(
                    "adding {} for new pair: {:?} from old pair: {:?}",
                    count, new_pair, pair
                );
                *pair_counts.entry(new_pair.to_string()).or_insert(0) += *count;
            }
            *pair_counts.get_mut(pair).unwrap() -= 0;
        }
    }

    println!("pair count: {:?}", pair_counts);
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
    DayCommand::new("day14", Function{func: main})
}
