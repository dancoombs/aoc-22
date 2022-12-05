use advent_of_code::helpers::Input;
use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part_one(input: Input) -> Result<u32> {
    get_sums(input).max().context("max not found")
}

pub fn part_two(input: Input) -> Result<u32> {
    Ok(get_sums(input).sorted().rev().take(3).sum())
}

pub fn get_sums(input: Input) -> impl Iterator<Item = u32> + '_ {
    input
        .group_and_tform_lines(|l| l.parse::<u32>().unwrap())
        .map(|g| g.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 24000);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 45000);
    }
}
