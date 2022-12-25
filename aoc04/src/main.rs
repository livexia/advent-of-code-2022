use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Range = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let pairs: Vec<(Range, Range)> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(f, l)| (f.split_once('-').unwrap(), l.split_once('-').unwrap()))
        .map(|(f, l)| {
            (
                (f.0.parse().unwrap(), f.1.parse().unwrap()),
                (l.0.parse().unwrap(), l.1.parse().unwrap()),
            )
        })
        .collect();

    part1(&pairs)?;
    part2(&pairs)?;

    let pairs = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Pair>>>()?;
    part1_with_pair_struct(&pairs)?;
    part2_with_pair_struct(&pairs)?;
    Ok(())
}

fn part1(pairs: &[(Range, Range)]) -> Result<()> {
    let count = pairs
        .iter()
        .filter(|(p1, p2)| fully_contain(p1, p2))
        .count();
    writeln!(
        io::stdout(),
        "In how many assignment pairs does one range fully contain the other? {count}",
    )?;
    Ok(())
}

fn part2(pairs: &[(Range, Range)]) -> Result<()> {
    let count = pairs.iter().filter(|(p1, p2)| overlap(p1, p2)).count();
    writeln!(
        io::stdout(),
        "In how many assignment pairs do the ranges overlap? {count}",
    )?;
    Ok(())
}

fn fully_contain(p1: &Range, p2: &Range) -> bool {
    (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1)

    // let (mut p1, mut p2) = (p1, p2);
    // if p1.0 > p2.0 {
    //     (p1, p2) = (p2, p1);
    // }
    // if p1.0 == p2.0 && p1.1 < p2.1 {
    //     (p1, p2) = (p2, p1);
    // }
    // p2.1 <= p1.1
}

fn overlap(p1: &Range, p2: &Range) -> bool {
    let (mut p1, mut p2) = (p1, p2);
    if p1.0 > p2.0 {
        (p1, p2) = (p2, p1);
    }
    p2.0 <= p1.1
}

fn part1_with_pair_struct(pairs: &[Pair]) -> Result<()> {
    let count = pairs.iter().filter(|p| p.fully_contain()).count();
    writeln!(
        io::stdout(),
        "In how many assignment pairs does one range fully contain the other? {count}",
    )?;
    Ok(())
}

fn part2_with_pair_struct(pairs: &[Pair]) -> Result<()> {
    let count = pairs.iter().filter(|p| p.overlap()).count();
    writeln!(
        io::stdout(),
        "In how many assignment pairs do the ranges overlap? {count}",
    )?;
    Ok(())
}

struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Pair> {
        if let Some((first, last)) = s.split_once(',') {
            let first: Range = if let Some((start, end)) = first.split_once('-') {
                (start.parse()?, end.parse()?)
            } else {
                return err!("This is not a valid pair: {:?}", s);
            };
            let second: Range = if let Some((start, end)) = last.split_once('-') {
                (start.parse()?, end.parse()?)
            } else {
                return err!("This is not a valid pair: {:?}", s);
            };
            Ok(Pair { first, second })
        } else {
            err!("This is not a pair: {:?}", s)
        }
    }
}

impl Pair {
    pub fn fully_contain(&self) -> bool {
        let (p1, p2) = (self.first, self.second);
        (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1)
    }

    pub fn overlap(&self) -> bool {
        let (p1, p2) = (self.first, self.second);
        let (mut p1, mut p2) = (p1, p2);
        if p1.0 > p2.0 {
            (p1, p2) = (p2, p1);
        }
        p2.0 <= p1.1
    }
}
