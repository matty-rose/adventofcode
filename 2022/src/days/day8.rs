use crate::registry::{DayCommand, Function};
use crate::utils;

fn part1(lines: Vec<String>) {
    let trees: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visible = trees[0].len() * 2 + (trees.len() - 2) * 2;
    for (i, row) in trees.iter().take(trees.len() - 1).skip(1).enumerate() {
        for (j, tree) in row.iter().take(row.len() - 1).skip(1).enumerate() {
            let (mut ups, mut downs, mut lefts, mut rights) = (
                Vec::<u32>::new(),
                Vec::<u32>::new(),
                Vec::<u32>::new(),
                Vec::<u32>::new(),
            );
            for up in 0..i + 1 {
                ups.push(trees[up][j + 1])
            }
            for down in i + 2..trees.len() {
                downs.push(trees[down][j + 1])
            }
            for left in 0..j + 1 {
                lefts.push(trees[i + 1][left])
            }
            for right in j + 2..trees[0].len() {
                rights.push(trees[i + 1][right])
            }

            if ups.iter().all(|e| e < tree)
                || downs.iter().all(|e| e < tree)
                || lefts.iter().all(|e| e < tree)
                || rights.iter().all(|e| e < tree)
            {
                visible += 1
            }
        }
    }
    println!("NUM VISIBLE TREES: {visible:?}");
}

fn part2(lines: Vec<String>) {
    let trees: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut max_score = 0;
    for (i, row) in trees.iter().take(trees.len() - 1).skip(1).enumerate() {
        for (j, tree) in row.iter().take(row.len() - 1).skip(1).enumerate() {
            let (mut ups, mut downs, mut lefts, mut rights) = (
                Vec::<u32>::new(),
                Vec::<u32>::new(),
                Vec::<u32>::new(),
                Vec::<u32>::new(),
            );
            for up in 0..i + 1 {
                ups.push(trees[up][j + 1])
            }
            for down in i + 2..trees.len() {
                downs.push(trees[down][j + 1])
            }
            for left in 0..j + 1 {
                lefts.push(trees[i + 1][left])
            }
            for right in j + 2..trees[0].len() {
                rights.push(trees[i + 1][right])
            }

            let mut up_score = 0;
            for (idx, up) in ups.iter().rev().enumerate() {
                if idx == 0 {
                    up_score += 1;
                    if up >= tree {
                        break;
                    } else {
                        continue;
                    }
                }

                if up >= tree {
                    up_score += 1;
                    break;
                }

                up_score += 1
            }

            let mut down_score = 0;
            for (idx, down) in downs.iter().enumerate() {
                if idx == 0 {
                    down_score += 1;
                    if down >= tree {
                        break;
                    } else {
                        continue;
                    }
                }

                if down >= tree {
                    down_score += 1;
                    break;
                }

                down_score += 1
            }

            let mut left_score = 0;
            for (idx, left) in lefts.iter().rev().enumerate() {
                if idx == 0 {
                    left_score += 1;
                    if left >= tree {
                        break;
                    } else {
                        continue;
                    }
                }

                if left >= tree {
                    left_score += 1;
                    break;
                }

                left_score += 1
            }

            let mut right_score = 0;
            for (idx, right) in rights.iter().enumerate() {
                if idx == 0 {
                    right_score += 1;
                    if right >= tree {
                        break;
                    } else {
                        continue;
                    }
                }

                if right >= tree {
                    right_score += 1;
                    break;
                }

                right_score += 1
            }

            let score = up_score * down_score * left_score * right_score;
            if score > max_score {
                max_score = score
            }
        }
    }
    println!("MAX SCORE: {max_score:?}");
}

// 5
// [5, 2]

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
    DayCommand::new("day8", Function{func: main})
}
