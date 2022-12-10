use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn part_one(input: Input) -> Result<i32> {
    let program = parse_insts(input)?;
    let mut cntr = 0_usize;
    let mut x = 1_i32;
    let mut signal_values = vec![];

    for inst in program {
        cntr += 1;
        capture_signals(&mut signal_values, cntr, x);
        match inst {
            Opcode::Nop => {}
            Opcode::Addx(v) => {
                cntr += 1;
                capture_signals(&mut signal_values, cntr, x);
                x += v;
            }
        }
    }

    Ok(signal_values.iter().sum())
}

pub fn part_two(input: Input) -> Result<i32> {
    let program = parse_insts(input)?;
    let mut cntr = 0_usize;
    let mut x = 1_i32;
    let mut screen = vec![['.'; 40]; 1];

    for inst in program {
        cntr += 1;
        update_screen(&mut screen, cntr, x);

        match inst {
            Opcode::Nop => {}
            Opcode::Addx(v) => {
                cntr += 1;
                update_screen(&mut screen, cntr, x);
                x += v;
            }
        }
    }

    for row in screen {
        println!("{}", row.iter().join(""));
    }

    Ok(0)
}

enum Opcode {
    Nop,
    Addx(i32),
}

impl Opcode {
    fn parse(s: &str) -> Result<Self> {
        let parts = s.split_whitespace().collect_vec();
        match parts[0] {
            "noop" => Ok(Opcode::Nop),
            "addx" => {
                let x = parts[1].parse()?;
                Ok(Opcode::Addx(x))
            }
            _ => Err(anyhow!("Unknown instruction: {}", parts[0])),
        }
    }
}

fn parse_insts(input: Input) -> Result<Vec<Opcode>> {
    input.as_str().lines().map(Opcode::parse).collect()
}

fn capture_signals(signal_values: &mut Vec<i32>, cntr: usize, x: i32) {
    if cntr == 20 + 40 * signal_values.len() {
        signal_values.push(x * cntr as i32);
    }
}

fn update_screen(screen: &mut Vec<[char; 40]>, cntr: usize, x: i32) {
    let pos = (cntr - 1) % 40;
    if (pos as i32 - x).abs() < 2 {
        while screen.len() < (cntr - 1) / 40 + 1 {
            screen.push(['.'; 40]);
        }

        screen[(cntr - 1) / 40][pos] = '#';
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 13140);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert!(part_two(Input::new(&input)).is_ok());
    }
}
