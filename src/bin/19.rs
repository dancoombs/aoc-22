use std::{collections::VecDeque, str::FromStr};

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Result};

// most of this is from dphil's solution
// https://github.com/dphilipson/advent-of-code-2022/blob/master/src/days/day19.rs
// Just worked through it on my own for understanding

pub fn part_one(input: Input) -> Result<u32> {
    Ok(input
        .as_str()
        .lines()
        .map(|line| {
            let bp = line.parse::<Blueprint>()?;
            let score = score_blueprint(&bp, 24)?;
            Ok(bp.id * score)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum())
}

pub fn part_two(input: Input) -> Result<u32> {
    Ok(input
        .as_str()
        .lines()
        .take(3)
        .map(|line| {
            let bp = line.parse::<Blueprint>()?;
            let score = score_blueprint(&bp, 32)?;
            Ok(score)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .product())
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: u32,
    costs: [[u32; 4]; 4],
}

#[derive(Clone, Debug)]
struct State {
    time: u32,
    robots: [u32; 4],
    resources: [u32; 4],
}

impl State {
    fn robot_wait(&self, costs: &[u32; 4]) -> Option<u32> {
        let mut wait = 0;
        for (r, c) in costs.iter().enumerate() {
            let need = c.saturating_sub(self.resources[r]);
            if need == 0 {
                continue;
            }
            if self.robots[r] == 0 {
                return None;
            }
            wait = wait.max((need + self.robots[r] - 1) / self.robots[r]);
        }

        Some(wait + 1) // one cycle to build robot
    }

    // ty dphil
    fn best_possible(&self, time_limit: u32) -> u32 {
        let rem = time_limit - self.time;
        // cur +
        self.resources[3] + self.robots[3] * rem + rem * (rem + 1) / 2
    }
}

// ty dphil
fn max_robots_needed(costs: &[[u32; 4]; 4]) -> [u32; 4] {
    let mut out = [0; 4];
    out[3] = u32::MAX;
    for i in 0..3 {
        let max = costs.iter().map(|c| c[i]).max().unwrap();
        out[i] = max;
    }
    out
}

fn score_blueprint(bp: &Blueprint, time_limit: u32) -> Result<u32> {
    let mut best_score = 0;
    let mut q = VecDeque::from([State {
        time: 0,
        robots: [1, 0, 0, 0],
        resources: [0; 4],
    }]);

    let max_robots = max_robots_needed(&bp.costs);

    while let Some(state) = q.pop_front() {
        if state.best_possible(time_limit) <= best_score {
            continue;
        }

        // try to build each robot type
        #[allow(clippy::needless_range_loop)]
        for robot in 0..4 {
            if state.robots[robot] >= max_robots[robot] {
                continue;
            }

            // determine how long we need to wait for the resources
            if let Some(wait) = state.robot_wait(&bp.costs[robot]) {
                if state.time + wait > time_limit {
                    continue;
                }

                // calculate new state after waiting
                let mut new_state = state.clone();
                new_state.time += wait; // time to wait for resources
                for resource in 0..4 {
                    new_state.resources[resource] += wait * new_state.robots[resource]; // accrue during wait
                    new_state.resources[resource] -= bp.costs[robot][resource]; // cost to build
                }
                new_state.robots[robot] += 1;

                // calculate best score this config of robots can achieve by doing nothing
                best_score = best_score.max(
                    new_state.resources[3] + new_state.robots[3] * (time_limit - new_state.time),
                );

                q.push_back(new_state);
            }
        }
    }

    Ok(best_score)
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = sscanf::sscanf!(s, "Blueprint {u32}: Each ore robot costs {u32} ore. Each clay robot costs {u32} ore. Each obsidian robot costs {u32} ore and {u32} clay. Each geode robot costs {u32} ore and {u32} obsidian.");
        match parsed {
            Ok(parsed) => Ok(Self {
                id: parsed.0,
                costs: [
                    [parsed.1, 0, 0, 0],
                    [parsed.2, 0, 0, 0],
                    [parsed.3, parsed.4, 0, 0],
                    [parsed.5, 0, parsed.6, 0],
                ],
            }),
            Err(_) => Err(anyhow!("Faliure to parse blueprint: {}", s)),
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 33);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 3472);
    }
}
