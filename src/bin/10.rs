use std::str::FromStr;

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn part_one(input: Input) -> Result<i32> {
    let screen = Screen::new(parse_program(input)?);
    let mut res = 0_i32;
    for (i, x) in screen.enumerate() {
        let cycle = i + 1;
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            res += x * cycle as i32;
        }
    }
    Ok(res)
}

pub fn part_two(input: Input) -> Result<i32> {
    let screen = Screen::new(parse_program(input)?);
    let mut pixels = vec![['.'; 40]; 1];
    for (i, x) in screen.enumerate() {
        // i == cycle - 1
        let pos = i % 40;
        if (pos as i32 - x).abs() < 2 {
            while pixels.len() < i / 40 + 1 {
                pixels.push(['.'; 40]);
            }

            pixels[i / 40][pos] = '#';
        }
    }

    for row in pixels {
        println!("{}", row.iter().join(""));
    }

    Ok(0)
}

fn parse_program(input: Input) -> Result<Vec<Opcode>> {
    input.as_str().lines().map(|s| s.parse()).collect()
}

#[derive(Debug)]
enum Opcode {
    Noop,
    Addx(i32),
}

impl FromStr for Opcode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect_vec();
        match parts[0] {
            "noop" => Ok(Opcode::Noop),
            "addx" => {
                let x = parts[1].parse()?;
                Ok(Opcode::Addx(x))
            }
            _ => Err(anyhow!("Unknown instruction: {}", parts[0])),
        }
    }
}

struct Screen {
    program: Vec<Opcode>,
    cntr: usize,
    x: i32,
    stalled: bool,
    inst_idx: usize,
}

impl Screen {
    fn new(program: Vec<Opcode>) -> Self {
        Self {
            program,
            cntr: 0,
            x: 1,
            stalled: false,
            inst_idx: 0,
        }
    }
}

impl Iterator for Screen {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inst_idx >= self.program.len() {
            return None;
        }

        let x_before = self.x;
        self.cntr += 1;
        match self.program[self.inst_idx] {
            Opcode::Noop => self.inst_idx += 1,
            Opcode::Addx(v) => {
                if self.stalled {
                    self.x += v;
                    self.inst_idx += 1;
                }
                self.stalled = !self.stalled;
            }
        };

        Some(x_before)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 13140);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert!(part_two(Input::new(&input)).is_ok());
    }
}
