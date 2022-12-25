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
    let rucksacks: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| {
                    if b < b'a' {
                        (b - b'A' + 27) as usize
                    } else {
                        (b - b'a' + 1) as usize
                    }
                })
                .collect()
        })
        .collect();

    part1(&rucksacks)?;
    part2(&rucksacks)?;
    part1_with_bits(&input)?;
    part2_with_bits(&input)?;
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
        "What is the sum of the priorities of those item types? {sum}"
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
        "What is the sum of the priorities of those item types? {sum}"
    )?;
    Ok(())
}

fn str_to_u64(s: &str) -> u64 {
    let mut result = 0u64;
    for c in s.bytes() {
        if c < b'a' {
            result |= 1 << (c - b'A' + 27)
        } else {
            result |= 1 << (c - b'a' + 1)
        }
    }
    result
}

fn u64_bits_count(mut num: u64) -> usize {
    let mut i = 0;
    let mut result = 0;
    while i <= 52 {
        if num & 1 == 1 {
            result += i;
        }
        num >>= 1;
        i += 1;
    }
    result
}

fn part1_with_bits(input: &str) -> Result<()> {
    let mut sum = 0;
    for line in input.lines() {
        let l = line.len();
        let s1 = str_to_u64(&line[0..l / 2]);
        let s2 = str_to_u64(&line[l / 2..l]);
        sum += u64_bits_count(s1 & s2);
    }
    writeln!(
        io::stdout(),
        "What is the sum of the priorities of those item types? {sum}"
    )?;
    Ok(())
}

fn part2_with_bits(input: &str) -> Result<()> {
    let mut sum = 0;
    for lines in input.lines().collect::<Vec<_>>().chunks(3) {
        let s1 = str_to_u64(lines[0]);
        let s2 = str_to_u64(lines[1]);
        let s3 = str_to_u64(lines[2]);
        sum += u64_bits_count(s1 & s2 & s3);
    }
    writeln!(
        io::stdout(),
        "What is the sum of the priorities of those item types? {sum}"
    )?;
    Ok(())
}
