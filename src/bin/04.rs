use std::ops::Range;

use advent_of_code::helpers::Input;

pub fn part_one(input: Input) -> Option<u32> {
    Some(
        get_ranges(input)
            .filter(|(a, b)| range_contains(a, b) || range_contains(b, a))
            .count() as u32,
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    Some(
        get_ranges(input)
            .filter(|(a, b)| ranges_overlap(a, b))
            .count() as u32,
    )
}

fn get_ranges(input: Input) -> impl Iterator<Item = (Range<u32>, Range<u32>)> + '_ {
    input.split_and_tform_lines(|l| {
        let mut i = l.split(',').map(parse_range);
        (i.next().unwrap(), i.next().unwrap())
    })
}

fn parse_range(s: &str) -> Range<u32> {
    let mut i = s.split('-').map(|a| a.parse::<u32>().unwrap());
    Range {
        start: i.next().unwrap(),
        end: i.next().unwrap(),
    }
}

fn range_contains(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start <= b.start && a.end >= b.end
}

fn ranges_overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    b.start <= a.end && a.start <= b.end
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(Input::new(&input)), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(Input::new(&input)), Some(4));
    }
}
