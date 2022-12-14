use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let paths = input
        .lines()
        .map(|l| l.trim().split(" -> ").map(|c| c.parse()).collect())
        .collect::<Result<Vec<Vec<Coord>>>>()?;
    let cave = Cave::new(&paths);
    part1(cave.clone())?;
    part2(cave.clone())?;
    Ok(())
}

fn part1(mut cave: Cave) -> Result<()> {
    let start = Instant::now();

    let result = cave.sand_fall_part1();
    writeln!(io::stdout(), "Part1: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(())
}

fn part2(mut cave: Cave) -> Result<()> {
    let start = Instant::now();

    let result = cave.sand_fall_part2();
    writeln!(io::stdout(), "Part2: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(())
}

#[derive(Clone)]
struct Cave {
    grid: HashMap<Coord, Material>,
    range_x: (usize, usize),
    range_y: (usize, usize),
    sand_src: Coord,
    floor: usize,
}

impl Cave {
    fn new(paths: &[Vec<Coord>]) -> Self {
        use Material::*;
        let mut cave = Cave {
            grid: HashMap::new(),
            range_x: (usize::MAX, 0),
            range_y: (0, 0),
            sand_src: Coord::new(500, 0),
            floor: 0,
        };
        for path in paths {
            for route in path.windows(2) {
                let (start, end) = route[0].range(&route[1]);
                cave.update_cave_size(route[0]);
                cave.update_cave_size(route[1]);

                cave.floor = cave.range_y.1 + 2;
                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        cave.grid.insert(Coord::new(x, y), Rock);
                    }
                }
            }
        }
        cave
    }

    fn update_cave_size(&mut self, c: Coord) {
        self.range_x.0 = self.range_x.0.min(c.x);
        self.range_x.1 = self.range_x.1.max(c.x);
        self.range_y.0 = self.range_y.0.min(c.y);
        self.range_y.1 = self.range_y.1.max(c.y);
    }

    fn sand_fall_part1(&mut self) -> usize {
        let mut count = 0;
        let mut cur = self.sand_src;
        while !self.into_abyss(&cur) {
            let next = cur.down();
            if !self.is_blocked(&next) {
                cur = next;
                continue;
            }
            let next = cur.left();
            if !self.is_blocked(&next) {
                cur = next;
                continue;
            }
            let next = cur.right();
            if !self.is_blocked(&next) {
                cur = next;
                continue;
            }
            self.sand_rest(cur);
            cur = self.sand_src;
            count += 1;
        }
        count
    }

    fn is_blocked(&self, c: &Coord) -> bool {
        if c.y == self.floor {
            return true;
        }
        match self.grid.get(&c) {
            Some(m) => match m {
                Material::Air => false,
                Material::Rock | Material::Sand => true,
            },
            None => false,
        }
    }

    fn sand_rest(&mut self, c: Coord) {
        self.update_cave_size(c);
        self.grid.insert(c, Material::Sand);
    }

    fn into_abyss(&self, c: &Coord) -> bool {
        c.x > self.range_x.1 || c.y > self.range_y.1 || c.x < self.range_x.0
    }

    fn sand_fall_part2(&mut self) -> usize {
        let mut count = 0;
        let mut cur = self.sand_src;
        loop {
            let next = cur.down();
            if !self.is_blocked(&next) {
                cur = next;
                continue;
            }
            let next = cur.left();
            if !self.is_blocked(&next) {
                cur = next;
                continue;
            }
            let next = cur.right();
            if !self.is_blocked(&next) {
                cur = next;
                continue;
            }
            self.sand_rest(cur);
            count += 1;
            if cur != self.sand_src {
                cur = self.sand_src;
            } else {
                break;
            }
        }
        count
    }

    fn draw_cave(&self) -> String {
        use Material::*;

        let mut map = String::new();
        map.push_str(&format!(
            "x: {} -> {} (left -> right)\n",
            self.range_x.0, self.range_x.1
        ));
        map.push_str(&format!(
            "y: {} -> {} (top -> bottom)\n",
            self.range_y.0, self.range_y.1
        ));
        map.push_str(&format!(
            "sand sorce: ({}, {})\n",
            self.sand_src.x, self.sand_src.y
        ));
        for y in self.range_y.0..=self.range_y.1 {
            for x in self.range_x.0..=self.range_x.1 {
                if x == self.sand_src.x && y == self.sand_src.y {
                    map.push('+');
                    continue;
                }
                match self.grid.get(&Coord::new(x, y)) {
                    Some(m) => match m {
                        Air => map.push('.'),
                        Rock => map.push('#'),
                        Sand => map.push('o'),
                    },
                    None => map.push('.'),
                }
            }
            map.push('\n');
        }
        map
    }
}

#[derive(Debug, Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn range(&self, other: &Coord) -> (Coord, Coord) {
        let (start, end) = if self < other {
            (self, other)
        } else {
            (other, self)
        };
        (start.clone(), end.clone())
    }

    fn down(self) -> Self {
        Coord {
            y: self.y + 1,
            x: self.x,
        }
    }

    fn left(self) -> Self {
        Coord {
            y: self.y + 1,
            x: self.x - 1,
        }
    }

    fn right(self) -> Self {
        Coord {
            y: self.y + 1,
            x: self.x + 1,
        }
    }
}

impl FromStr for Coord {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((x, y)) = s.split_once(",") {
            return Ok(Coord {
                x: x.parse()?,
                y: y.parse()?,
            });
        }
        err!("not a valid coordinate: {}", s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        use crate::{Cave, Coord};

        let input = "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9";
        let paths: Vec<Vec<Coord>> = input
            .lines()
            .map(|l| l.trim().split(" -> ").map(|c| c.parse().unwrap()).collect())
            .collect();
        let cave = Cave::new(&paths);
        println!("{}", cave.draw_cave());
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0].len(), 3);
        assert_eq!(paths[1].len(), 4);
        let mut part1_cave = cave.clone();
        assert_eq!(24, part1_cave.sand_fall_part1());
        println!("{}", part1_cave.draw_cave());

        let mut part2_cave = cave.clone();
        assert_eq!(93, part2_cave.sand_fall_part2());
        println!("{}", part2_cave.draw_cave());
    }
}
