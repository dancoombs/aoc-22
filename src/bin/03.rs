use std::collections::HashSet;

use advent_of_code::helpers::Input;
use itertools::Itertools;

pub fn part_one(input: Input) -> Option<u32> {
    input
        .split_and_tform_lines(|l| {
            let (c0, c1) = l.split_at(l.len() / 2);
            find_collision(c0, c1)
        })
        .sum1()
}

pub fn part_two(input: Input) -> Option<u32> {
    input
        .as_str()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|g| {
            *g.map(|l| {
                l.as_bytes()
                    .iter()
                    .map(to_priority)
                    .collect::<HashSet<u32>>()
            })
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap()
            .iter()
            .next()
            .unwrap()
        })
        .sum1()
}

fn find_collision(c0: &str, c1: &str) -> u32 {
    let mut items = HashSet::new();
    c0.as_bytes().iter().for_each(|c| {
        items.insert(to_priority(c));
    });
    c1.as_bytes()
        .iter()
        .filter(|c| items.contains(&to_priority(c)))
        .map(to_priority)
        .next()
        .unwrap()
}

fn to_priority(c: &u8) -> u32 {
    match c {
        (97..=122) => (c - 96).into(),
        (65..=90) => (c - 38).into(),
        _ => panic!("Bad char {c}"),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(Input::new(&input)), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(Input::new(&input)), Some(70));
    }
}
