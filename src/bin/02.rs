use advent_of_code::helpers::Input;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: Input) -> Option<u32> {
    let score_map: HashMap<(&str, &str), u32> = HashMap::from([
        (("A", "X"), 4),
        (("A", "Y"), 8),
        (("A", "Z"), 3),
        (("B", "X"), 1),
        (("B", "Y"), 5),
        (("B", "Z"), 9),
        (("C", "X"), 7),
        (("C", "Y"), 2),
        (("C", "Z"), 6),
    ]);

    score(input, &score_map)
}

pub fn part_two(input: Input) -> Option<u32> {
    let score_map: HashMap<(&str, &str), u32> = HashMap::from([
        (("A", "X"), 3),
        (("A", "Y"), 4),
        (("A", "Z"), 8),
        (("B", "X"), 1),
        (("B", "Y"), 5),
        (("B", "Z"), 9),
        (("C", "X"), 2),
        (("C", "Y"), 6),
        (("C", "Z"), 7),
    ]);

    score(input, &score_map)
}

fn score(input: Input, rules: &HashMap<(&str, &str), u32>) -> Option<u32> {
    input
        .split_and_tform_lines(|l| *rules.get(&parse_move(l)).unwrap())
        .sum1()
}

fn parse_move(l: &str) -> (&str, &str) {
    l.split(' ').collect_tuple().unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(Input::new(&input)), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(Input::new(&input)), Some(12));
    }
}
