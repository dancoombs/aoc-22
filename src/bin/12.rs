use std::collections::VecDeque;

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};

pub fn part_one(input: Input) -> Result<u32> {
    let pr = parse_grid(input)?;
    bfs(&pr.grid, pr.start, pr.end)
}

pub fn part_two(input: Input) -> Result<u32> {
    let pr = parse_grid(input)?;
    let mut starts = pr.lowests.clone();
    starts.push(pr.start);
    starts
        .iter()
        .map(|&s| bfs(&pr.grid, s, pr.end).unwrap_or(u32::MAX))
        .min()
        .ok_or_else(|| anyhow!("no paths found"))
}

fn bfs(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> Result<u32> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start.0][start.1] = true;
    let mut q = VecDeque::from([(start, 0)]);
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !q.is_empty() {
        let ((i, j), dist) = q.pop_front().unwrap();
        if (i, j) == end {
            return Ok(dist);
        }
        let h = grid[i][j];
        for dir in dirs {
            let (m, n) = (i as i32 + dir.0, j as i32 + dir.1);
            if m < 0 || n < 0 || m >= grid.len() as i32 || n >= grid[0].len() as i32 {
                continue;
            }
            let (m, n) = (m as usize, n as usize);
            if visited[m][n] || grid[m][n] as i32 - h as i32 > 1 {
                continue;
            }
            visited[m][n] = true;
            q.push_back(((m, n), dist + 1));
        }
    }

    Err(anyhow!("Path not found!"))
}

struct ParseResult {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    lowests: Vec<(usize, usize)>,
    end: (usize, usize),
}

fn parse_grid(input: Input) -> Result<ParseResult> {
    let mut start = (usize::MAX, usize::MAX);
    let mut lowests = vec![];
    let mut end = (usize::MAX, usize::MAX);
    let grid = input
        .as_str()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start = (i, j);
                        0
                    }
                    'E' => {
                        end = (i, j);
                        25
                    }
                    'a' => {
                        lowests.push((i, j));
                        0
                    }
                    _ => c as u8 - b'a',
                })
                .collect()
        })
        .collect();
    Ok(ParseResult {
        grid,
        start,
        lowests,
        end,
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 31);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 29);
    }
}
