use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let buffer: Vec<char> = input.chars().collect();

    part1(&buffer)?;
    part2(&buffer)?;
    Ok(())
}

fn part1(buffer: &[char]) -> Result<()> {
    let length = buffer.len();
    let mut left = 0;
    let l = 4;
    while left + l - 1 < length {
        if unique(&buffer[left..left + l], l) {
            writeln!(io::stdout(), "How many characters need to be processed before the first start-of-packet marker is detected? {}", left + l)?;
            return Ok(());
        } else {
            left += 1;
        }
    }
    err!("Can not find the first start-of-packet marker")
}

fn part2(buffer: &[char]) -> Result<()> {
    let length = buffer.len();
    let mut left = 0;
    let l = 14;
    while left + l - 1 < length {
        if unique(&buffer[left..left + l], l) {
            writeln!(io::stdout(), "How many characters need to be processed before the first start-of-packet marker is detected? {}", left + l)?;
            return Ok(());
        } else {
            left += 1;
        }
    }
    err!("Can not find the first start-of-packet marker")
}

fn unique(chars: &[char], l: usize) -> bool {
    use std::collections::HashSet;
    let set: HashSet<&char> = HashSet::from_iter(chars.iter());
    set.len() == l
}
