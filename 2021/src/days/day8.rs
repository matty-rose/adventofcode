use crate::registry::{DayCommand, Function};
use crate::utils;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::string::ParseError;

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

#[derive(Debug)]
struct DigitMap {
    map: HashMap<usize, String>,
    outputs: Vec<String>,
}

impl FromStr for DigitMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = HashMap::<usize, String>::new();
        let mut split = s
            .split(" | ")
            .map(|s| s.split_whitespace().map(|st| st.to_string()));
        let digits: Vec<String> = split.next().unwrap().collect();
        let outputs: Vec<String> = split.next().unwrap().collect();

        // Digits we know the length of
        result.insert(
            1,
            digits
                .iter()
                .find(|d| d.len() == 2usize)
                .unwrap()
                .to_string(),
        );
        result.insert(
            4,
            digits
                .iter()
                .find(|d| d.len() == 4usize)
                .unwrap()
                .to_string(),
        );
        result.insert(
            7,
            digits
                .iter()
                .find(|d| d.len() == 3usize)
                .unwrap()
                .to_string(),
        );
        result.insert(
            8,
            digits
                .iter()
                .find(|d| d.len() == 7usize)
                .unwrap()
                .to_string(),
        );

        // digits 0, 6, 9 all have length 6, but only 6 has length 5 once subtracting the segments from 1 (the 2 length)
        let six = digits
            .iter()
            .filter(|d| d.len() == 6usize)
            .find(|d| {
                d.chars()
                    .collect::<HashSet<char>>()
                    .difference(&result[&1].chars().collect::<HashSet<char>>())
                    .count()
                    == 5
            })
            .unwrap();
        println!("six: {}", six);
        result.insert(6, six.to_string());

        // digits 2, 3, 5 all have length 5, but only 3 will g down to length 3 when subtracting segments from 1 (the 2 length)
        let three = digits
            .iter()
            .filter(|d| d.len() == 5usize)
            .find(|d| {
                d.chars()
                    .collect::<HashSet<char>>()
                    .difference(&result[&1].chars().collect::<HashSet<char>>())
                    .count()
                    == 3
            })
            .unwrap();
        println!("three: {}", three);
        result.insert(3, three.to_string());

        // set of 3 + set of 4 gives 9
        let three_set: HashSet<char> = result[&3].chars().collect();
        let four_set: HashSet<char> = result[&4].chars().collect();
        let nine_set: HashSet<char> = HashSet::from_iter(three_set.union(&four_set).copied());
        let nine = digits
            .iter()
            .find(|d| d.chars().collect::<HashSet<char>>() == nine_set)
            .unwrap();
        println!("nine: {}", nine);
        result.insert(9, nine.to_string());

        // by elimination, know 0
        let zero = digits
            .iter()
            .find(|d| d.len() == 6usize && *d != &result[&6] && *d != &result[&9])
            .unwrap();
        println!("zero: {}", zero);
        result.insert(0, zero.to_string());

        // out of 2 and 5, only 5 has length 2 when subtracting segments from 4
        let five = digits
            .iter()
            .filter(|d| d.len() == 5usize)
            .find(|d| {
                d.chars()
                    .collect::<HashSet<char>>()
                    .difference(&result[&4].chars().collect::<HashSet<char>>())
                    .count()
                    == 2
                    && *d != &result[&3]
            })
            .unwrap();
        println!("five: {}", five);
        result.insert(5, five.to_string());

        // by elimination, know 2
        let two = digits
            .iter()
            .find(|d| d.len() == 5usize && *d != &result[&3] && *d != &result[&5])
            .unwrap();
        println!("two: {}", two);
        result.insert(2, two.to_string());

        Ok(DigitMap {
            map: result,
            outputs,
        })
    }
}

impl DigitMap {
    fn find_digit_by_string(&self, value: &String) -> Option<&usize> {
        self.map.iter().find_map(|(key, val)| {
            if val.chars().collect::<HashSet<char>>() == value.chars().collect::<HashSet<char>>() {
                Some(key)
            } else {
                None
            }
        })
    }

    fn calculate_value(&self) -> usize {
        let value: usize = self
            .outputs
            .iter()
            .map(|o| self.find_digit_by_string(o).unwrap().to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        println!("{:?}", value);
        value
    }
}

fn part2(lines: Vec<String>) {
    let digits: Vec<DigitMap> = lines
        .iter()
        .map(|l| DigitMap::from_str(l).unwrap())
        .collect();
    println!("digits: {:?}", digits);
    let total: usize = digits
        .iter()
        .map(|d| d.calculate_value())
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("total: {:?}", total);
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
