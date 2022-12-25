use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);
type Step = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let moves: Vec<Move> = input.lines().map(|l| l.parse()).collect::<Result<_>>()?;

    part1(&moves)?;
    part2(&moves)?;
    Ok(())
}

fn part1(moves: &[Move]) -> Result<()> {
    move_rope(moves, 2)?;
    Ok(())
}

fn part2(moves: &[Move]) -> Result<()> {
    move_rope(moves, 10)?;
    Ok(())
}

fn move_rope(moves: &[Move], length: usize) -> Result<()> {
    let mut visited = HashSet::new();
    let mut ropes = vec![(0, 0); length];

    for m in moves {
        let steps = m.get_step();
        let move_fn = m.move_fn();
        for _ in 0..steps {
            for i in 0..length - 1 {
                if i == 0 {
                    // only head can alway move
                    ropes[i] = move_fn(ropes[i]);
                }
                ropes[i + 1] = move_tail(ropes[i], ropes[i + 1]);
                if i + 1 == length - 1 {
                    visited.insert(ropes[i + 1]);
                }
            }
        }
    }
    writeln!(
        io::stdout(),
        "With rope length at {}: How many positions does the tail of the rope visit at least once? {}",
        length,
        visited.len()
    )?;
    Ok(())
}

fn move_tail(head: Coord, tail: Coord) -> Coord {
    let d = distance(head, tail);
    if d < 2 {
        // touch or cover
        tail
    } else if head.0 - tail.0 == 0 || head.1 - tail.1 == 0 {
        // same row or column
        ((tail.0 + head.0) / 2, (tail.1 + head.1) / 2)
    } else if d == 2 {
        // diagonally
        tail
    } else if d > 2 {
        // need move diagonally should be only one possible way
        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let np = (tail.0 + dx, tail.1 + dy);
            if distance(head, np) < d {
                // make sure next postion will shorten the distance
                return np;
            }
        }
        unreachable!("move diagonally but no possible way");
    } else {
        unreachable!()
    }
}

fn distance(head: Coord, tail: Coord) -> Step {
    (tail.0 - head.0).abs() + (tail.1 - head.1).abs()
}

#[derive(Debug)]
enum Move {
    Left(Step),
    Right(Step),
    Up(Step),
    Down(Step),
}

impl Move {
    fn move_fn(&self) -> fn(Coord) -> Coord {
        match self {
            Move::Left(_) => |(x, y)| (x, y - 1),
            Move::Right(_) => |(x, y)| (x, y + 1),
            Move::Up(_) => |(x, y)| (x - 1, y),
            Move::Down(_) => |(x, y)| (x + 1, y),
        }
    }

    fn get_step(&self) -> Step {
        match self {
            Move::Left(s) => *s,
            Move::Right(s) => *s,
            Move::Up(s) => *s,
            Move::Down(s) => *s,
        }
    }
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((d, steps)) = s.split_once(' ') {
            if let Ok(steps) = steps.parse::<Step>() {
                match d {
                    "L" => Ok(Move::Left(steps)),
                    "R" => Ok(Move::Right(steps)),
                    "U" => Ok(Move::Up(steps)),
                    "D" => Ok(Move::Down(steps)),
                    _ => err!("This is not a valid step: {}", s),
                }
            } else {
                err!("This is not a valid step: {}", s)
            }
        } else {
            err!("This is not a valid step: {}", s)
        }
    }
}
