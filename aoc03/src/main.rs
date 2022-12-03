use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    println!("{}", 'Z' as u8 - 'A' as u8 + 27);
    io::stdin().read_to_string(&mut input)?;
    let rucksacks: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| {
                    if b < 'a' as u8 {
                        (b as u8 - 'A' as u8 + 27) as usize
                    } else {
                        (b as u8 - 'a' as u8 + 1) as usize
                    }
                })
                .collect()
        })
        .collect();

    part1(&rucksacks)?;
    part2(&rucksacks)?;
    Ok(())
}

fn part1(rucksacks: &[Vec<usize>]) -> Result<()> {
    let mut sum = 0;
    for items in rucksacks {
        let l = items.len();
        let mut dup = vec![(false, false); 53];
        for i in 0..l / 2 {
            dup[items[i]].0 = true;
        }
        for i in l / 2..l {
            dup[items[i]].1 = true;
        }
        sum += dup
            .iter()
            .enumerate()
            .filter(|(_, c)| c == &&(true, true))
            .map(|(i, _)| i)
            .sum::<usize>();
    }
    writeln!(
        io::stdout(),
        "What is the sum of the priorities of those item types? {}",
        sum
    )?;
    Ok(())
}

fn part2(rucksacks: &[Vec<usize>]) -> Result<()> {
    let mut sum = 0;
    for group in rucksacks.chunks(3) {
        let mut dup = vec![(false, false, false); 53];
        for &i in &group[0] {
            dup[i].0 = true;
        }
        for &i in &group[1] {
            dup[i].1 = true;
        }
        for &i in &group[2] {
            dup[i].2 = true;
        }
        sum += dup
            .iter()
            .enumerate()
            .filter(|(_, c)| c == &&(true, true, true))
            .map(|(i, _)| i)
            .sum::<usize>();
    }
    writeln!(
        io::stdout(),
        "What is the sum of the priorities of those item types? {}",
        sum
    )?;
    Ok(())
}
