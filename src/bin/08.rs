use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};

pub fn part_one(input: Input) -> Result<u32> {
    let grid = parse_grid(input)?;
    let mut counted = vec![vec![false; grid[0].len()]; grid.len()];
    let mut count = (2 * grid.len() + 2 * (grid[0].len() - 2)) as u32; // all outer trees visible

    // left, right
    for i in 1..(grid[0].len() - 1) {
        count += visible_in_dir(&grid, &mut counted, 0, i, [1, 0]);
        count += visible_in_dir(&grid, &mut counted, grid.len() - 1, i, [-1, 0]);
    }
    // up, down
    for i in 1..(grid.len() - 1) {
        count += visible_in_dir(&grid, &mut counted, i, 0, [0, 1]);
        count += visible_in_dir(&grid, &mut counted, i, grid[0].len() - 1, [0, -1]);
    }

    Ok(count)
}

pub fn part_two(input: Input) -> Result<u32> {
    let grid = parse_grid(input)?;
    let mut max_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let score = score_tree(&grid, i, j);
            if score > max_score {
                max_score = score;
            }
        }
    }
    Ok(max_score)
}

fn visible_in_dir(
    grid: &[Vec<u32>],
    counted: &mut [Vec<bool>],
    i: usize,
    j: usize,
    dir: [i32; 2],
) -> u32 {
    let mut max_height = grid[i][j];
    let mut count = 0;
    let (mut x, mut y) = move_dir(dir, i, j);

    while x > 0 && x < (grid.len() - 1) && y > 0 && y < (grid[0].len() - 1) {
        if grid[x][y] > max_height {
            max_height = grid[x][y];
            if !counted[x][y] {
                count += 1;
            }
            counted[x][y] = true;
        }
        (x, y) = move_dir(dir, x, y);
    }

    count
}

fn score_tree(grid: &[Vec<u32>], i: usize, j: usize) -> u32 {
    let dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    let mut score = 1;
    let tree_height = grid[i][j];

    for dir in dirs {
        let mut dir_score = 1;
        let (mut x, mut y) = move_dir(dir, i, j);

        while x > 0 && x < (grid.len() - 1) && y > 0 && y < (grid[0].len() - 1) {
            if grid[x][y] < tree_height {
                dir_score += 1;
            } else {
                break;
            }
            (x, y) = move_dir(dir, x, y);
        }

        score *= dir_score;
    }

    score
}

fn move_dir(dir: [i32; 2], i: usize, j: usize) -> (usize, usize) {
    ((i as i32 + dir[0]) as usize, (j as i32 + dir[1]) as usize)
}

fn parse_grid(input: Input) -> Result<Vec<Vec<u32>>> {
    let mut grid = Vec::new();
    for line in input.as_str().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).ok_or_else(|| anyhow!("Invalid digit"))?);
        }
        grid.push(row);
    }
    Ok(grid)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 21);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 8);
    }
}
