use std::collections::HashMap;

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};
use itertools::Itertools;

/*
Rocks:

####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
*/

const ROCKS: [[[u8; 4]; 4]; 5] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [1, 1, 1, 0]],
    [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0]],
];

const WIDTH: usize = 7;
const START_LEFT: usize = 2;
const START_ABOVE: usize = 3;
const DROPS: usize = 2022;

pub fn part_one(input: Input) -> Result<usize> {
    let jets: Vec<char> = input.as_str().trim().chars().collect();
    let mut chamber = vec![[0_u8; WIDTH]; START_ABOVE];
    let mut jet_idx = 0;
    let mut rock_iter = ROCKS.iter().cycle();

    for _ in 0..DROPS {
        let rock = rock_iter.next().unwrap();
        drop_rock(&mut chamber, rock, &jets, &mut jet_idx)?;
    }

    Ok(top(&chamber))
}

pub fn part_two(input: Input) -> Result<usize> {
    let jets: Vec<char> = input.as_str().trim().chars().collect();
    let cycle = find_cycle(&jets)?;

    let target: usize = 1000000000000;

    let rem = target - cycle.cycle_start.drops;
    let cycles = rem / cycle.cycle_drops;
    let rem = rem % cycle.cycle_drops;

    // simulate one cycle plus remainder
    let mut chamber = vec![[0_u8; WIDTH]; START_ABOVE];
    let mut jet_idx = 0;
    let mut rock_iter = ROCKS.iter().cycle();
    for _ in 0..(cycle.cycle_start.drops + rem) {
        let rock = rock_iter.next().unwrap();
        drop_rock(&mut chamber, rock, &jets, &mut jet_idx)?;
    }

    Ok(top(&chamber) + cycles * cycle.cycle_height)
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct CycleKey {
    rock_idx: usize,
    jet_idx: usize,
    snapshot: u128,
}

#[derive(Debug, Clone)]
struct CycleValue {
    drops: usize,
    height: usize,
}

#[derive(Debug)]
struct Cycle {
    cycle_height: usize,
    cycle_drops: usize,
    cycle_start: CycleValue,
}

fn find_cycle(jets: &Vec<char>) -> Result<Cycle> {
    let mut chamber = vec![[0_u8; WIDTH]; START_ABOVE];
    let mut rock_count = 0;
    let mut jet_idx = 0;

    let mut cycle_map = HashMap::<CycleKey, CycleValue>::new();

    loop {
        chamber.resize(top(&chamber), [0; WIDTH]);
        if let Some(snapshot) = snapshot_chamber(&chamber) {
            let key = CycleKey {
                rock_idx: rock_count % ROCKS.len(),
                jet_idx,
                snapshot,
            };
            let value = CycleValue {
                drops: rock_count,
                height: top(&chamber),
            };
            if let Some(old_value) = cycle_map.get(&key) {
                return Ok(Cycle {
                    cycle_height: value.height - old_value.height,
                    cycle_drops: value.drops - old_value.drops,
                    cycle_start: old_value.clone(),
                });
            }
            cycle_map.insert(key, value);
        }

        drop_rock(
            &mut chamber,
            &ROCKS[rock_count % ROCKS.len()],
            jets,
            &mut jet_idx,
        )?;
        rock_count += 1;
    }
}

fn snapshot_chamber(chamber: &[[u8; WIDTH]]) -> Option<u128> {
    if chamber.len() < 20 {
        return None;
    }

    let mut snapshot = 0_u128;
    for i in 0..128 {
        snapshot |= (chamber[chamber.len() - 1 - (i / WIDTH)][i % WIDTH] as u128) << i;
    }

    Some(snapshot)
}

fn drop_rock(
    chamber: &mut Vec<[u8; WIDTH]>,
    rock: &[[u8; 4]; 4],
    jets: &Vec<char>,
    jet_idx: &mut usize,
) -> Result<()> {
    // add rock to chamber
    let mut vert_off = top(chamber) + START_ABOVE;
    chamber.resize(vert_off + rock.len(), [0; WIDTH]);
    for (row_idx, row) in rock.iter().rev().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            chamber[vert_off + row_idx][col_idx + START_LEFT] = if *col == 1 { 2 } else { 0 };
        }
    }

    loop {
        // push left or right
        let jet = jets[*jet_idx];
        *jet_idx = (*jet_idx + 1) % jets.len();
        match jet {
            '>' => {
                if chamber[vert_off..vert_off + 4]
                    .iter()
                    .map(can_shift_right)
                    .all(|x| x)
                {
                    chamber[vert_off..vert_off + 4]
                        .iter_mut()
                        .for_each(shift_right);
                }
            }
            '<' => {
                if chamber[vert_off..vert_off + 4]
                    .iter()
                    .map(can_shift_left)
                    .all(|x| x)
                {
                    chamber[vert_off..vert_off + 4]
                        .iter_mut()
                        .for_each(shift_left);
                }
            }
            _ => Err(anyhow!("Invalid jet: {}", jet))?,
        };

        // check if stuck
        if vert_off == 0 || (0..WIDTH).any(|col| !can_shift_down(chamber, col, vert_off)) {
            for row in chamber[vert_off..vert_off + 4].iter_mut() {
                for c in row {
                    if *c == 2 {
                        *c = 1;
                    }
                }
            }

            break;
        }

        // fall down
        for col in 0..WIDTH {
            shift_down(chamber, col, vert_off);
        }
        vert_off -= 1;
    }

    Ok(())
}

fn top(chamber: &[[u8; WIDTH]]) -> usize {
    match chamber.iter().rev().position(|row| row.contains(&1)) {
        Some(x) => chamber.len() - x,
        None => 0,
    }
}

fn can_shift_left(row: &[u8; WIDTH]) -> bool {
    if row[0] == 2 {
        return false;
    }

    for i in 0..WIDTH - 1 {
        if row[i] == 1 && row[i + 1] == 2 {
            return false;
        }
    }

    true
}

fn shift_left(row: &mut [u8; WIDTH]) {
    for i in 1..WIDTH {
        if row[i] == 2 {
            row[i - 1] = 2;
            row[i] = 0;
        }
    }
}

fn can_shift_right(row: &[u8; WIDTH]) -> bool {
    if row[WIDTH - 1] == 2 {
        return false;
    }

    for i in (1..WIDTH).rev() {
        if row[i] == 1 && row[i - 1] == 2 {
            return false;
        }
    }

    true
}

fn shift_right(row: &mut [u8; WIDTH]) {
    for i in (0..WIDTH - 1).rev() {
        if row[i] == 2 {
            row[i + 1] = 2;
            row[i] = 0;
        }
    }
}

fn can_shift_down(chamber: &[[u8; WIDTH]], col: usize, vert_off: usize) -> bool {
    for i in vert_off..chamber.len() {
        if chamber[i][col] == 2 && chamber[i - 1][col] == 1 {
            return false;
        }
    }
    true
}

fn shift_down(chamber: &mut [[u8; WIDTH]], col: usize, vert_off: usize) {
    for i in vert_off..chamber.len() {
        if chamber[i][col] == 2 {
            chamber[i - 1][col] = 2;
            chamber[i][col] = 0;
        }
    }
}

fn _print_chamber(chamber: &[[u8; WIDTH]]) {
    for row in chamber.iter().rev() {
        let p = row
            .iter()
            .map(|c| match c {
                0 => '.',
                1 => '#',
                2 => '@',
                _ => '?',
            })
            .join("");
        println!("{p}");
    }
    println!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 3068);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 1514285714288);
    }
}
