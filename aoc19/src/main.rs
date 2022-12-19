use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::os::macos::raw::stat;
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Key = (
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse()).collect::<Result<_>>()?;

    part1(&blueprints)?;
    // part2()?;
    Ok(())
}

fn part1(blueprints: &[Blueprint]) -> Result<usize> {
    let start = Instant::now();
    let mut result = 0;

    for b in &blueprints[1..] {
        println!("{:?}", b);
        result += b.execute(0, Status::new(), &mut HashSet::new()) * b.id;

        println!("{:?}", b);
        println!("{:?}", result);
    }
    writeln!(io::stdout(), "Part1: {:?}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Status {
    robot_count: [usize; 4],
    goods: [usize; 4],
}

impl Status {
    fn new() -> Self {
        Status {
            robot_count: [1, 0, 0, 0],
            goods: [0, 0, 0, 0],
        }
    }

    fn collect(&mut self) {
        for (index, count) in self.robot_count.iter().enumerate() {
            self.goods[index] += count;
        }
    }

    fn to_key(&self, time: usize) -> Vec<usize> {
        let mut r = [self.robot_count, self.goods].concat();
        r.push(time);
        r
    }
}

impl Blueprint {
    fn execute(
        &self,
        time: usize,
        mut status: Status,
        memo: &mut HashSet<(Status, usize)>,
    ) -> usize {
        memo.insert((status.clone(), time));
        if time == 24 {
            return status.goods[3];
        }
        let (r1, r2, r3, r4) = (
            self.can_build_geode_robot(&status),
            self.can_build_obsidian_robot(&status),
            self.can_build_clay_robot(&status),
            self.can_build_ore_robot(&status),
        ); // dry-run before collect
        status.collect();
        let mut new_status = vec![];
        if r1 {
            new_status.push(self.build_geode_robot(&status))
        }
        if r2 {
            new_status.push(self.build_obsidian_robot(&status))
        }
        if r3 {
            new_status.push(self.build_clay_robot(&status))
        }
        if r4 {
            new_status.push(self.build_ore_robot(&status))
        }
        new_status.push(status);
        new_status
            .into_iter()
            .map(|s| {
                if !memo.contains(&(s, time + 1)) {
                    self.execute(time + 1, s, memo)
                } else {
                    0
                }
            })
            .max()
            .unwrap()
    }

    fn build_geode_robot(&self, status: &Status) -> Status {
        let mut status = status.clone();
        status.goods[0] -= self.geode_cost.0;
        status.goods[2] -= self.geode_cost.1;
        status.robot_count[3] += 1;
        status
    }

    fn build_obsidian_robot(&self, status: &Status) -> Status {
        let mut status = status.clone();
        status.goods[0] -= self.obsidian_cost.0;
        status.goods[1] -= self.obsidian_cost.1;
        status.robot_count[2] += 1;
        status
    }

    fn build_clay_robot(&self, status: &Status) -> Status {
        let mut status = status.clone();
        status.goods[0] -= self.clay_cost;
        status.robot_count[1] += 1;
        status
    }

    fn build_ore_robot(&self, status: &Status) -> Status {
        let mut status = status.clone();
        status.goods[0] -= self.ore_cost;
        status.robot_count[0] += 1;
        status
    }

    fn can_build_geode_robot(&self, status: &Status) -> bool {
        status.goods[0] >= self.geode_cost.0 && status.goods[2] >= self.geode_cost.1
    }

    fn can_build_obsidian_robot(&self, status: &Status) -> bool {
        status.goods[0] >= self.obsidian_cost.0 && status.goods[1] >= self.obsidian_cost.1
    }

    fn can_build_clay_robot(&self, status: &Status) -> bool {
        status.goods[0] >= self.clay_cost
    }

    fn can_build_ore_robot(&self, status: &Status) -> bool {
        status.goods[0] >= self.ore_cost
    }
}

impl FromStr for Blueprint {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let r: Vec<_> = s
            .trim()
            .split([' ', ':'])
            .filter_map(|w| w.parse::<usize>().ok())
            .collect();
        if r.len() != 7 {
            err!("input is not a valid blueprint: {}", s)
        } else {
            Ok(Self {
                id: r[0],
                ore_cost: r[1],
                clay_cost: r[2],
                obsidian_cost: (r[3], r[4]),
                geode_cost: (r[5], r[6]),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example_input() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(part1(&blueprints).unwrap(), 33);
    }
}
