use std::{cmp, collections::HashMap};

use advent_of_code::helpers::Input;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: Input) -> Result<u32> {
    let valves = parse(input)?;
    let v_map = valves
        .into_iter()
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let on = 0_u64;
    let mut memo = HashMap::<Memo, u32>::new();

    Ok(dfs(&v_map, "AA", 30, on, &mut memo))
}

pub fn part_two(input: Input) -> Result<u32> {
    let valves = parse(input)?;
    let v_map = valves
        .into_iter()
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let on = 0_u64;
    let mut memo = HashMap::<Memo2, u32>::new();
    let distances = calc_distances(&v_map);

    Ok(dfs_2(&mut Dfs2 {
        valves: &v_map,
        distances: &distances,
        valve1: "AA",
        time1: 26,
        valve2: "AA",
        time2: 26,
        on,
        memo: &mut memo,
    }))
}

fn is_set(n: u64, i: u64) -> bool {
    n & (1 << i) != 0
}

fn set_bit(n: u64, i: u64) -> u64 {
    n | (1 << i)
}

#[derive(Eq, Hash, PartialEq)]
struct Memo {
    cur: String,
    time: u32,
    on: u64,
}

fn dfs(
    valves: &HashMap<String, Valve>,
    cur: &str,
    time: u32,
    on: u64,
    memo: &mut HashMap<Memo, u32>,
) -> u32 {
    if time == 0 {
        return 0;
    }
    let key = Memo {
        cur: cur.to_string(),
        time,
        on,
    };
    if let Some(score) = memo.get(&key) {
        return *score;
    }

    let valve = valves.get(cur).unwrap();
    let mut max_score = 0_u32;

    for n in &valve.tunnels {
        max_score = max_score.max(dfs(valves, n, time - 1, on, memo));
    }
    if valve.rate > 0 && !is_set(on, valve.index) {
        let on = set_bit(on, valve.index);
        max_score = max_score.max(valve.rate * (time - 1) + dfs(valves, cur, time - 1, on, memo));
    }
    memo.insert(key, max_score);

    max_score
}

#[derive(Eq, Hash, PartialEq)]
struct Memo2 {
    valve1: String,
    time1: u32,
    valve2: String,
    time2: u32,
    on: u64,
}

struct Dfs2<'a> {
    valves: &'a HashMap<String, Valve>,
    distances: &'a Vec<Vec<u32>>,
    valve1: &'a str,
    time1: u32,
    valve2: &'a str,
    time2: u32,
    on: u64,
    memo: &'a mut HashMap<Memo2, u32>,
}

fn dfs_2(i: &mut Dfs2) -> u32 {
    if i.time1 == 0 && i.time2 == 0 {
        return 0;
    }
    let key = Memo2 {
        valve1: i.valve1.to_string(),
        time1: i.time1,
        valve2: i.valve2.to_string(),
        time2: i.time2,
        on: i.on,
    };
    if let Some(score) = i.memo.get(&key) {
        return *score;
    }
    let mut max_score = 0_u32;

    let v1 = i.valves.get(&i.valve1.to_string()).unwrap();
    let v2 = i.valves.get(&i.valve2.to_string()).unwrap();
    let dists1 = &i.distances[v1.index as usize];
    let dists2 = &i.distances[v2.index as usize];

    for v in i.valves.values() {
        if v.rate == 0 || is_set(i.on, v.index) {
            continue;
        }

        let new_on = set_bit(i.on, v.index);

        let v_dist1 = dists1[v.index as usize];
        if v_dist1 < u32::MAX && i.time1 as i32 - v_dist1 as i32 >= 1 {
            let time_left = i.time1 - v_dist1 - 1;
            let search_score = dfs_2(&mut Dfs2 {
                valves: i.valves,
                distances: i.distances,
                valve1: v.name.as_str(),
                time1: time_left,
                valve2: i.valve2,
                time2: i.time2,
                on: new_on,
                memo: i.memo,
            });
            max_score = max_score.max(search_score + v.rate * time_left);
        }

        let v_dist2 = dists2[v.index as usize];
        if v_dist2 < u32::MAX && i.time2 as i32 - v_dist2 as i32 >= 1 {
            let time_left = i.time2 - v_dist2 - 1;
            let search_score = dfs_2(&mut Dfs2 {
                valves: i.valves,
                distances: i.distances,
                valve1: i.valve1,
                time1: i.time1,
                valve2: v.name.as_str(),
                time2: time_left,
                on: new_on,
                memo: i.memo,
            });
            max_score = max_score.max(search_score + v.rate * time_left);
        }
    }

    i.memo.insert(key, max_score);
    max_score
}

fn calc_distances(valves: &HashMap<String, Valve>) -> Vec<Vec<u32>> {
    let mut distances = vec![vec![std::u32::MAX; valves.len()]; valves.len()];
    for v in valves.values() {
        for t in &v.tunnels {
            let t = valves.get(t).unwrap();
            distances[v.index as usize][t.index as usize] = 1;
        }
        distances[v.index as usize][v.index as usize] = 0;
    }
    let len = valves.len();
    for i in 0..len {
        for j in 0..len {
            for k in 0..len {
                if distances[j][i] < std::u32::MAX && distances[i][k] < std::u32::MAX {
                    distances[j][k] = cmp::min(distances[j][k], distances[j][i] + distances[i][k]);
                }
            }
        }
    }

    distances
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    index: u64,
    rate: u32,
    tunnels: Vec<String>,
}

fn parse_line(line: &str, index: u64) -> Result<Valve> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Valve (?P<name>\w+) has flow rate=(?P<rate>\d+); tunnel[s]? lead[s]? to valve[s]? (?P<tunnels>.*$)").unwrap();
    }

    let caps = RE.captures(line).context("invalid input, not a match")?;
    let name = caps
        .name("name")
        .context("invalid input, no name")?
        .as_str()
        .to_string();
    let rate = caps
        .name("rate")
        .context("invalid input, no rate")?
        .as_str()
        .parse::<u32>()?;
    let tunnels = caps
        .name("tunnels")
        .context("invalid input, no tunnels")?
        .as_str()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    Ok(Valve {
        name,
        index,
        rate,
        tunnels,
    })
}

fn parse(input: Input) -> Result<Vec<Valve>> {
    input
        .as_str()
        .lines()
        .enumerate()
        .map(|(i, l)| parse_line(l, i as u64))
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 1651);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 1707);
    }
}
