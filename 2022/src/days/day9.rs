use std::collections::HashSet;

use crate::registry::{DayCommand, Function};
use crate::utils;

fn get_visited_points(start: (i32, i32), end: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut points = HashSet::new();
    points.insert(start);
    points.insert(end);
    if start.0 < end.0 && start.1 == end.1 {
        // End is directly to the right of start
        for new_x in start.0..end.0 + 1 {
            points.insert((new_x, start.1));
        }
    } else if start.0 > end.0 && start.1 == end.1 {
        // End is directly to the left of start
        for new_x in end.0..start.0 + 1 {
            points.insert((new_x, start.1));
        }
    } else if start.0 == end.0 && start.1 < end.1 {
        // End is directly above start
        for new_y in start.1..end.1 + 1 {
            points.insert((start.0, new_y));
        }
    } else if start.0 == end.0 && start.1 > end.1 {
        // End is directly below start
        for new_y in end.1..start.1 + 1 {
            points.insert((start.0, new_y));
        }
    } else if start.0 < end.0 && start.1 < end.1 {
        // End is above and to the right of start
        let (diag_x, diag_y) = (start.0 + 1, start.1 + 1);
        points.insert((diag_x, diag_y));
        if diag_x == end.0 {
            for new_y in diag_y..end.1 + 1 {
                points.insert((diag_x, new_y));
            }
        } else if diag_y == end.1 {
            for new_x in diag_x..end.0 + 1 {
                points.insert((new_x, diag_y));
            }
        }
    } else if start.0 < end.0 && start.1 > end.1 {
        // End is below and to the right of start
        let (diag_x, diag_y) = (start.0 + 1, start.1 - 1);
        points.insert((diag_x, diag_y));
        if diag_x == end.0 {
            for new_y in end.1..diag_y + 1 {
                points.insert((diag_x, new_y));
            }
        } else if diag_y == end.1 {
            for new_x in diag_x..end.0 + 1 {
                points.insert((new_x, diag_y));
            }
        }
    } else if start.0 > end.0 && start.1 < end.1 {
        // End is above and to the left of start
        let (diag_x, diag_y) = (start.0 - 1, start.1 + 1);
        points.insert((diag_x, diag_y));
        if diag_x == end.0 {
            for new_y in diag_y..end.1 + 1 {
                points.insert((diag_x, new_y));
            }
        } else if diag_y == end.1 {
            for new_x in end.0..diag_x + 1 {
                points.insert((new_x, diag_y));
            }
        }
    } else if start.0 > end.0 && start.1 > end.1 {
        // End is below and to the left of start
        let (diag_x, diag_y) = (start.0 - 1, start.1 - 1);
        points.insert((diag_x, diag_y));
        if diag_x == end.0 {
            for new_y in end.1..diag_y + 1 {
                points.insert((diag_x, new_y));
            }
        } else if diag_y == end.1 {
            for new_x in end.0..diag_x + 1 {
                points.insert((new_x, diag_y));
            }
        }
    }
    points
}

fn part1(lines: Vec<String>) {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for instruction in lines {
        match instruction.split_once(' ') {
            Some(("R", num_str)) => {
                let num = num_str.parse::<i32>().unwrap();
                head = (head.0 + num, head.1);
                let new_tail = if (head.0 - tail.0).abs() > 1 {
                    (head.0 - 1, head.1)
                } else {
                    tail
                };
                let visited = get_visited_points(tail, new_tail);
                tail_positions = tail_positions.union(&visited).copied().collect();
                tail = new_tail;
            }
            Some(("L", num_str)) => {
                let num = num_str.parse::<i32>().unwrap();
                head = (head.0 - num, head.1);
                let new_tail = if (head.0 - tail.0).abs() > 1 {
                    (head.0 + 1, head.1)
                } else {
                    tail
                };
                let visited = get_visited_points(tail, new_tail);
                tail_positions = tail_positions.union(&visited).copied().collect();
                tail = new_tail;
            }
            Some(("U", num_str)) => {
                let num = num_str.parse::<i32>().unwrap();
                head = (head.0, head.1 + num);
                let new_tail = if (head.1 - tail.1).abs() > 1 {
                    (head.0, head.1 - 1)
                } else {
                    tail
                };
                let visited = get_visited_points(tail, new_tail);
                tail_positions = tail_positions.union(&visited).copied().collect();
                tail = new_tail;
            }
            Some(("D", num_str)) => {
                let num = num_str.parse::<i32>().unwrap();
                head = (head.0, head.1 - num);
                let new_tail = if (head.1 - tail.1).abs() > 1 {
                    (head.0, head.1 + 1)
                } else {
                    tail
                };
                let visited = get_visited_points(tail, new_tail);
                tail_positions = tail_positions.union(&visited).copied().collect();
                tail = new_tail;
            }
            _ => continue,
        }
    }

    println!("Num tail positions: {}", tail_positions.len());
}

fn part2(lines: Vec<String>) {
    unimplemented!()
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
    DayCommand::new("day9", Function{func: main})
}
