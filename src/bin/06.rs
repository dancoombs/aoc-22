use std::collections::{hash_map::Entry, HashMap};

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};

pub fn part_one(input: Input) -> Result<u32> {
    distinct_window_array(input, 4)
}

pub fn part_two(input: Input) -> Result<u32> {
    distinct_window_array(input, 14)
}

fn byte_to_idx(i: u8) -> usize {
    (i - b'a') as usize
}

// 2nd attempt after seeing answers, way faster
fn distinct_window_array(input: Input, window_len: usize) -> Result<u32> {
    let bytes = input.as_bytes();
    let mut window_counts = [0_u8; 26];
    let mut unique_count = 0;

    for i in 0..bytes.len() {
        let add_idx = byte_to_idx(bytes[i]);
        window_counts[add_idx] += 1;
        if window_counts[add_idx] == 1 {
            unique_count += 1;
        }

        if i >= window_len {
            let rm_idx = byte_to_idx(bytes[i - window_len]);
            window_counts[rm_idx] -= 1;
            if window_counts[rm_idx] == 0 {
                unique_count -= 1;
            }
        }

        if unique_count == window_len {
            return Ok((i + 1) as u32);
        }
    }

    Err(anyhow!("Not found"))
}

fn _distinct_window_map(input: Input, window_len: usize) -> Result<u32> {
    let bytes = input.as_bytes();
    let mut window_vals = HashMap::new();

    for i in 0..bytes.len() {
        window_vals
            .entry(bytes[i])
            .and_modify(|e| *e += 1)
            .or_insert(1);

        if i >= window_len {
            if let Entry::Occupied(e) = window_vals
                .entry(bytes[i - window_len])
                .and_modify(|e| *e -= 1)
            {
                if *e.get() == 0 {
                    e.remove_entry();
                }
            }
        }

        if window_vals.len() == window_len {
            return Ok((i + 1) as u32);
        }
    }

    Err(anyhow!("Not found"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 7);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 19);
    }
}
