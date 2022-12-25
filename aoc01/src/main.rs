use std::error::Error;
use std::io::{self, Read, Write};

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut elves_calories: Vec<i32> = vec![0];
    let mut last = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves_calories.push(0);
            last += 1;
        } else {
            elves_calories[last] += line.parse::<i32>().unwrap();
        }
    }
    elves_calories.sort_by(|a, b| b.cmp(a));

    part1(&elves_calories)?;
    part2(&elves_calories)?;
    Ok(())
}

fn part1(elves_calories: &[i32]) -> Result<()> {
    writeln!(
        io::stdout(),
        "How many total Calories is that Elf carrying? {}",
        elves_calories[0]
    )?;
    Ok(())
}

fn part2(elves_calories: &[i32]) -> Result<()> {
    writeln!(
        io::stdout(),
        "How many Calories are those Elves carrying in total? {}",
        elves_calories[0..3].iter().sum::<i32>()
    )?;
    Ok(())
}
