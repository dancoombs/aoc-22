use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};
use itertools::Itertools;

// 0 = right, 1 = down, 2 = left, 3 = up

pub fn part_one(input: Input) -> Result<u64> {
    solve(input, true)
}

pub fn part_two(input: Input) -> Result<u64> {
    solve(input, false)
}

fn solve(input: Input, part_one: bool) -> Result<u64> {
    let (path, grid) = parse(input)?;
    let mut dir = 0;
    let mut pos = (1, 1);
    while grid[1][pos.1] == ' ' {
        pos.1 += 1;
    }

    let mut path_chars = path.chars().peekable();
    while path_chars.peek().is_some() {
        let op = path_chars
            .take_while_ref(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>();
        if let Ok(cnt) = op {
            (pos, dir) = move_pos(&grid, dir, &pos, cnt, part_one);
        } else if let Some(r) = path_chars.next() {
            dir = rotate(dir, r == 'L');
        }
    }

    println!("{pos:?} {dir}");

    Ok(password(dir, &pos))
}

fn inc((x, y): &(usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (*x, y + 1),
        1 => (x + 1, *y),
        2 => (*x, y - 1),
        3 => (x - 1, *y),
        _ => unreachable!(),
    }
}

fn wrap_one(pos: &(usize, usize), dir: usize, grid: &[Vec<char>]) -> (usize, usize) {
    let mut new = match dir {
        0 => (pos.0, 0),
        1 => (0, pos.1),
        2 => (pos.0, grid[0].len() - 1),
        3 => (grid.len() - 1, pos.1),
        _ => unreachable!(),
    };

    while grid[new.0][new.1] == ' ' {
        new = inc(&new, dir);
    }

    new
}

// CUBE_DIRS[face][dir] = (new_face, new_dir, flip)
#[rustfmt::skip]
const CUBE_DIRS : [[(usize, usize, bool); 4]; 6] = [
    [
        (3, 2, true),
        (2, 2, false),
        (1, 2, false),
        (5, 3, false),
    ],
    [
        (0, 0, false),
        (2, 1, false),
        (4, 0, true),
        (5, 0, false),
    ],
    [
        (0, 3, false),
        (0, 1, false),
        (4, 1, false),
        (1, 3, false),
    ],
    [
        (0, 2, true),
        (5, 2, false),
        (4, 2, false),
        (2, 3, false),
    ],
    [
        (3, 0, false),
        (5, 1, false),
        (1, 0, true),
        (2, 0, false),
    ],
    [
        (3, 3, false),
        (0, 1, false),
        (1, 1, false),
        (4, 3, false),
    ],
];

const EDGES: [[(usize, usize); 2]; 6] = [
    [(1, 50), (101, 150)],
    [(1, 50), (51, 100)],
    [(51, 100), (51, 100)],
    [(101, 150), (51, 100)],
    [(101, 150), (1, 50)],
    [(151, 200), (1, 50)],
];

// TESTING DATA

// CUBE_DIRS[face][dir] = (new_face, new_dir, flip)
// #[rustfmt::skip]
// const CUBE_DIRS: [[(usize, usize, bool); 4]; 6] = [
//     [
//         (3, 2, true),
//         (1, 1, false),
//         (4, 1, false),
//         (5, 1, true),
//     ],
//     [
//         (3, 1, true),
//         (2, 1, false),
//         (4, 2, false),
//         (0, 3, false),
//     ],
//     [
//         (3, 0, false),
//         (5, 3, true),
//         (3, 4, true),
//         (1, 3, false),
//     ],
//     [
//         (0, 2, true),
//         (5, 0, true),
//         (2, 2, false),
//         (1, 2, true),
//     ],
//     [
//         (1, 0, false),
//         (2, 0, true),
//         (5, 2, false),
//         (0, 0, false),
//     ],
//     [
//         (4, 0, false),
//         (3, 2, true),
//         (3, 3, true),
//         (0, 1, true),
//     ],
// ];

// const EDGES: [[(usize, usize); 2]; 6] = [
//     [(1, 4), (9, 12)],
//     [(5, 8), (9, 12)],
//     [(9, 12), (9, 12)],
//     [(9, 12), (13, 16)],
//     [(5, 8), (5, 8)],
//     [(5, 8), (1, 4)],
// ];

// 1. determine which face you're on
//   check if you're on an edge
// 2. determine which face you're going to using the cube dir table
// 3. determine new direction using the cube dir table
// 4. determine new coordinates using the edge table and flip bool
//  extract coordinate, flip if needed, apply to new edge

fn wrap_two(pos: &(usize, usize), dir: usize) -> ((usize, usize), usize) {
    let face = EDGES
        .iter()
        .position(|e| e[0].0 <= pos.0 && pos.0 <= e[0].1 && e[1].0 <= pos.1 && pos.1 <= e[1].1)
        .unwrap();

    let dist_on_edge = match dir {
        0 | 2 => pos.0 - EDGES[face][0].0,
        1 | 3 => pos.1 - EDGES[face][1].0,
        _ => unreachable!(),
    };

    let (new_face, new_dir, flip) = CUBE_DIRS[face][dir];

    let new_pos = match new_dir {
        0 => {
            if flip {
                (EDGES[new_face][0].1 - dist_on_edge, EDGES[new_face][1].0)
            } else {
                (EDGES[new_face][0].0 + dist_on_edge, EDGES[new_face][1].0)
            }
        }
        1 => {
            if flip {
                (EDGES[new_face][0].0, EDGES[new_face][1].1 - dist_on_edge)
            } else {
                (EDGES[new_face][0].0, EDGES[new_face][1].0 + dist_on_edge)
            }
        }
        2 => {
            if flip {
                (EDGES[new_face][0].1 - dist_on_edge, EDGES[new_face][1].1)
            } else {
                (EDGES[new_face][0].0 + dist_on_edge, EDGES[new_face][1].1)
            }
        }
        3 => {
            if flip {
                (EDGES[new_face][0].1, EDGES[new_face][1].1 - dist_on_edge)
            } else {
                (EDGES[new_face][0].1, EDGES[new_face][1].0 + dist_on_edge)
            }
        }
        _ => unreachable!(),
    };

    (new_pos, new_dir)
}

fn move_pos(
    grid: &[Vec<char>],
    dir: usize,
    pos: &(usize, usize),
    count: usize,
    part_one: bool,
) -> ((usize, usize), usize) {
    let mut new_pos = *pos;
    let mut new_dir = dir;
    for _ in 0..count {
        let mut next = inc(&new_pos, new_dir);
        let mut next_dir = new_dir;
        // check for wrap around, find next character
        if grid[next.0][next.1] == ' ' {
            if part_one {
                next = wrap_one(&next, next_dir, grid);
            } else {
                (next, next_dir) = wrap_two(&new_pos, next_dir);
            }
        }

        // check for wall
        if grid[next.0][next.1] == '#' {
            break;
        }
        new_pos = next;
        new_dir = next_dir;
    }
    (new_pos, new_dir)
}

fn rotate(dir: usize, left: bool) -> usize {
    if left {
        if dir == 0 {
            3
        } else {
            dir - 1
        }
    } else {
        (dir + 1) % 4
    }
}

fn password(dir: usize, pos: &(usize, usize)) -> u64 {
    1000 * pos.0 as u64 + 4 * pos.1 as u64 + dir as u64
}

fn parse(input: Input) -> Result<(String, Vec<Vec<char>>)> {
    let parts = input.as_str().split("\n\n").collect::<Vec<_>>();

    let max = parts[0]
        .lines()
        .map(|line| line.len())
        .max()
        .ok_or_else(|| anyhow!("No lines"))?
        + 2;

    // pad the grid with spaces so we don't need to do edge checking logic
    let mut grid = vec![vec![' '; max]];
    grid.extend(parts[0].lines().map(|line| {
        let mut chars = vec![' '];
        chars.extend(line.chars());
        chars.resize(max, ' ');
        chars
    }));
    grid.push(vec![' '; max]);

    Ok((parts[1].trim_end().to_string(), grid))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 6032);
    }

    // COMMENT IN TESTING DATA TO TEST PART TWO
    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 22);
    //     assert_eq!(part_two(Input::new(&input)).unwrap(), 5031);
    // }
}
