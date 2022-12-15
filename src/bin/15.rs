use std::collections::HashSet;

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: Input) -> Result<u32> {
    part_one_inner(input, 2000000)
}

fn part_one_inner(input: Input, y_coord: i32) -> Result<u32> {
    let lines = parse(input)?;

    let beacons = lines
        .iter()
        .filter(|pr| pr.beacon.1 == y_coord)
        .map(|pr| pr.beacon.1)
        .collect::<HashSet<_>>();

    let ranges = lines
        .iter()
        .filter_map(|pr| row_range(&pr.sensor, pr.distance, y_coord))
        .collect::<Vec<_>>();
    let merged = merge_ranges(&ranges);

    let marked = merged.iter().fold(0, |sum, (x1, x2)| sum + (x2 - x1 + 1));
    Ok(marked as u32 - beacons.len() as u32)
}

pub fn part_two(input: Input) -> Result<u64> {
    part_two_inner(input, 4_000_000)
}

fn part_two_inner(input: Input, max_dim: i32) -> Result<u64> {
    let lines = parse(input)?;

    for i in 0..max_dim {
        let ranges = lines
            .iter()
            .filter_map(|pr| row_range(&pr.sensor, pr.distance, i))
            .collect::<Vec<_>>();
        let merged = merge_ranges(&ranges);
        if merged.len() > 1 {
            let x = merged[1].0 - 1;
            let x_c: u64 = x as u64 * 4_000_000;
            return Ok(x_c + i as u64);
        }
    }

    Err(anyhow!("no solution found"))
}

fn merge_ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|(x1, _)| *x1);
    let mut merged = Vec::new();
    let mut current = ranges[0];
    for (x1, x2) in ranges.iter().skip(1) {
        if *x1 <= current.1 {
            current.1 = current.1.max(*x2);
        } else {
            merged.push(current);
            current = (*x1, *x2);
        }
    }
    merged.push(current);
    merged
}

fn row_range((x, y): &(i32, i32), dist: i32, row: i32) -> Option<(i32, i32)> {
    let r = dist - (y - row).abs();
    if r < 0 {
        return None;
    }

    Some((x - r, x + r))
}

fn manhattan_distance((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

struct ParsedLine {
    sensor: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

fn parse_line(l: &str) -> Result<ParsedLine> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Sensor at x=(?P<x1>-?\d+), y=(?P<y1>-?\d+): closest beacon is at x=(?P<x2>-?\d+), y=(?P<y2>-?\d+$)").unwrap();
    }
    let caps = RE.captures(l).context("invalid input")?;
    let x1 = caps
        .name("x1")
        .context("invalid input")?
        .as_str()
        .parse::<i32>()?;
    let y1 = caps
        .name("y1")
        .context("invalid input")?
        .as_str()
        .parse::<i32>()?;
    let x2 = caps
        .name("x2")
        .context("invalid input")?
        .as_str()
        .parse::<i32>()?;
    let y2 = caps
        .name("y2")
        .context("invalid input")?
        .as_str()
        .parse::<i32>()?;

    Ok(ParsedLine {
        sensor: (x1, y1),
        beacon: (x2, y2),
        distance: manhattan_distance(&(x1, y1), &(x2, y2)),
    })
}

fn parse(input: Input) -> Result<Vec<ParsedLine>> {
    input
        .as_str()
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_inner(Input::new(&input), 10).unwrap(), 26);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_inner(Input::new(&input), 20).unwrap(), 56000011);
    }
}
