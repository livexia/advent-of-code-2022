use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Key = (u16, u16, u16, u16, u16, u16, u16, u16, u16); // 9 u16, with time

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse()).collect::<Result<_>>()?;

    part1(&mut blueprints)?;
    part2(&mut blueprints)?;
    Ok(())
}

fn part1(blueprints: &mut [Blueprint]) -> Result<u16> {
    let start = Instant::now();
    let mut result = 0;

    for b in blueprints {
        result += b.execute(0, State::new(), &mut HashSet::new(), 24) * b.id;
    }
    writeln!(io::stdout(), "Part1: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(blueprints: &mut [Blueprint]) -> Result<u16> {
    let start = Instant::now();

    let l = blueprints.len();
    let result = blueprints[..3.min(l)]
        .iter_mut()
        .map(|b| b.execute(0, State::new(), &mut HashSet::new(), 32))
        .product();
    writeln!(io::stdout(), "Part2: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: u16,
    ore_cost: u16,
    clay_cost: u16,
    obsidian_cost: (u16, u16),
    geode_cost: (u16, u16),
    max_ore: u16,
    max_clay: u16,
    max_obsidian: u16,
    max_geodes: u16,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    robots: [u16; 4],
    goods: [u16; 4],
}

impl State {
    fn new() -> Self {
        State {
            robots: [1, 0, 0, 0],
            goods: [0, 0, 0, 0],
        }
    }

    fn collect(&mut self) {
        for (index, count) in self.robots.iter().enumerate() {
            self.goods[index] += count;
        }
    }

    fn key(&self, time: u16) -> Key {
        (
            self.robots[0],
            self.robots[1],
            self.robots[2],
            self.robots[3],
            self.goods[0],
            self.goods[1],
            self.goods[2],
            self.goods[3],
            time,
        )
    }
}

impl Blueprint {
    fn execute(
        &mut self,
        time: u16,
        mut state: State,
        memo: &mut HashSet<Key>,
        time_limit: u16,
    ) -> u16 {
        if time == time_limit {
            self.max_geodes = self.max_geodes.max(state.goods[3]);
            return state.goods[3];
        }
        let mut p_g = state.goods[3];
        for i in 1..(time_limit - time + 1) {
            // max geode robot count is time_limit - time
            p_g += state.robots[3] + i - 1
        }
        if p_g <= self.max_geodes {
            return 0;
        }
        state = self.trim_goods(state, time, time_limit);
        let key = state.key(time);
        memo.insert(key);
        let (r1, r2, r3, r4) = (
            self.can_build_geode_robot(&state),
            self.can_build_obsidian_robot(&state),
            self.can_build_clay_robot(&state),
            self.can_build_ore_robot(&state),
        ); // dry-run before collect
        state.collect();
        let mut new_state = vec![];
        if r1 {
            new_state.push(self.build_geode_robot(&state))
        } else {
            if r2 && state.robots[2] < self.max_obsidian {
                new_state.push(self.build_obsidian_robot(&state))
            }
            if r3 && state.robots[1] < self.max_clay {
                new_state.push(self.build_clay_robot(&state))
            }
            if r4 && state.robots[0] < self.max_ore {
                new_state.push(self.build_ore_robot(&state))
            }
            new_state.push(state);
        }
        let r = new_state
            .into_iter()
            .map(|s| {
                let key = s.key(time + 1);
                if memo.contains(&key) {
                    0
                } else {
                    self.execute(time + 1, s, memo, time_limit)
                }
            })
            .max()
            .unwrap();
        self.max_geodes = self.max_geodes.max(r);
        r
    }

    fn trim_goods(&self, mut state: State, time: u16, time_limit: u16) -> State {
        state.goods[0] = state.goods[0].min((time_limit - time) * self.max_ore);
        state.goods[1] = state.goods[1].min((time_limit - time) * self.max_clay);
        state.goods[2] = state.goods[2].min((time_limit - time) * self.max_obsidian);
        state
    }

    fn build_geode_robot(&self, state: &State) -> State {
        let mut state = *state;
        state.goods[0] -= self.geode_cost.0;
        state.goods[2] -= self.geode_cost.1;
        state.robots[3] += 1;
        state
    }

    fn build_obsidian_robot(&self, state: &State) -> State {
        let mut state = *state;
        state.goods[0] -= self.obsidian_cost.0;
        state.goods[1] -= self.obsidian_cost.1;
        state.robots[2] += 1;
        state
    }

    fn build_clay_robot(&self, state: &State) -> State {
        let mut state = *state;
        state.goods[0] -= self.clay_cost;
        state.robots[1] += 1;
        state
    }

    fn build_ore_robot(&self, state: &State) -> State {
        let mut state = *state;
        state.goods[0] -= self.ore_cost;
        state.robots[0] += 1;
        state
    }

    fn can_build_geode_robot(&self, state: &State) -> bool {
        state.goods[0] >= self.geode_cost.0 && state.goods[2] >= self.geode_cost.1
    }

    fn can_build_obsidian_robot(&self, state: &State) -> bool {
        state.goods[0] >= self.obsidian_cost.0 && state.goods[1] >= self.obsidian_cost.1
    }

    fn can_build_clay_robot(&self, state: &State) -> bool {
        state.goods[0] >= self.clay_cost
    }

    fn can_build_ore_robot(&self, state: &State) -> bool {
        state.goods[0] >= self.ore_cost
    }
}

impl FromStr for Blueprint {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let r: Vec<_> = s
            .trim()
            .split([' ', ':'])
            .filter_map(|w| w.parse::<u16>().ok())
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
                max_ore: r[1].max(r[2]).max(r[3]).max(r[4]),
                max_clay: r[4],
                max_obsidian: r[6],
                max_geodes: 0,
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
        let mut blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse().unwrap()).collect();
        // assert_eq!(part1(&blueprints[..1]).unwrap(), 9);
        // assert_eq!(part1(&blueprints[1..]).unwrap(), 24);
        assert_eq!(part1(&mut blueprints).unwrap(), 33);
        assert_eq!(part2(&mut blueprints[..1]).unwrap(), 56);
        assert_eq!(part2(&mut blueprints[1..]).unwrap(), 62);
    }
}
