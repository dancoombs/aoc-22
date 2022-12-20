use advent_of_code::helpers::Input;
use anyhow::Result;

pub fn part_one(input: Input) -> Result<i64> {
    run(input, 1, 1)
}

pub fn part_two(input: Input) -> Result<i64> {
    run(input, 811589153, 10)
}

fn modi(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn mix<T>(v: &mut Vec<T>, from: usize, to: usize) {
    let i = v.remove(from);
    v.insert(to, i);
}

fn run(input: Input, mult: i64, rounds: i64) -> Result<i64> {
    let orig = parse(input, mult);
    let mut pos = Vec::from_iter(0..orig.len());
    let mut mixed = orig.clone();
    let len = orig.len() as i64;

    for _ in 0..rounds {
        for (i, v) in orig.iter().enumerate() {
            // find pos, cool trick dphil
            let p = pos.iter().position(|&x| x == i).unwrap() as i64;
            let to = modi(p + v, len - 1);

            // mix value
            mix(&mut mixed, p as usize, to as usize);
            mix(&mut pos, p as usize, to as usize);
        }
    }

    let mut iter = mixed.iter().cycle();
    for i in iter.by_ref() {
        if *i == 0 {
            break;
        }
    }
    let i = iter.nth(999).unwrap();
    let j = iter.nth(999).unwrap();
    let k = iter.nth(999).unwrap();

    Ok(i + j + k)
}

fn parse(input: Input, mult: i64) -> Vec<i64> {
    input
        .as_str()
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * mult)
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 3);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 1623178306);
    }
}
