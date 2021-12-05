use crate::registry::{DayCommand, Function};
use crate::utils;

fn binary_to_int(s: &String) -> isize {
    isize::from_str_radix(s, 2).unwrap()
}

fn get_common_bit(bins: &Vec<String>, position: usize) -> char {
    let count = bins.iter().fold(0, |acc, el| {
        acc + el.chars().nth(position).unwrap().to_digit(2).unwrap()
    });

    let half = bins.len() as f64 / 2 as f64;
    if count as f64 > half || count as f64 == half {
        '1'
    } else {
        '0'
    }
}

fn get_gamma(bins: &Vec<String>) -> String {
    let gamma: String = (0..5).map(|v| get_common_bit(bins, v)).collect::<String>();
    gamma
}

fn get_epsilon(bins: &Vec<String>) -> String {
    let epsilon: String = (0..5)
        .map(|v| {
            if get_common_bit(bins, v) == '0' {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();
    epsilon
}

fn part1(bins: Vec<String>) {
    let gamma = get_gamma(&bins);
    let gamma_value = binary_to_int(&gamma);
    println!("{:?}", gamma_value);

    let epsilon = get_epsilon(&bins);
    let epsilon_value = binary_to_int(&epsilon);
    println!("{:?}", epsilon_value);

    println!("Solution to part 1 is {}", gamma_value * epsilon_value)
}

fn part2(bins: Vec<String>) {
    let mut gamma_bins = bins.to_vec();
    let mut epsilon_bins = bins.to_vec();

    let total_len = gamma_bins.len();
    for idx in 0..total_len {
        let gamma = get_common_bit(&gamma_bins, idx);
        gamma_bins = gamma_bins
            .into_iter()
            .filter(|b| b.chars().nth(idx).unwrap() == gamma)
            .collect();
        if gamma_bins.len() == 1 {
            break;
        }
    }
    let gamma_value = binary_to_int(&gamma_bins[0]);
    println!("{:?}", gamma_value);

    let total_len = epsilon_bins.len();
    for idx in 0..total_len {
        let epsilon = if get_common_bit(&epsilon_bins, idx) == '0' {
            '1'
        } else {
            '0'
        };
        epsilon_bins = epsilon_bins
            .into_iter()
            .filter(|b| b.chars().nth(idx).unwrap() == epsilon)
            .collect();
        if epsilon_bins.len() == 1 {
            break;
        }
    }
    let epsilon_value = binary_to_int(&epsilon_bins[0]);
    println!("{:?}", epsilon_value);

    println!("Solution to part 2 is {}", gamma_value * epsilon_value)
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
    DayCommand::new("day3", Function{func: main})
}
