use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};

pub fn part_one(input: Input) -> Result<String> {
    let sum = input
        .as_str()
        .lines()
        .map(snafu_to_base10)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<i64>();
    base10_to_snafu(sum)
}

pub fn part_two(_input: Input) -> Result<String> {
    Ok("Merry Christmas!".to_owned())
}

fn snafu_to_base10(input: &str) -> Result<i64> {
    let mut ret = 0;
    for (i, c) in input.chars().rev().enumerate() {
        let d = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => return Err(anyhow!("Invalid character")),
        };
        ret += d * 5_i64.pow(i as u32);
    }

    Ok(ret)
}

fn base10_to_snafu(mut x: i64) -> Result<String> {
    let mut ret = String::new();
    while x > 0 {
        let rem = x % 5;
        let rem = (rem + 2) % 5 - 2;

        let c = match rem {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => return Err(anyhow!("Invalid remainder")),
        };
        ret.push(c);

        x = (x - rem) / 5;
    }

    Ok(ret.chars().rev().collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(Input::new(&input)).unwrap(), "2=-1=0");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(Input::new(&input)).unwrap(), "Merry Christmas!");
    }
}
