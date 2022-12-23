use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Integer = i32;
type Pos = (Integer, Integer);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ground: Ground = input.parse()?;

    part1(&mut ground.clone())?;
    // part2()?;
    Ok(())
}

fn part1(ground: &mut Ground) -> Result<Integer> {
    let start = Instant::now();
    ground.rounds(10);
    let result = ground.smallest_rectangle_tiles() - ground.elves.len() as Integer;
    dbg!(result);

    writeln!(io::stdout(), "Part1: {:?}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn north(pos: &Pos) -> Pos {
    // north is up, x - 1
    (pos.0 - 1, pos.1)
}

fn south(pos: &Pos) -> Pos {
    // south is down, x + 1
    (pos.0 + 1, pos.1)
}

fn west(pos: &Pos) -> Pos {
    // west is left, y - 1
    (pos.0, pos.1 - 1)
}

fn east(pos: &Pos) -> Pos {
    // east is right, y + 1
    (pos.0, pos.1 + 1)
}

#[derive(Debug, Clone)]
struct Ground {
    elves: HashSet<Pos>,
    dir: Vec<[String; 4]>,
}

impl Ground {
    fn new(elves: HashSet<Pos>) -> Self {
        Ground {
            elves,
            dir: vec![
                [
                    "N".to_string(),
                    "NE".to_string(),
                    "NW".to_string(),
                    "N".to_string(),
                ],
                [
                    "S".to_string(),
                    "SE".to_string(),
                    "SW".to_string(),
                    "S".to_string(),
                ],
                [
                    "W".to_string(),
                    "NW".to_string(),
                    "SW".to_string(),
                    "W".to_string(),
                ],
                [
                    "E".to_string(),
                    "NE".to_string(),
                    "SE".to_string(),
                    "E".to_string(),
                ],
            ],
        }
    }

    fn next(pos: Pos, dir: &str) -> Pos {
        dir.chars().fold(pos, |cur, c| match c {
            'N' => north(&cur),
            'S' => south(&cur),
            'E' => east(&cur),
            'W' => west(&cur),
            _ => unreachable!("wrong direction: {}", dir),
        })
    }

    fn any_adjacent(&self, pos: &Pos) -> bool {
        let dir = ["N", "S", "W", "E", "NW", "NE", "SW", "SE"];
        dir.iter()
            .any(|s| self.elves.contains(&Ground::next(*pos, s)))
    }

    fn rounds(&mut self, time: usize) {
        // println!("{}", self.draw());
        for _ in 0..time {
            // first half round
            let mut next_move: HashMap<Pos, Vec<Pos>> = HashMap::new();
            for &pos in &self.elves {
                let mut flag = true;
                if !self.any_adjacent(&pos) {
                    next_move.entry(pos).or_insert(vec![]).push(pos);
                    continue;
                }
                for dir in &self.dir {
                    if dir[..3]
                        .iter()
                        .all(|s| !self.elves.contains(&Ground::next(pos, s)))
                    {
                        let next = Ground::next(pos, &dir[3]);
                        next_move.entry(next).or_insert(vec![]).push(pos);
                        flag = false;
                        break;
                    }
                }
                if flag {
                    // no move made stay at the same position
                    next_move.entry(pos).or_insert(vec![]).push(pos);
                }
            }
            // second half round
            let elves: HashSet<_> = next_move
                .into_iter()
                .map(|(p, v)| if v.len() == 1 { vec![p] } else { v })
                .flatten()
                .collect();
            self.elves = elves;
            // rotate the direction
            let first_dir = self.dir.remove(0);
            self.dir.push(first_dir);
            // println!("{}", self.draw());
        }
    }

    fn smallest_rectangle_tiles(&self) -> Integer {
        let (mut min_x, mut min_y) = (Integer::MAX, Integer::MAX);
        let (mut max_x, mut max_y) = (Integer::MIN, Integer::MIN);
        for &(x, y) in &self.elves {
            min_x = x.min(min_x);
            min_y = y.min(min_y);
            max_x = x.max(max_x);
            max_y = y.max(max_y);
        }
        return (max_x - min_x + 1) * (max_y - min_y + 1);
    }

    fn draw(&self) -> String {
        let (mut min_x, mut min_y) = (Integer::MAX, Integer::MAX);
        let (mut max_x, mut max_y) = (Integer::MIN, Integer::MIN);
        for &(x, y) in &self.elves {
            min_x = x.min(min_x);
            min_y = y.min(min_y);
            max_x = x.max(max_x);
            max_y = y.max(max_y);
        }
        let mut s = String::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if self.elves.contains(&(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }
}

impl FromStr for Ground {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut elves = HashSet::new();
        for (x, line) in s.lines().enumerate() {
            if line.is_empty() {
                return err!("not a valid input");
            }
            for (y, c) in line.trim().char_indices() {
                if c == '#' {
                    elves.insert((x as i32, y as i32));
                }
            }
        }
        Ok(Ground::new(elves))
    }
}

#[test]
fn example_input() {
    let input = "....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..";
    let ground: Ground = input.parse().unwrap();
    assert_eq!(part1(&mut ground.clone()).unwrap(), 110);
}
