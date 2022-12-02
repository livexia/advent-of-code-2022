use std::error::Error;
use std::io::{self, Read, Write};
use std::ops::Sub;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let rounds: Vec<(Shape, Shape)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(f, l)| (f.parse().unwrap(), l.parse().unwrap()))
        .collect();

    part1(&rounds)?;
    // part2()?;
    Ok(())
}

fn part1(rounds: &[(Shape, Shape)]) -> Result<()> {
    let score = rounds.iter().fold(0, |score, &(a, b)| (b - a) + score);
    writeln!(io::stdout(), "What would your total score be if everything goes exactly according to your strategy guide? {}", score)?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        use Shape::*;

        if s.len() != 1 {
            err!("{:?} is not a shape", s)
        } else {
            Ok(match s {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => return err!("{:?} is not a shape", s),
            })
        }
    }
}

impl Sub for Shape {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        use Shape::*;
        let score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        score
            + match (self, rhs) {
                (Rock, Rock) => 3,
                (Rock, Paper) => 0,
                (Rock, Scissors) => 6,
                (Paper, Rock) => 6,
                (Paper, Paper) => 3,
                (Paper, Scissors) => 0,
                (Scissors, Rock) => 0,
                (Scissors, Paper) => 6,
                (Scissors, Scissors) => 3,
            }
    }
}
