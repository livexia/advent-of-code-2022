use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut map: Map = input.parse().unwrap();
    let (start, end) = map.start_and_end();
    let map_cache = build_map_cache(&mut map);

    part1(&map_cache, start, end)?;
    part2(&map_cache, start, end)?;
    Ok(())
}

fn part1(map_cache: &[Map], start_pos: (usize, usize), end_pos: (usize, usize)) -> Result<usize> {
    let start = Instant::now();

    let find_time = avoid_blizzards(map_cache, start_pos, end_pos, 0);
    writeln!(io::stdout(), "Part 1: {find_time}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(find_time)
}

fn part2(map_cache: &[Map], start_pos: (usize, usize), end_pos: (usize, usize)) -> Result<usize> {
    let start = Instant::now();

    let find_time = avoid_blizzards(map_cache, start_pos, end_pos, 0);
    let find_time = avoid_blizzards(map_cache, end_pos, start_pos, find_time);
    let find_time = avoid_blizzards(map_cache, start_pos, end_pos, find_time);
    writeln!(io::stdout(), "Part 2: {find_time}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(find_time)
}

fn build_map_cache(map: &mut Map) -> Vec<Map> {
    let mut cache = vec![];
    let cycle_count = lcm(map.height - 2, map.width - 2);
    for _ in 0..cycle_count {
        cache.push(map.clone());
        map.next();
    }
    cache
}

fn lcm(small: usize, big: usize) -> usize {
    let mut m = 1;
    while m < small {
        if m * big % small == 0 {
            return m * big;
        }
        m += 1;
    }
    small * big
}

fn avoid_blizzards(
    map_cache: &[Map],
    start: (usize, usize),
    end: (usize, usize),
    start_time: usize,
) -> usize {
    let cycle = map_cache.len();

    let mut queue = VecDeque::new(); // cur position and minutes
    queue.push_back((start, start_time + 1));
    let mut find_time = 0;
    let mut visited = HashSet::new();
    'find: while let Some((cur, time)) = queue.pop_front() {
        let cycle_time = time % cycle;
        if visited.insert((cur, cycle_time)) {
            let map = &map_cache[cycle_time];
            let (x, y) = cur;
            for next in [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y + 1),
                (x, y.saturating_sub(1)),
            ] {
                if next == end {
                    find_time = time;
                    break 'find;
                } else if map.moveable(next.0, next.1) {
                    queue.push_back((next, time + 1));
                }
            }
            if map.moveable(cur.0, cur.1) || cur == start {
                queue.push_back((cur, time + 1))
            }
        }
    }
    find_time
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Map {
    blizzards: Vec<[u128; 4]>, // 0 up, 1 down, 2 right, 3 left
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn next(&mut self) {
        let mut next_blizzards = vec![[0; 4]; self.height];
        // start and end never has blizzard
        for x in 1..self.height - 1 {
            for y in 1..self.width - 1 {
                for (d, v) in self.blizzards[x].iter().enumerate() {
                    if v & (1 << y) != 0 {
                        let (nx, ny) = self.next_pos(x, y, d);
                        next_blizzards[nx][d] |= 1 << ny;
                    }
                }
            }
        }
        self.blizzards = next_blizzards;
    }

    fn wrap(&self, x: usize, y: usize) -> (usize, usize) {
        let mut nx = x;
        let mut ny = y;
        if x == 0 {
            nx = self.height - 2;
        }
        if y == 0 {
            ny = self.width - 2;
        }
        if x == self.height - 1 {
            nx = 1;
        }
        if y == self.width - 1 {
            ny = 1;
        }
        (nx, ny)
    }

    fn next_pos(&self, x: usize, y: usize, c: usize) -> (usize, usize) {
        match c {
            0 => self.wrap(x - 1, y),
            1 => self.wrap(x + 1, y),
            2 => self.wrap(x, y + 1),
            3 => self.wrap(x, y - 1),
            _ => unreachable!("{}", c),
        }
    }

    fn start_and_end(&self) -> ((usize, usize), (usize, usize)) {
        (self.start, self.end)
    }

    fn moveable(&self, x: usize, y: usize) -> bool {
        x > 0
            && y > 0
            && x < self.height - 1
            && y < self.width - 1
            && self.blizzards[x].iter().all(|v| v & (1 << y) == 0)
    }

    #[allow(dead_code)]
    fn draw(&self, e_x: usize, e_y: usize) -> String {
        let mut s = String::new();
        for x in 0..self.height {
            for y in 0..self.width {
                if x == e_x && y == e_y {
                    s.push('E')
                } else if (x == 0 && y == self.start.1) || (x == self.height - 1 && y == self.end.1)
                {
                    s.push('.')
                } else if x == 0 || y == 0 || x == self.height - 1 || y == self.width - 1 {
                    s.push('#');
                } else {
                    let mask = 1 << y;
                    let b: Vec<usize> = self.blizzards[x]
                        .iter()
                        .enumerate()
                        .filter(|(_, &b)| b & mask != 0)
                        .map(|(i, _)| i)
                        .collect();

                    if b.is_empty() {
                        s.push('.')
                    } else if b.len() > 1 {
                        s.push_str(&b.len().to_string())
                    } else {
                        s.push(match b[0] {
                            0 => '^',
                            1 => 'v',
                            2 => '>',
                            3 => '<',
                            _ => unreachable!(),
                        })
                    }
                }
            }
            s.push('\n');
        }
        s
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let start = s
            .lines()
            .next()
            .unwrap()
            .trim()
            .chars()
            .position(|v| v == '.')
            .unwrap();
        let end = s
            .lines()
            .last()
            .unwrap()
            .trim()
            .chars()
            .position(|v| v == '.')
            .unwrap();
        let height = s.lines().count();
        let width = s.lines().next().unwrap().trim().len();
        let mut blizzards = vec![[0; 4]; height];
        for (x, line) in s.lines().enumerate() {
            for (y, c) in line.trim().char_indices() {
                let row = &mut blizzards[x];
                match c {
                    '#' | '.' => (),
                    '^' => row[0] |= 1 << y,
                    'v' => row[1] |= 1 << y,
                    '>' => row[2] |= 1 << y,
                    '<' => row[3] |= 1 << y,
                    _ => unreachable!("{c}"),
                }
            }
        }
        Ok(Map {
            start: (0, start),
            end: (height - 1, end),
            width,
            height,
            blizzards,
        })
    }
}

#[test]
fn example_input() {
    let input = "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";
    let mut map: Map = input.parse().unwrap();
    let (start, end) = map.start_and_end();
    println!("{}", map.draw(start.0, start.1));
    let map_cache = build_map_cache(&mut map);
    assert_eq!(18, part1(&map_cache, start, end).unwrap());
    assert_eq!(54, part2(&map_cache, start, end).unwrap());
}
