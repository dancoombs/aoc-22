use advent_of_code::helpers::Input;
use anyhow::{anyhow, Context, Result};
use itertools::{enumerate, Itertools};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref DIRS: HashMap<&'static str, [i32; 2]> =
        HashMap::from([("R", [0, 1]), ("L", [0, -1]), ("U", [1, 0]), ("D", [-1, 0])]);
}

pub fn part_one(input: Input) -> Result<u32> {
    simulate_rope(input, 2, false)
}

pub fn part_two(input: Input) -> Result<u32> {
    simulate_rope(input, 10, false)
}

fn simulate_rope(input: Input, num_knots: usize, print: bool) -> Result<u32> {
    let steps = parse_input(input)?;
    let mut tail_locs = HashSet::<[i32; 2]>::new();

    let mut knots = vec![[0; 2]; num_knots];

    for (dir, count) in steps {
        for _ in 0..count {
            // move head
            knots[0][0] += dir[0];
            knots[0][1] += dir[1];

            // knots follow
            for i in 1..knots.len() {
                move_knot(&mut knots, i);
            }

            tail_locs.insert(*knots.last().context(anyhow!("No tail"))?);

            if print {
                print_rope(&knots);
            }
        }
    }

    Ok(tail_locs.len() as u32)
}

fn move_knot(knots: &mut [[i32; 2]], i: usize) {
    let dist = knot_dist(&knots[i - 1], &knots[i]);
    if dist[0].abs() == 2 || dist[1].abs() == 2 {
        knots[i][0] += move_towards(dist[0]);
        knots[i][1] += move_towards(dist[1]);
    }
}

fn knot_dist(a: &[i32; 2], b: &[i32; 2]) -> [i32; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

fn move_towards(dist: i32) -> i32 {
    match dist.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn parse_input(input: Input) -> Result<Vec<([i32; 2], usize)>> {
    input
        .as_str()
        .lines()
        .map(|l| {
            let (d, c) = l
                .split_whitespace()
                .collect_tuple()
                .ok_or_else(|| anyhow!("Invalid input"))?;
            let dir = DIRS.get(d).ok_or_else(|| anyhow!("Invalid direction"))?;
            let count = c.parse::<usize>()?;
            Ok((*dir, count))
        })
        .collect()
}

fn print_rope(knots: &[[i32; 2]]) {
    let mut grid = vec![vec![".".to_owned(); 6]; 5];
    for (i, knot) in enumerate(knots).rev() {
        grid[knot[0] as usize][knot[1] as usize] = if i == 0 {
            "H".to_string()
        } else {
            i.to_string()
        };
    }
    for row in grid.iter().rev() {
        println!("{}", row.join(""));
    }
    println!();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 88);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 36);
    }
}
