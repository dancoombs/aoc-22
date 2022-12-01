use advent_of_code::helpers::Input;
use itertools::Itertools;

pub fn part_one(input: Input) -> Option<u32> {
    get_sums(input).max()
}

pub fn part_two(input: Input) -> Option<u32> {
    get_sums(input).sorted().rev().take(3).sum1()
}

pub fn get_sums(input: Input) -> impl Iterator<Item = u32> + '_ {
    input
        .tform_and_group_lines(|l| l.parse::<u32>().unwrap())
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
        assert_eq!(part_one(Input::new(&input)), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(Input::new(&input)), Some(70));
    }
}
