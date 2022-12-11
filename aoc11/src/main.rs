use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Level = u64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let monkeys = input
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<Monkey>>>()?;

    part1(monkeys.clone())?;
    part2(monkeys.clone())?;
    Ok(())
}

fn part1(mut monkeys: Vec<Monkey>) -> Result<()> {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let l = monkeys[i].items.len();
            for _ in 0..l {
                let (item, m_id) = monkeys[i].part1_throw()?;
                monkeys[m_id].items.push_back(item);
            }
        }
    }

    let mut result: Vec<Level> = monkeys.iter().map(|m| m.times).collect();
    result.sort();
    let result = result.pop().unwrap() * result.pop().unwrap();
    writeln!(
        io::stdout(),
        "What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans? {}", 
        result
    )?;

    Ok(())
}

fn part2(mut monkeys: Vec<Monkey>) -> Result<()> {
    let div = monkeys.iter().map(|m| m.test.0).fold(1, |r, d| r * d);
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let l = monkeys[i].items.len();
            for _ in 0..l {
                let (item, m_id) = monkeys[i].part2_throw(div)?;
                monkeys[m_id].items.push_back(item);
            }
        }
    }

    let mut result: Vec<Level> = monkeys.iter().map(|m| m.times).collect();
    result.sort();
    let result = result.pop().unwrap() * result.pop().unwrap();
    writeln!(
        io::stdout(),
        "What is the level of monkey business after 10000 rounds of stuff-slinging simian shenanigans? {}", 
        result
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
enum Operation {
    Old,
    Num(Level),
    Add,
    Multi,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<Level>,
    operation: (Operation, Operation, Operation),
    test: (Level, usize, usize),
    times: Level,
}

impl Monkey {
    fn part1_throw(&mut self) -> Result<(Level, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.times += 1;
            let op1 = match self.operation.0 {
                Operation::Old => item,
                Operation::Num(n) => n,
                _ => return err!("This is not a valid operation"),
            };
            let op2 = match self.operation.1 {
                Operation::Old => item,
                Operation::Num(n) => n,
                _ => return err!("This is not a valid operation"),
            };
            let new = match self.operation.2 {
                Operation::Add => op1 + op2,
                Operation::Multi => op1 * op2,
                _ => return err!("This is not a valid operation"),
            };
            let new = new / 3;
            return if new % self.test.0 == 0 {
                Ok((new, self.test.1))
            } else {
                Ok((new, self.test.2))
            };
        }

        err!("can not throw")
    }

    fn part2_throw(&mut self, div: Level) -> Result<(Level, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.times += 1;
            let op1 = match self.operation.0 {
                Operation::Old => item,
                Operation::Num(n) => n,
                _ => return err!("This is not a valid operation"),
            };
            let op2 = match self.operation.1 {
                Operation::Old => item,
                Operation::Num(n) => n,
                _ => return err!("This is not a valid operation"),
            };
            let new = match self.operation.2 {
                Operation::Add => op1 + op2,
                Operation::Multi => op1 % div * op2 % div % div,
                _ => return err!("This is not a valid operation"),
            };
            return if new % self.test.0 == 0 {
                Ok((new, self.test.1))
            } else {
                Ok((new, self.test.2))
            };
        }

        err!("can not throw")
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().map(|s| s.trim()).collect();
        if lines[0].starts_with("Monkey") {
            if let Some(items) = lines[1].strip_prefix("Starting items: ") {
                let items = items
                    .split(", ")
                    .map(|i| i.parse().unwrap())
                    .collect::<VecDeque<Level>>();
                if let Some(operation) = lines[2].strip_prefix("Operation: new = ") {
                    let op: Vec<&str> = operation.split(" ").collect();
                    let op1 = match op[0] {
                        "old" => Operation::Old,
                        _ => Operation::Num(op[0].parse::<Level>().unwrap()),
                    };
                    let op2 = match op[2] {
                        "old" => Operation::Old,
                        _ => Operation::Num(op[2].parse::<Level>().unwrap()),
                    };
                    let op = match op[1] {
                        "*" => Operation::Multi,
                        "+" => Operation::Add,
                        _ => return err!("This is not a valid operation"),
                    };
                    let operation = (op1, op2, op);
                    if let Some(div) = lines[3].strip_prefix("Test: divisible by ") {
                        let div = div.parse::<Level>().unwrap();
                        if let Some(m1) = lines[4].strip_prefix("If true: throw to monkey ") {
                            let m1 = m1.parse::<usize>().unwrap();
                            if let Some(m2) = lines[5].strip_prefix("If false: throw to monkey ") {
                                let m2 = m2.parse::<usize>().unwrap();
                                let test = (div, m1, m2);
                                return Ok(Monkey {
                                    items,
                                    operation,
                                    test,
                                    times: 0,
                                });
                            }
                        }
                    }
                }
            }
        }
        err!("This is not a monkey: {}", s)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part1_work() {
        use crate::{part1, Monkey, Result};
        use std::fs;

        let input = fs::read_to_string("input/test_input.txt").unwrap();
        let monkeys = input
            .split("\n\n")
            .map(|s| s.parse())
            .collect::<Result<Vec<Monkey>>>()
            .unwrap();
        assert!(part1(monkeys).is_ok());
    }
}
