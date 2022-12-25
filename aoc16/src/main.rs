use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (valves, aa_id, _) = parse_input(&input)?;

    part1(&valves, aa_id)?;
    part2(&valves, aa_id)?;
    Ok(())
}

fn part1(valves: &[Valve], aa_id: usize) -> Result<usize> {
    let start = Instant::now();

    let mut memorization = vec![vec![usize::MAX; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        for j in 0..valves.len() {
            shortest_dis_bfs(&mut memorization, valves, i, j);
        }
    }
    let closed: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(i, _)| i)
        .collect();
    let mut memo = HashMap::new();
    let total_pressure = dp(
        &memorization,
        &closed,
        0,
        valves,
        aa_id,
        0,
        0,
        0,
        30,
        &mut memo,
    );

    writeln!(io::stdout(), "Part1: {total_pressure}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(total_pressure)
}

fn part2(valves: &[Valve], aa_id: usize) -> Result<usize> {
    let start = Instant::now();

    let mut memorization = vec![vec![usize::MAX; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        for j in 0..valves.len() {
            shortest_dis_bfs(&mut memorization, valves, i, j);
        }
    }
    let mut closed: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(i, _)| i)
        .collect();
    closed.sort_by(|&id1, &id2| valves[id2].flow_rate.cmp(&valves[id1].flow_rate));
    let mut memo: HashMap<((usize, usize), (usize, usize), u64), usize> = HashMap::new();
    let mut memo2: HashMap<(usize, usize, u64), usize> = HashMap::new();
    let total_pressure = dp_part2(
        &memorization,
        &closed,
        0,
        valves,
        (aa_id, aa_id),
        (0, 0),
        (0, 0),
        (0, 0),
        26,
        &mut memo,
        &mut memo2,
    );

    writeln!(io::stdout(), "Part2: {total_pressure}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(total_pressure)
}

fn shortest_dis_bfs(memorization: &mut [Vec<usize>], valves: &[Valve], start: usize, dest: usize) {
    if memorization[start][dest] != usize::MAX {
        return;
    }
    let mut visited = vec![false; valves.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut dis = 0;
    while !queue.is_empty() {
        let count = queue.len();
        for _ in 0..count {
            let cur = queue.pop_front().unwrap();
            visited[cur] = true;
            if cur == dest {
                memorization[start][dest] = dis;
                return;
            }
            for &next in &valves[cur].dest {
                if !visited[next] {
                    queue.push_back(next);
                }
            }
        }
        dis += 1;
    }
}

fn dp(
    memorization: &[Vec<usize>],
    closed: &[usize],
    opened: u64,
    valves: &[Valve],
    id: usize,
    total_pressure: usize,
    pressure: usize,
    time: usize,
    time_limit: usize,
    memo: &mut HashMap<(usize, usize, u64), usize>,
) -> usize {
    if let Some(&result) = memo.get(&(pressure, time, opened)) {
        return result;
    }
    if opened == closed.iter().fold(0, |bit, i| bit | (1 << i)) {
        return total_pressure + (time_limit - time) * pressure;
    }
    let result = closed
        .iter()
        .map(|&next| {
            if opened & (1 << next) == 0 {
                let mut new_opend = opened;
                new_opend |= 1 << next;
                let d = memorization[id][next] + 1;
                if time + d > time_limit {
                    total_pressure + (time_limit - time) * pressure
                } else {
                    dp(
                        memorization,
                        closed,
                        new_opend,
                        valves,
                        next,
                        total_pressure + pressure * d,
                        pressure + valves[next].flow_rate,
                        time + d,
                        time_limit,
                        memo,
                    )
                }
            } else {
                0
            }
        })
        .max()
        .unwrap();
    memo.insert((pressure, time, opened), result);
    result
}

fn dp_part2(
    memorization: &[Vec<usize>],
    closed: &[usize],
    opened: u64,
    valves: &[Valve],
    id: (usize, usize),
    total_pressure: (usize, usize),
    pressure: (usize, usize),
    time: (usize, usize),
    time_limit: usize,
    memo: &mut HashMap<((usize, usize), (usize, usize), u64), usize>,
    memo2: &mut HashMap<(usize, usize, u64), usize>,
) -> usize {
    if let Some(&result) = memo.get(&(pressure, time, opened)) {
        return result;
    }
    if opened == closed.iter().fold(0, |bit, i| bit | (1 << i)) {
        return total_pressure.0
            + total_pressure.1
            + (time_limit - time.0) * pressure.0
            + (time_limit - time.1) * pressure.1;
    }
    let mut result = 0;

    let l = closed.len();
    for id0 in 0..l {
        let next0 = closed[id0];
        for id1 in 0..l {
            let next1 = closed[id1];
            if next0 == next1 {
                continue;
            }
            if opened & (1 << next0) != 0 || opened & (1 << next1) != 0 {
                continue;
            }
            let d0 = memorization[id.0][next0] + 1;
            let d1 = memorization[id.1][next1] + 1;
            if time.0 + d0 <= time_limit && time.1 + d1 <= time_limit {
                let mut new_opened = opened;
                new_opened |= 1 << next0;
                new_opened |= 1 << next1;
                result = result.max(dp_part2(
                    memorization,
                    closed,
                    new_opened,
                    valves,
                    (next0, next1),
                    (
                        total_pressure.0 + d0 * pressure.0,
                        total_pressure.1 + d1 * pressure.1,
                    ),
                    (
                        pressure.0 + valves[next0].flow_rate,
                        pressure.1 + valves[next1].flow_rate,
                    ),
                    (time.0 + d0, time.1 + d1),
                    time_limit,
                    memo,
                    memo2,
                ));
            } else if d0 + time.0 > time_limit && d1 + time.1 > time_limit {
                result = result.max(
                    total_pressure.0
                        + total_pressure.1
                        + pressure.0 * (time_limit - time.0)
                        + pressure.1 * (time_limit - time.1),
                )
            } else {
                let (total_pressure, pressure, time, d0, next0) = if d0 + time.0 > d1 + time.1 {
                    (
                        rev_tuple(total_pressure),
                        rev_tuple(pressure),
                        rev_tuple(time),
                        d1,
                        next1,
                    )
                } else {
                    (total_pressure, pressure, time, d0, next0)
                };
                let mut new_opened = opened;
                new_opened |= 1 << next0;
                result = result.max(
                    total_pressure.1
                        + pressure.1 * (time_limit - time.1)
                        + dp(
                            memorization,
                            closed,
                            new_opened,
                            valves,
                            next0,
                            total_pressure.0 + pressure.0 * d0,
                            pressure.0 + valves[next0].flow_rate,
                            time.0 + d0,
                            time_limit,
                            memo2,
                        ),
                );
            }
        }
    }
    memo.insert((pressure, time, opened), result);
    memo.insert(((pressure.1, pressure.0), (time.1, time.0), opened), result);
    result
}

fn rev_tuple(t: (usize, usize)) -> (usize, usize) {
    (t.1, t.0)
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    dest: Vec<usize>,
}

impl Valve {
    fn new(flow_rate: usize, dest: Vec<usize>) -> Self {
        Self { flow_rate, dest }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Valve>, usize, HashMap<&str, usize>)> {
    fn get_id<'a>(
        name: &'a str,
        valves_index: &mut HashMap<&'a str, usize>,
        index: &mut usize,
    ) -> usize {
        if let Some(&id) = valves_index.get(name) {
            id
        } else {
            valves_index.insert(name, *index);
            *index += 1;
            *index - 1
        }
    }

    let mut valves_index: HashMap<&str, usize> = HashMap::new();
    let mut index = 0;
    let mut aa_id = 0;
    let mut valves: Vec<_> = (0..input.lines().count()).map(|_| None).collect();
    for line in input.lines() {
        if let Some((part1, part2)) = line.trim().split_once("; ") {
            if let Some((name, rate)) = part1.split_once(" has flow rate=") {
                if let Some(name) = name.strip_prefix("Valve ") {
                    let id = get_id(name, &mut valves_index, &mut index);
                    valves_index.insert(name, id);
                    if name == "AA" {
                        aa_id = id;
                    }
                    let rate: usize = rate.parse()?;
                    let dest: Vec<_> =
                        if let Some(dest) = part2.strip_prefix("tunnels lead to valves ") {
                            dest.split(", ").collect()
                        } else if let Some(dest) = part2.strip_prefix("tunnel leads to valve ") {
                            dest.split(", ").collect()
                        } else {
                            vec![]
                        };
                    let dest: Vec<_> = dest
                        .iter()
                        .map(|n| get_id(n, &mut valves_index, &mut index))
                        .collect();
                    valves[id] = Some(Valve::new(rate, dest));
                }
            }
        }
    }
    if valves.iter().all(|v| v.is_some()) {
        return Ok((valves.into_iter().flatten().collect(), aa_id, valves_index));
    }
    err!("not a valid input")
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_input() {
        use crate::*;
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II";
        let (valves, aa_id, valves_index) = parse_input(input).unwrap();
        println!("{:?}", valves_index);
        assert_eq!(aa_id, 0);
        assert_eq!(valves.len(), input.lines().count());
        assert_eq!(part1(&valves, aa_id).unwrap(), 1651);
        assert_eq!(part2(&valves, aa_id).unwrap(), 1707);
    }

    #[test]
    fn test_shortest_path() {
        use crate::*;
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB, CC, DD
        Valve BB has flow rate=13; tunnels lead to valves CC, DD
        Valve CC has flow rate=2; tunnels lead to valves DD
        Valve DD has flow rate=20; tunnels lead to valves FF
        Valve FF has flow rate=20; tunnels lead to valves FF";
        let (valves, _, all_index) = parse_input(input).unwrap();
        dbg!(all_index);
        let mut memorization = vec![vec![usize::MAX; valves.len()]; valves.len()];
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                shortest_dis_bfs(&mut memorization, &valves, i, j);
            }
        }
        assert_eq!(memorization[0][4], 2);
        assert_eq!(memorization[0][3], 1);
        assert_eq!(memorization[0][2], 1);
        assert_eq!(memorization[0][1], 1);
        assert_eq!(memorization[0][0], 0);
        assert_eq!(memorization[1][4], 2);
    }
}
