use crate::registry::{DayCommand, Function};
use crate::utils;

#[derive(Debug)]
struct Cave {
    height: usize,
    width: usize,
    heightmap: Vec<Vec<u32>>,
}

impl Cave {
    fn new(raw_heightmap: Vec<String>) -> Self {
        let height = raw_heightmap.len();
        let width = raw_heightmap[0].len();
        let heightmap: Vec<Vec<u32>> = raw_heightmap
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        Cave {
            height,
            width,
            heightmap,
        }
    }

    fn check_low_point(&self, x: usize, y: usize) -> bool {
        let above = match y {
            y if y > 0 => self.heightmap[y][x] < self.heightmap[y - 1][x],
            _ => true,
        };
        let below = match y {
            y if y < self.height - 1 => self.heightmap[y][x] < self.heightmap[y + 1][x],
            _ => true,
        };
        let left = match x {
            x if x > 0 => self.heightmap[y][x] < self.heightmap[y][x - 1],
            _ => true,
        };
        let right = match x {
            x if x < self.width - 1 => self.heightmap[y][x] < self.heightmap[y][x + 1],
            _ => true,
        };
        above && below && left && right
    }

    fn get_basin(
        &self,
        x: usize,
        y: usize,
        visited: &mut Vec<(usize, usize)>,
        points: &mut Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        if self.heightmap[y][x] == 9 || x > self.width || y > self.height {
            return points.to_vec();
        } else {
            points.push((x, y));
        }

        // above
        if y > 0
            && !visited.contains(&(x, y - 1))
            && self.heightmap[y - 1][x] > self.heightmap[y][x]
        {
            visited.push((x, y - 1));
            self.get_basin(x, y - 1, visited, points);
        }
        // below
        if y < self.height - 1
            && !visited.contains(&(x, y + 1))
            && self.heightmap[y + 1][x] > self.heightmap[y][x]
        {
            visited.push((x, y + 1));
            self.get_basin(x, y + 1, visited, points);
        }
        // left
        if x > 0
            && !visited.contains(&(x - 1, y))
            && self.heightmap[y][x - 1] > self.heightmap[y][x]
        {
            visited.push((x - 1, y));
            self.get_basin(x - 1, y, visited, points);
        }
        // right
        if x < self.width - 1
            && !visited.contains(&(x + 1, y))
            && self.heightmap[y][x + 1] > self.heightmap[y][x]
        {
            visited.push((x + 1, y));
            self.get_basin(x + 1, y, visited, points);
        }

        return points.to_vec();
    }
}

fn part1(cave: Cave) {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (y, row) in cave.heightmap.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if cave.check_low_point(x, y) {
                low_points.push((x, y));
            };
        }
    }
    let risk: usize = low_points
        .iter()
        .map(|(x, y)| (cave.heightmap[*y][*x] + 1) as usize)
        .sum();
    println!("total risk: {:?}", risk);
}

fn part2(cave: Cave) {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (y, row) in cave.heightmap.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if cave.check_low_point(x, y) {
                low_points.push((x, y));
            };
        }
    }

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| {
            cave.get_basin(*x, *y, &mut Vec::new(), &mut Vec::new())
                .len()
        })
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    let total: usize = basin_sizes.iter().take(3).product();
    println!("total basin: {:?}", total);
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let cave = Cave::new(lines);
    match part {
        "1" => part1(cave),
        "2" => part2(cave),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day9", Function{func: main})
}
