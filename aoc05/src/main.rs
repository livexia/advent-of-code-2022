use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut procedures: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("move") {
            // move 3 from 9 to 4
            let mut procedure = vec![];
            for word in line.split(" ") {
                if !["move", "from", "to"].contains(&word) {
                    procedure.push(word.parse::<usize>().unwrap())
                }
            }
            procedures.push(procedure);
        } else {
            for (i, c) in line.char_indices() {
                if [' ', '[', ']'].contains(&c) || c.is_numeric() {
                    continue;
                } else {
                    let index = i / 4;
                    if stacks.len() <= index + 1 {
                        for _ in stacks.len()..=index {
                            stacks.push(vec![]);
                        }
                    }
                    stacks[index].push(c);
                }
            }
        }
    }
    for s in &mut stacks {
        s.reverse();
    }

    part1(stacks.clone(), &procedures)?;
    part2(stacks.clone(), &procedures)?;
    Ok(())
}

fn part1(stacks: Vec<Vec<char>>, procedures: &[Vec<usize>]) -> Result<()> {
    let mut stacks = stacks;
    for p in procedures {
        let count = p[0];
        let src = p[1] - 1;
        let dest = p[2] - 1;
        for _ in 0..count {
            let c = stacks[src].pop().unwrap();
            stacks[dest].push(c);
        }
    }
    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    writeln!(
        io::stdout(),
        "Part1: After the rearrangement procedure completes, what crate ends up on top of each stack? {}",
        result
    )?;
    Ok(())
}

fn part2(stacks: Vec<Vec<char>>, procedures: &[Vec<usize>]) -> Result<()> {
    let mut stacks = stacks;
    for p in procedures {
        let count = p[0];
        let src = p[1] - 1;
        let dest = p[2] - 1;
        let index = stacks[src].len() - count;
        let last = stacks[src].split_off(index);
        stacks[dest].extend(last.iter());

        // with middle stack
        // let mut temp = vec![];
        // for _ in 0..count {
        //     temp.push(stacks[src].pop().unwrap());
        // }
        // while let Some(c) = temp.pop() {
        //     stacks[dest].push(c);
        // }
    }
    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    writeln!(
        io::stdout(),
        "Part2: After the rearrangement procedure completes, what crate ends up on top of each stack? {}",
        result
    )?;
    Ok(())
}
