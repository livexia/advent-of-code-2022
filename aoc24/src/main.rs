use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
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
    let cycle_count = (map.height as i32 - 2) * (map.width as i32 - 2);
    for _ in 0..cycle_count {
        cache.push(map.clone());
        map.next();
    }
    cache
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
        if visited.contains(&(cur, cycle_time)) {
            continue;
        }
        visited.insert((cur, cycle_time));
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
    find_time
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Map {
    blizzards: Vec<Vec<Vec<char>>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn next(&mut self) {
        let mut next_blizzards = vec![vec![vec![]; self.width]; self.height];
        // start and end never has blizzard
        for x in 1..self.height - 1 {
            for y in 1..self.width - 1 {
                for &c in &self.blizzards[x][y] {
                    let (nx, ny) = self.next_pos(x, y, c);
                    next_blizzards[nx][ny].push(c)
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

    fn next_pos(&self, x: usize, y: usize, c: char) -> (usize, usize) {
        match c {
            '>' => self.wrap(x, y + 1),
            '<' => self.wrap(x, y - 1),
            '^' => self.wrap(x - 1, y),
            'v' => self.wrap(x + 1, y),
            _ => unreachable!("{}", c),
        }
    }

    fn start_and_end(&self) -> ((usize, usize), (usize, usize)) {
        (self.start, self.end)
    }

    fn moveable(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x >= self.height - 1 || y >= self.width - 1 {
            return false;
        }
        self.blizzards[x][y].is_empty()
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
                } else if self.blizzards[x][y].is_empty() {
                    s.push('.')
                } else if self.blizzards[x][y].len() == 1 {
                    s.push(self.blizzards[x][y][0])
                } else {
                    s.push_str(&self.blizzards[x][y].len().to_string())
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
        let blizzards: Vec<Vec<_>> = s
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| {
                        if c == '.' || c == '#' {
                            vec![]
                        } else {
                            vec![c]
                        }
                    })
                    .collect()
            })
            .collect();
        Ok(Map {
            start: (0, start),
            end: (blizzards.len() - 1, end),
            width: blizzards[0].len(),
            height: blizzards.len(),
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
    let map_cache = build_map_cache(&mut map);
    println!("{}", map.draw(start.0, start.1));
    assert_eq!(18, part1(&map_cache, start, end).unwrap());
    assert_eq!(54, part2(&map_cache, start, end).unwrap());
}
