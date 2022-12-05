use advent_of_code::helpers::Input;
use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: Input) -> Option<u32> {
    let (mut stacks, insts) = parse_input(input);
    insts.iter().for_each(|i| {
        for _ in 0..i.count {
            let x = stacks[i.from - 1].pop().unwrap();
            stacks[i.to - 1].push(x);
        }
    });

    let ret = stack_tops(&stacks);
    println!("Day 5 Part 1 Solution: {ret}");
    Some(0)
}

pub fn part_two(input: Input) -> Option<u32> {
    let (mut stacks, insts) = parse_input(input);
    insts.iter().for_each(|i| {
        let mut cs: Vec<char> = vec![];
        for _ in 0..i.count {
            cs.push(stacks[i.from - 1].pop().unwrap());
        }
        stacks[i.to - 1].extend(cs.iter().rev());
    });

    let ret = stack_tops(&stacks);
    println!("Day 5 Part 2 Solution: {ret}");
    Some(0)
}

fn parse_input(input: Input) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (initial, instructions) = input.split("\n\n");
    let stacks = parse_initial_state(initial);
    let insts: Vec<Instruction> = instructions.lines().map(Instruction::from_string).collect();
    (stacks, insts)
}

fn stack_tops(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|s| s.last().unwrap())
        .fold("".to_string(), |ret, c| format!("{ret}{c}"))
}

fn parse_initial_state(s: &str) -> Vec<Vec<char>> {
    let num = s
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = vec![Vec::<char>::new(); num];

    s.lines().rev().skip(1).for_each(|l| {
        for (i, c) in l.chars().enumerate() {
            if c.is_uppercase() {
                stacks[(i - 1) / 4].push(c)
            }
        }
    });

    stacks
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_string(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        }
        Self {
            count: RE
                .captures(s)
                .unwrap()
                .name("count")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            from: RE
                .captures(s)
                .unwrap()
                .name("from")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            to: RE
                .captures(s)
                .unwrap()
                .name("to")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
        }
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
        assert_eq!(part_one(Input::new(&input)), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(Input::new(&input)), Some(0));
    }
}
