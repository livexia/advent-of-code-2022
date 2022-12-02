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

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let rounds: Vec<(Shape, Shape)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(f, l)| (f.parse().unwrap(), l.parse().unwrap()))
        .collect();
    let score = rounds.iter().fold(0, |score, &(a, b)| (b - a) + score);
    writeln!(io::stdout(), "What would your total score be if everything goes exactly according to your strategy guide? {}", score)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    use Outcome::*;
    use Shape::*;
    let score = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(f, l)| (f.parse::<Shape>().unwrap(), l.parse::<Outcome>().unwrap()))
        .fold(0, |score, (shape, outcome)| {
            score
                + match (shape, outcome) {
                    (Rock, Lose) => 3 + 0,
                    (Rock, Draw) => 1 + 3,
                    (Rock, Win) => 2 + 6,
                    (Paper, Lose) => 1 + 0,
                    (Paper, Draw) => 2 + 3,
                    (Paper, Win) => 3 + 6,
                    (Scissors, Lose) => 2 + 0,
                    (Scissors, Draw) => 3 + 3,
                    (Scissors, Win) => 1 + 6,
                }
        });
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

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        use Outcome::*;

        if s.len() != 1 {
            err!("{:?} is not a outcome", s)
        } else {
            Ok(match s {
                "X" => Lose,
                "Y" => Draw,
                "Z" => Win,
                _ => return err!("{:?} is not a outcome", s),
            })
        }
    }
}
