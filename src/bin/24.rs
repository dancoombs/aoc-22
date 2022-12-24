use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn part_one(input: Input) -> Result<u32> {
    let grid = input.as_str().parse::<Grid>()?;
    let goals = [(grid.cells.len() - 1, grid.cells[0].len() - 2)];

    find_solution(grid, &goals)
}

pub fn part_two(input: Input) -> Result<u32> {
    let grid = input.as_str().parse::<Grid>()?;
    let goals = [
        (grid.cells.len() - 1, grid.cells[0].len() - 2),
        (0, 1),
        (grid.cells.len() - 1, grid.cells[0].len() - 2),
    ];

    find_solution(grid, &goals)
}

fn find_solution(mut grid: Grid, goals: &[(usize, usize)]) -> Result<u32> {
    let mut seen = HashSet::new();
    let mut grid_time = 0;
    let mut states = VecDeque::from([(0, (0, 1), 0)]);

    while !states.is_empty() {
        let (t, (i, j), mut stage) = states.pop_front().unwrap();
        if (i, j) == goals[stage] {
            stage += 1;
            if stage == goals.len() {
                return Ok(t);
            }
        }
        if t == grid_time {
            grid = grid.step();
            grid_time += 1;
        }

        if seen.contains(&(t, (i, j), stage)) {
            continue;
        }
        seen.insert((t, (i, j), stage));

        // move
        for (di, dj) in DIR.iter() {
            // special case for the top left and bottom right corners
            if (i == 0 && *di == -1) || (i == grid.cells.len() - 1 && *di == 1) {
                continue;
            }

            let (ni, nj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
            match &grid.cells[ni][nj] {
                Cell::Wall => continue,
                Cell::Blizzard(v) => {
                    if v.is_empty() {
                        states.push_back((t + 1, (ni, nj), stage));
                    }
                }
            }
        }

        // wait
        if let Cell::Blizzard(v) = &grid.cells[i][j] {
            if v.is_empty() {
                states.push_back((t + 1, (i, j), stage));
            }
        }
    }

    Err(anyhow!("Solution not found"))
}

const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Debug, Clone)]
enum Cell {
    Wall,
    Blizzard(Vec<usize>),
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Blizzard(vec![]),
            '#' => Cell::Wall,
            '>' => Cell::Blizzard(vec![0]),
            '<' => Cell::Blizzard(vec![1]),
            '^' => Cell::Blizzard(vec![2]),
            'v' => Cell::Blizzard(vec![3]),
            _ => panic!("Invalid cell: {}", c),
        }
    }

    fn _to_char(&self) -> char {
        match self {
            Cell::Wall => '#',
            Cell::Blizzard(v) => {
                if v.len() > 1 {
                    return v.len().to_string().chars().next().unwrap();
                }

                match v.as_slice() {
                    [] => '.',
                    [0] => '>',
                    [1] => '<',
                    [2] => '^',
                    [3] => 'v',
                    _ => panic!("Invalid blizzard"),
                }
            }
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn empty(&self) -> Self {
        Grid {
            cells: self
                .cells
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|c| match c {
                            Cell::Blizzard(_) => Cell::Blizzard(vec![]),
                            _ => c.clone(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn step(&self) -> Self {
        let mut ret = self.empty();

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Wall => (),
                    Cell::Blizzard(v) => {
                        for b in v {
                            let (di, dj) = DIR[*b];
                            let (mut ni, mut nj) =
                                ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                            if let Cell::Wall = &ret.cells[ni][nj] {
                                if ni == 0 {
                                    ni = self.cells.len() - 2;
                                } else if ni == self.cells.len() - 1 {
                                    ni = 1;
                                } else if nj == 0 {
                                    nj = self.cells[0].len() - 2;
                                } else if nj == self.cells[0].len() - 1 {
                                    nj = 1;
                                } else {
                                    panic!("Invalid wall");
                                }
                            }

                            if let Cell::Blizzard(nv) = &mut ret.cells[ni][nj] {
                                nv.push(*b);
                            } else {
                                panic!("Invalid cell");
                            }
                        }
                    }
                }
            }
        }

        ret
    }

    fn _print(&self) {
        self.cells.iter().for_each(|row| {
            println!("{}", row.iter().map(Cell::_to_char).join(""));
        });
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cells = s
            .lines()
            .map(|line| line.chars().map(Cell::from_char).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Grid { cells })
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 18);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 54);
    }
}
