use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut elves_calories = Vec::new();
    let mut calories: Vec<i32> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            if !calories.is_empty() {
                elves_calories.push(calories.clone());
                calories.clear();
            }
        } else {
            calories.push(line.parse().unwrap());
        }
    }

    part1(&elves_calories)?;
    // part2()?;
    Ok(())
}

fn part1(elves_calories: &[Vec<i32>]) -> Result<()> {
    let max_calories = elves_calories
        .iter()
        .map(|c| c.iter().sum::<i32>())
        .max()
        .unwrap();
    writeln!(
        io::stdout(),
        "How many total Calories is that Elf carrying?: {}",
        max_calories
    )?;
    Ok(())
}
