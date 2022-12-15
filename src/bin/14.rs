use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn part_one(_input: Input) -> Result<u32> {
    let pr = parse_paths(_input)?;
    let mut grid = vec![vec![0_u32; pr.y_max + 1]; pr.x_max + 1];
    fill_grid(&mut grid, &pr.paths);

    let mut i = 0;
    loop {
        let mut grain = (500, 0);
        loop {
            if grain.1 == pr.y_max {
                break;
            } else if grid[grain.0][grain.1 + 1] == 0 {
                grain.1 += 1;
            } else if grid[grain.0 - 1][grain.1 + 1] == 0 {
                grain.0 -= 1;
                grain.1 += 1;
            } else if grid[grain.0 + 1][grain.1 + 1] == 0 {
                grain.0 += 1;
                grain.1 += 1;
            } else {
                grid[grain.0][grain.1] = 1;
                break;
            }
        }

        if grain.1 == pr.y_max {
            break;
        }

        i += 1;
    }

    Ok(i)
}

pub fn part_two(_input: Input) -> Result<u32> {
    let pr = parse_paths(_input)?;
    let mut grid = vec![vec![0_u32; pr.y_max + 3]; pr.x_max + 5000];
    fill_grid(&mut grid, &pr.paths);
    grid.iter_mut().for_each(|row| {
        row[pr.y_max + 2] = 1;
    });

    let mut i = 0;
    loop {
        let mut grain = (500, 0);
        loop {
            if grid[grain.0][grain.1 + 1] == 0 {
                grain.1 += 1;
            } else if grid[grain.0 - 1][grain.1 + 1] == 0 {
                grain.0 -= 1;
                grain.1 += 1;
            } else if grid[grain.0 + 1][grain.1 + 1] == 0 {
                grain.0 += 1;
                grain.1 += 1;

                let len = grid.len();
                if len == grain.0 + 1 {
                    grid.push(vec![0; pr.y_max + 3]);
                    grid[len - 1][pr.y_max + 2] = 1;
                }
            } else {
                grid[grain.0][grain.1] = 1;
                break;
            }
        }

        if grain == (500, 0) {
            break;
        }

        i += 1;
    }

    Ok(i + 1)
}

struct ParseResult {
    paths: Vec<Vec<(usize, usize)>>,
    x_max: usize,
    y_max: usize,
}

fn fill_grid(grid: &mut [Vec<u32>], paths: &Vec<Vec<(usize, usize)>>) {
    for path in paths {
        for i in 1..path.len() {
            let (x1, y1) = path[i - 1];
            let (x2, y2) = path[i];

            if x2 > x1 {
                grid[x1..=x2].iter_mut().for_each(|row| row[y1] = 1);
            } else if x1 > x2 {
                grid[x2..=x1].iter_mut().for_each(|row| row[y1] = 1);
            } else if y2 > y1 {
                grid[x1][y1..=y2].iter_mut().for_each(|cell| *cell = 1);
            } else if y1 > y2 {
                grid[x1][y2..=y1].iter_mut().for_each(|cell| *cell = 1);
            }
        }
    }
}

fn parse_paths(input: Input) -> Result<ParseResult> {
    let mut x_max = 0;
    let mut y_max = 0;
    let paths = input
        .as_str()
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    p.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect_tuple()
                        .ok_or_else(|| anyhow!("Invalid path"))
                        .map(|(x, y)| {
                            x_max = x_max.max(x);
                            y_max = y_max.max(y);
                            (x, y)
                        })
                })
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()?;

    Ok(ParseResult {
        paths,
        x_max,
        y_max,
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 24);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 93);
    }
}
