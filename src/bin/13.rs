use std::cmp::Ordering;

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use serde_json::{json, Value};

pub fn part_one(input: Input) -> Result<usize> {
    Ok(input
        .as_str()
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|l| serde_json::from_str::<Value>(l).unwrap())
                .collect_tuple()
                .unwrap()
        })
        .positions(|(a, b)| ord(&a, &b).unwrap() == Ordering::Less)
        .map(|i| i + 1)
        .sum())
}

pub fn part_two(input: Input) -> Result<usize> {
    let mut packets: Vec<Value> = input
        .as_str()
        .split_ascii_whitespace()
        .map(|l| serde_json::from_str::<Value>(l).context("bad json conversion"))
        .collect::<Result<_>>()?;

    let beacons = vec![json!([json!([2])]), json!([json!([6])])];

    packets.extend(beacons.iter().cloned());
    packets.sort_by(|a, b| ord(a, b).unwrap());

    Ok(packets
        .iter()
        .positions(|p| beacons.contains(p))
        .map(|i| i + 1)
        .product())
}

fn num_ord(a: &serde_json::Number, b: &serde_json::Number) -> Result<Ordering> {
    Ok(a.as_u64()
        .context("bad number conversion")?
        .cmp(&b.as_u64().context("bad number conversion")?))
}

fn ord(x: &Value, y: &Value) -> Result<Ordering> {
    match (x, y) {
        (Value::Number(a), Value::Number(b)) => num_ord(a, b),
        (Value::Array(a), Value::Array(b)) => {
            for (a_i, b_i) in a.iter().zip(b) {
                let ord = ord(a_i, b_i)?;
                if ord != Ordering::Equal {
                    return Ok(ord);
                }
            }
            Ok(a.len().cmp(&b.len()))
        }
        (Value::Number(_), Value::Array(_)) => ord(&json!([x.clone()]), y),
        (Value::Array(_), Value::Number(_)) => ord(x, &json!([y.clone()])),
        _ => Err(anyhow!("Unexpected value")),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 140);
    }
}
