use advent_of_code::helpers::Input;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: Input) -> Result<String> {
    let (mut stacks, insts) = parse_input(input)?;
    for i in insts {
        for _ in 0..i.count {
            let x = stacks[i.from - 1].pop().context("invalid pop")?;
            stacks[i.to - 1].push(x);
        }
    }

    Ok(stack_tops(&stacks))
}

pub fn part_two(input: Input) -> Result<String> {
    let (mut stacks, insts) = parse_input(input)?;
    for i in insts {
        let mut cs: Vec<char> = vec![];
        for _ in 0..i.count {
            cs.push(stacks[i.from - 1].pop().context("invalid pop")?);
        }
        stacks[i.to - 1].extend(cs.iter().rev());
    }

    Ok(stack_tops(&stacks))
}

fn parse_input(input: Input) -> Result<(Vec<Vec<char>>, Vec<Instruction>)> {
    let (initial, instructions) = input.split("\n\n");
    let stacks = parse_initial_state(initial)?;
    let insts: Vec<Instruction> = instructions
        .lines()
        .map(|l| Instruction::from_string(l).unwrap())
        .collect();
    Ok((stacks, insts))
}

fn stack_tops(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .map(|s| s.last().unwrap())
        .fold("".to_string(), |ret, c| format!("{ret}{c}"))
}

fn parse_initial_state(s: &str) -> Result<Vec<Vec<char>>> {
    let num = s
        .lines()
        .last()
        .context("no lines parsed")?
        .split_whitespace()
        .last()
        .context("no whitespace parsed")?
        .parse::<usize>()?;
    let mut stacks = vec![Vec::<char>::new(); num];

    s.lines().rev().skip(1).for_each(|l| {
        for (i, c) in l.chars().enumerate() {
            if c.is_uppercase() {
                stacks[(i - 1) / 4].push(c)
            }
        }
    });

    Ok(stacks)
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_string(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        }
        let captures = RE.captures(s).context("no matches found")?;
        Ok(Self {
            count: captures
                .name("count")
                .context("count not found")?
                .as_str()
                .parse()?,
            from: captures
                .name("from")
                .context("from not found")?
                .as_str()
                .parse()?,
            to: captures
                .name("to")
                .context("to not found")?
                .as_str()
                .parse()?,
        })
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(Input::new(&input)).unwrap(), "CMZ".to_string());
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(Input::new(&input)).unwrap(), "MCD".to_string());
    }
}
