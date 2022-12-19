use advent_of_code::helpers::Input;
use anyhow::{Context, Result};
use itertools::Itertools;

const DIRS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub fn part_one(input: Input) -> Result<u32> {
    let cubes = parse_cubes(&input)?;
    let mut grid = vec![vec![vec![false; 20]; 20]; 20];
    for (x, y, z) in cubes.iter() {
        grid[*x][*y][*z] = true;
    }

    let mut count = 0;
    for (x, y, z) in cubes {
        for (dx, dy, dz) in DIRS.iter() {
            if x as i32 + dx < 0
                || y as i32 + dy < 0
                || z as i32 + dz < 0
                || x as i32 + dx >= grid.len() as i32
                || y as i32 + dy >= grid[x].len() as i32
                || z as i32 + dz >= grid[x][y].len() as i32
                || !grid[(x as i32 + dx) as usize][(y as i32 + dy) as usize]
                    [(z as i32 + dz) as usize]
            {
                count += 1;
            }
        }
    }

    Ok(count)
}

// hardcode max instead of parsing
const MAX_DIM: usize = 22;

pub fn part_two(input: Input) -> Result<u32> {
    let mut cubes = parse_cubes(&input)?;
    // shift so we don't have to deal with negative indices
    for cube in cubes.iter_mut() {
        cube.0 += 1;
        cube.1 += 1;
        cube.2 += 1;
    }

    let mut grid = vec![vec![vec![false; MAX_DIM]; MAX_DIM]; MAX_DIM];
    let mut exposed = vec![vec![vec![false; MAX_DIM]; MAX_DIM]; MAX_DIM];

    for (x, y, z) in cubes.iter() {
        grid[*x][*y][*z] = true;
    }

    // flood fill to determine exposed faces
    let mut q = vec![(0_usize, 0_usize, 0_usize)];
    while !q.is_empty() {
        let n = q.pop().unwrap();
        if grid[n.0][n.1][n.2] || exposed[n.0][n.1][n.2] {
            continue;
        }

        exposed[n.0][n.1][n.2] = true;
        for dir in DIRS.iter() {
            let x = n.0 as i32 + dir.0;
            let y = n.1 as i32 + dir.1;
            let z = n.2 as i32 + dir.2;
            if x >= 0
                && y >= 0
                && z >= 0
                && x < MAX_DIM as i32
                && y < MAX_DIM as i32
                && z < MAX_DIM as i32
            {
                q.push((x as usize, y as usize, z as usize));
            }
        }
    }

    let mut count = 0;
    for n in cubes {
        for dir in DIRS.iter() {
            let x = (n.0 as i32 + dir.0) as usize;
            let y = (n.1 as i32 + dir.1) as usize;
            let z = (n.2 as i32 + dir.2) as usize;

            if !grid[x][y][z] && exposed[x][y][z] {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn parse_cubes(input: &Input) -> Result<Vec<(usize, usize, usize)>> {
    input
        .as_str()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .context("invalid input")
        })
        .collect::<Result<_>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 64);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 58);
    }
}
