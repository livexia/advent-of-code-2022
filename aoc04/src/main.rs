use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Pair = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let pairs: Vec<(Pair, Pair)> = input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(f, l)| (f.split_once("-").unwrap(), l.split_once("-").unwrap()))
        .map(|(f, l)| {
            (
                (f.0.parse().unwrap(), f.1.parse().unwrap()),
                (l.0.parse().unwrap(), l.1.parse().unwrap()),
            )
        })
        .collect();

    part1(&pairs)?;
    part2(&pairs)?;
    Ok(())
}

fn part1(pairs: &[(Pair, Pair)]) -> Result<()> {
    let count = pairs
        .iter()
        .filter(|(p1, p2)| fully_contain(p1, p2))
        .count();
    writeln!(
        io::stdout(),
        "In how many assignment pairs does one range fully contain the other? {}",
        count
    )?;
    Ok(())
}

fn part2(pairs: &[(Pair, Pair)]) -> Result<()> {
    let count = pairs.iter().filter(|(p1, p2)| overlap(p1, p2)).count();
    writeln!(
        io::stdout(),
        "In how many assignment pairs do the ranges overlap? {}",
        count
    )?;
    Ok(())
}

fn fully_contain(p1: &Pair, p2: &Pair) -> bool {
    (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1)
}

fn overlap(p1: &Pair, p2: &Pair) -> bool {
    let (mut p1, mut p2) = (p1, p2);
    if p1.0 > p2.0 {
        (p1, p2) = (p2, p1);
    }
    p2.0 <= p1.1
}
