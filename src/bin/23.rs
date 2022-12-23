use std::collections::{HashMap, HashSet};

use advent_of_code::helpers::Input;
use anyhow::Result;

const DIRS: [(i32, i32); 8] = [
    (0, 1),   // 0 E
    (-1, 1),  // 1 NE
    (-1, 0),  // 2 N
    (-1, -1), // 3 NW
    (0, -1),  // 4 W
    (1, -1),  // 5 SW
    (1, 0),   // 6 S
    (1, 1),   // 7 SE
];

pub fn part_one(input: Input) -> Result<u32> {
    let mut elves = HashSet::<(i32, i32)>::new();
    for (i, l) in input.as_str().lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    for i in 0..10 {
        // to, from
        let mut maybe_move = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();
        for elf in &elves {
            let mut occ = [false; 8];
            for (d, dir) in DIRS.iter().enumerate() {
                let pos = (elf.0 + dir.0, elf.1 + dir.1);
                if elves.contains(&pos) {
                    occ[d] = true;
                }
            }
            if occ.iter().all(|&o| !o) {
                continue;
            }

            for m in 0..4 {
                match (m + i) % 4 {
                    0 => {
                        if !occ[2] && !occ[1] && !occ[3] {
                            let e = maybe_move.entry((elf.0 - 1, elf.1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    1 => {
                        if !occ[6] && !occ[5] && !occ[7] {
                            let e = maybe_move.entry((elf.0 + 1, elf.1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    2 => {
                        if !occ[3] && !occ[4] && !occ[5] {
                            let e = maybe_move.entry((elf.0, elf.1 - 1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    3 => {
                        if !occ[0] && !occ[1] && !occ[7] {
                            let e = maybe_move.entry((elf.0, elf.1 + 1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        for (k, v) in &maybe_move {
            if v.len() == 1 {
                assert!(!elves.contains(k));
                elves.remove(&v[0]);
                elves.insert(*k);
            }
        }
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for elf in &elves {
        min_x = min_x.min(elf.0);
        max_x = max_x.max(elf.0);
        min_y = min_y.min(elf.1);
        max_y = max_y.max(elf.1);
    }

    Ok((max_x - min_x + 1) as u32 * (max_y - min_y + 1) as u32 - elves.len() as u32)
}

pub fn part_two(input: Input) -> Result<u32> {
    let mut elves = HashSet::<(i32, i32)>::new();
    for (i, l) in input.as_str().lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    let mut i = 0;
    loop {
        // to, from
        let mut maybe_move = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();
        for elf in &elves {
            let mut occ = [false; 8];
            for (d, dir) in DIRS.iter().enumerate() {
                let pos = (elf.0 + dir.0, elf.1 + dir.1);
                if elves.contains(&pos) {
                    occ[d] = true;
                }
            }
            if occ.iter().all(|&o| !o) {
                continue;
            }

            for m in 0..4 {
                match (m + i) % 4 {
                    0 => {
                        if !occ[2] && !occ[1] && !occ[3] {
                            let e = maybe_move.entry((elf.0 - 1, elf.1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    1 => {
                        if !occ[6] && !occ[5] && !occ[7] {
                            let e = maybe_move.entry((elf.0 + 1, elf.1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    2 => {
                        if !occ[3] && !occ[4] && !occ[5] {
                            let e = maybe_move.entry((elf.0, elf.1 - 1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    3 => {
                        if !occ[0] && !occ[1] && !occ[7] {
                            let e = maybe_move.entry((elf.0, elf.1 + 1)).or_default();
                            e.push(*elf);
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        let mut moved = false;
        for (k, v) in &maybe_move {
            if v.len() == 1 {
                assert!(!elves.contains(k));
                elves.remove(&v[0]);
                elves.insert(*k);
                moved = true;
            }
        }

        if !moved {
            return Ok(i as u32 + 1);
        }

        i += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 110);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 20);
    }
}
