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
    // part2()?;
    Ok(())
}

fn part1(moves: &[Move]) -> Result<()> {
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for m in moves {
        let steps = m.get_step();
        let move_fn = m.move_fn();
        for _ in 0..steps {
            head = move_fn(head);
            tail = move_tail(head, tail);
            visited.insert(tail);
        }
    }
    let max_height = visited.iter().map(|(i, _)| i).max().unwrap() + 1;
    let min_height = visited.iter().map(|(i, _)| i).min().unwrap() - 1;
    let max_width = visited.iter().map(|(_, i)| i).max().unwrap() + 1;
    let min_width = visited.iter().map(|(_, i)| i).min().unwrap() - 1;
    for i in min_height..max_height {
        for j in min_width..max_width {
            if visited.contains(&(i, j)) {
                print!("x");
            } else {
                print!("~")
            }
        }
        println!()
    }
    dbg!(visited.len());
    Ok(())
}

fn move_tail(head: Coord, tail: Coord) -> Coord {
    let d = distance(head, tail);
    if d == 1 {
        // touch
        return tail;
    } else if head.0 - tail.0 == 0 {
        // same row
        return (tail.0, (tail.1 + head.1) / 2);
    } else if head.1 - tail.1 == 0 {
        // same column
        return ((tail.0 + head.0) / 2, tail.1);
    } else if d == 2 {
        // diagonally
        return tail;
    } else if d > 2 {
        // need move diagonally
        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let np = (tail.0 + dx, tail.1 + dy);
            if distance(head, np) == 1 {
                return np;
            }
        }
        dbg!(tail, head);
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
        if let Some((d, steps)) = s.split_once(" ") {
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
