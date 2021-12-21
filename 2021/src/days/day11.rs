use crate::registry::{DayCommand, Function};
use crate::utils;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Octopus {
    energy: usize,
    flashed: bool,
}

#[derive(Debug)]
struct Cavern {
    octopi: Vec<Vec<Octopus>>,
    height: usize,
    width: usize,
}

impl FromStr for Cavern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<Octopus>> = s
            .split('\n')
            .map(|r| {
                r.chars()
                    .map(|c| Octopus {
                        energy: c.to_digit(10).unwrap() as usize,
                        flashed: false,
                    })
                    .collect::<Vec<Octopus>>()
            })
            .collect();
        let height = data.len();
        let width = data[0].len();
        Ok(Cavern {
            octopi: data,
            height,
            width,
        })
    }
}

impl Cavern {
    fn flash(&mut self, x: isize, y: isize) {
        // If already flashed, do nothing
        if self.octopi[y as usize][x as usize].flashed {
            return;
        }

        // If less than 9 energy, do nothing (level has already been incremented by parent call)
        if self.octopi[y as usize][x as usize].energy <= 9 {
            return;
        }

        // Consider this octopus to have flashed
        self.octopi[y as usize][x as usize].flashed = true;

        let dirs = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for (dx, dy) in dirs {
            if (x + dx) < 0
                || (y + dy) < 0
                || (x + dx) >= self.width as isize
                || (y + dy) >= self.height as isize
            {
                continue;
            }
            self.octopi[(y + dy) as usize][(x + dx) as usize].energy += 1;
            self.flash(x + dx, y + dy);
        }
    }

    fn step(&mut self, break_on_sync: bool) -> (usize, bool) {
        let mut flashers: Vec<(isize, isize)> = Vec::new();
        // Global increase
        for y in 0..self.height {
            for x in 0..self.width {
                self.octopi[y][x].energy += 1;
                if self.octopi[y][x].energy > 9 {
                    flashers.push((x as isize, y as isize));
                }
            }
        }

        // Flash
        while let Some((x, y)) = flashers.pop() {
            self.flash(x, y);
        }

        // Count flashes
        let flash_count = self.octopi.iter().flatten().filter(|o| o.flashed).count();

        if break_on_sync && flash_count == self.height * self.width {
            return (flash_count, true);
        }

        // Reset
        for y in 0..self.height {
            for x in 0..self.width {
                if self.octopi[y][x].flashed {
                    self.octopi[y][x].energy = 0;
                }

                self.octopi[y][x].flashed = false;
            }
        }
        (flash_count, false)
    }
}

fn part1(mut cavern: Cavern) {
    let mut flash_count = 0;
    for _idx in 0..100 {
        let (n, _) = cavern.step(false);
        flash_count += n;
    }
    println!("flash count {}", flash_count);
}

fn part2(mut cavern: Cavern) {
    let mut step = 0;
    loop {
        step += 1;
        let (_n, sync) = cavern.step(true);
        if sync {
            println!("First step synced is {}", step);
            break;
        }
    }
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let cavern = Cavern::from_str(&lines.join("\n")).unwrap();
    match part {
        "1" => part1(cavern),
        "2" => part2(cavern),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day11", Function{func: main})
}
