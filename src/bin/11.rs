use std::{collections::VecDeque, str::FromStr};

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Context, Result};

pub fn part_one(input: Input) -> Result<u64> {
    monkey_business(input, 20, 3)
}

pub fn part_two(input: Input) -> Result<u64> {
    monkey_business(input, 10000, 1)
}

fn monkey_business(input: Input, num_rounds: usize, worry_divisor: u64) -> Result<u64> {
    let mut monkeys = parse_monkeys(input)?;
    let mut num_inspections = run_rounds(&mut monkeys, num_rounds, worry_divisor)?;
    num_inspections.sort();
    Ok(num_inspections[num_inspections.len() - 1] * num_inspections[num_inspections.len() - 2])
}

fn run_rounds(
    monkeys: &mut Vec<Monkey>,
    num_rounds: usize,
    worry_divisor: u64,
) -> Result<Vec<u64>> {
    // trick to keep the worry score from overflowing while still maintaining the
    // modular arithmetic
    let common_divisor: u64 = monkeys.iter().map(|m| m.divisor).product();
    let mut num_inspections = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        for m in 0..monkeys.len() {
            while !monkeys[m].is_empty() {
                let (new_worry, to) = monkeys[m].throw(worry_divisor)?;
                num_inspections[m] += 1;
                monkeys[to].items.push_back(new_worry % common_divisor);
            }
        }
    }
    Ok(num_inspections)
}

fn parse_monkeys(input: Input) -> Result<Vec<Monkey>> {
    input
        .as_str()
        .split("\n\n")
        .map(|g| g.parse())
        .collect::<Result<_>>()
}

type OpFunc = Box<dyn Fn(u64) -> u64>;
type TestFunc = Box<dyn Fn(u64) -> usize>;

struct Monkey {
    items: VecDeque<u64>,
    op: OpFunc,
    test: TestFunc,
    divisor: u64,
}

impl Monkey {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn throw(&mut self, divisor: u64) -> Result<(u64, usize)> {
        let item = self.items.pop_front().context("nothing to throw")?;
        let new_worry = (self.op)(item) / divisor;
        let to = (self.test)(new_worry);
        Ok((new_worry, to))
    }

    fn create_items(s: &str) -> Result<VecDeque<u64>> {
        s.strip_prefix("  Starting items: ")
            .context("no items")?
            .split(", ")
            .map(|s| s.parse().context("can't parse item"))
            .collect()
    }

    fn create_op(s: &str) -> Result<OpFunc> {
        let parts: Vec<&str> = s
            .strip_prefix("  Operation: new = ")
            .context("no operation")?
            .split(' ')
            .collect();
        match parts[1] {
            "+" => {
                let operand: u64 = parts[2].parse()?;
                Ok(Box::new(move |x| x + operand))
            }
            "*" => match parts[2] {
                "old" => Ok(Box::new(|x| x * x)),
                _ => {
                    let operand: u64 = parts[2].parse()?;
                    Ok(Box::new(move |x| x * operand))
                }
            },
            _ => Err(anyhow!("Unknown operation")),
        }
    }

    fn create_test(s: Vec<&str>) -> Result<(TestFunc, u64)> {
        let divisor: u64 = s[0]
            .strip_prefix("  Test: divisible by ")
            .context("no test")?
            .parse()?;
        let true_monkey = s[1]
            .strip_prefix("    If true: throw to monkey ")
            .context("no true monkey")?
            .parse()?;
        let false_monkey = s[2]
            .strip_prefix("    If false: throw to monkey ")
            .context("no false monkey")?
            .parse()?;
        Ok((
            Box::new(move |x| {
                if x % divisor == 0 {
                    true_monkey
                } else {
                    false_monkey
                }
            }),
            divisor,
        ))
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // skip "Monkey #"
        let items = Monkey::create_items(lines.next().context("no items")?)?;
        let op = Monkey::create_op(lines.next().context("no operation")?)?;
        let test = Monkey::create_test(lines.collect())?;
        Ok(Self {
            items,
            op,
            test: test.0,
            divisor: test.1,
        })
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 10605);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 2713310158);
    }
}
