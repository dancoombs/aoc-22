use std::collections::HashMap;

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};

pub fn part_one(input: Input) -> Result<i64> {
    let monkeys = parse(input)?;
    solve_one(&monkeys, "root")
}

pub fn part_two(input: Input) -> Result<i64> {
    let monkeys = parse(input)?;
    let root = monkeys
        .get("root")
        .ok_or_else(|| anyhow!("root not found"))?;
    let root_parts = root.split_whitespace().collect::<Vec<_>>();

    let mut contains_human = HashMap::new();
    mark_contains_human(&monkeys, "root", &mut contains_human)?;

    if *contains_human.get(root_parts[0]).unwrap() && *contains_human.get(root_parts[2]).unwrap() {
        return Err(anyhow!("Both monkeys are humans"));
    }

    let (solve, eq) = if *contains_human.get(root_parts[0]).unwrap() {
        (root_parts[0], solve_one(&monkeys, root_parts[2])?)
    } else {
        (root_parts[2], solve_one(&monkeys, root_parts[0])?)
    };

    solve_two(&monkeys, solve, &contains_human, eq)
}

fn solve_one(monkeys: &HashMap<String, String>, name: &str) -> Result<i64> {
    let value = monkeys
        .get(name)
        .ok_or_else(|| anyhow!("Monkey not found"))?;
    if let Ok(value) = value.parse::<i64>() {
        return Ok(value);
    }

    let parts = value.split_whitespace().collect::<Vec<_>>();
    let p0 = solve_one(monkeys, parts[0])?;
    let p1 = solve_one(monkeys, parts[2])?;

    match parts[1] {
        "+" => Ok(p0 + p1),
        "-" => Ok(p0 - p1),
        "*" => Ok(p0 * p1),
        "/" => Ok(p0 / p1),
        _ => Err(anyhow!("Unknown operator")),
    }
}

// x op y = z inverted
fn solve_two(
    monkeys: &HashMap<String, String>,
    name: &str,
    contains_human: &HashMap<String, bool>,
    z: i64,
) -> Result<i64> {
    if name == "humn" {
        return Ok(z);
    }

    let value = monkeys
        .get(name)
        .ok_or_else(|| anyhow!("Monkey not found"))?;
    if let Ok(value) = value.parse::<i64>() {
        return Ok(value);
    }

    let parts = value.split_whitespace().collect::<Vec<_>>();
    if *contains_human.get(parts[0]).unwrap() {
        let y = solve_one(monkeys, parts[2])?;
        match parts[1] {
            "+" => solve_two(monkeys, parts[0], contains_human, z - y),
            "-" => solve_two(monkeys, parts[0], contains_human, z + y),
            "*" => solve_two(monkeys, parts[0], contains_human, z / y),
            "/" => solve_two(monkeys, parts[0], contains_human, z * y),
            _ => Err(anyhow!("Unknown operator")),
        }
    } else {
        let x = solve_one(monkeys, parts[0])?;
        match parts[1] {
            "+" => solve_two(monkeys, parts[2], contains_human, z - x),
            "-" => solve_two(monkeys, parts[2], contains_human, x - z),
            "*" => solve_two(monkeys, parts[2], contains_human, z / x),
            "/" => solve_two(monkeys, parts[2], contains_human, x / z),
            _ => Err(anyhow!("Unknown operator")),
        }
    }
}

fn mark_contains_human(
    monkeys: &HashMap<String, String>,
    name: &str,
    contains_human: &mut HashMap<String, bool>,
) -> Result<bool> {
    let value = monkeys
        .get(name)
        .ok_or_else(|| anyhow!("Monkey not found"))?;

    let contains = if name == "humn" {
        true
    } else if value.parse::<i64>().is_err() {
        let parts = value.split_whitespace().collect::<Vec<_>>();
        let a = mark_contains_human(monkeys, parts[0], contains_human)?;
        let b = mark_contains_human(monkeys, parts[2], contains_human)?;
        a || b
    } else {
        false
    };

    contains_human.insert(name.to_owned(), contains);
    Ok(contains)
}

fn parse(input: Input) -> Result<HashMap<String, String>> {
    input
        .as_str()
        .lines()
        .map(|l| {
            let mut split = l.split(": ");
            let from = split.next().unwrap();
            let to = split.next().unwrap();
            Ok((from.to_string(), to.to_string()))
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 152);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 301);
    }
}
