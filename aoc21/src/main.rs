use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Integer = i64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (monkeys, index) = parse_input(&input).unwrap();
    assert_eq!(monkeys.len(), input.lines().count());

    part1(&monkeys, &index)?;
    // part2()?;
    Ok(())
}

fn part1(monkeys: &[Monkey], index: &HashMap<&str, Integer>) -> Result<Integer> {
    let start = Instant::now();
    let r = dfs(monkeys, index.get("root").unwrap(), &mut HashMap::new());

    writeln!(io::stdout(), "Part1: {:?}", r)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(r)
}

fn dfs(monkeys: &[Monkey], root: &Integer, memo: &mut HashMap<Integer, Integer>) -> Integer {
    // cycle?
    if let Some(&r) = memo.get(&root) {
        return r;
    }
    let yell = &monkeys[*root as usize].yell;
    match yell {
        Yell::Number(n) => *n,
        Yell::Operation((m1, op, m2)) => {
            let op1 = dfs(monkeys, m1, memo);
            let op2 = dfs(monkeys, m2, memo);
            let r = match op {
                '+' => op1 + op2,
                '-' => op1 - op2,
                '*' => op1 * op2,
                '/' => op1 / op2,
                _ => unreachable!(),
            };
            memo.insert(*root, r);
            r
        }
    }
}

#[derive(Debug)]
enum Yell {
    Number(Integer),
    Operation((Integer, char, Integer)),
}

#[derive(Debug)]
struct Monkey {
    id: Integer,
    yell: Yell,
}

fn parse_input(input: &str) -> Result<(Vec<Monkey>, HashMap<&str, Integer>)> {
    fn get_moneky_id<'a>(
        index: &mut HashMap<&'a str, Integer>,
        monkeys: &mut Vec<Option<Monkey>>,
        max_id: &mut Integer,
        name: &'a str,
    ) -> Integer {
        if let Some(&id) = index.get(name) {
            id
        } else {
            index.insert(name, *max_id);
            monkeys.push(None);
            *max_id += 1;
            *max_id - 1
        }
    }

    let mut index = HashMap::new();
    let mut max_id = 0;
    let mut result = vec![];
    for line in input.lines() {
        if let Some((name, yell)) = line.trim().split_once(": ") {
            let id = get_moneky_id(&mut index, &mut result, &mut max_id, name);
            let yell = yell.split(" ").collect::<Vec<_>>();
            if yell.len() == 1 {
                result[id as usize] = Some(Monkey {
                    id,
                    yell: Yell::Number(yell[0].parse()?),
                });
            } else if yell.len() == 3 {
                let id1 = get_moneky_id(&mut index, &mut result, &mut max_id, yell[0]);
                let op = yell[1].trim().chars().next().unwrap();
                let id2 = get_moneky_id(&mut index, &mut result, &mut max_id, yell[2]);
                result[id as usize] = Some(Monkey {
                    id,
                    yell: Yell::Operation((id1, op, id2)),
                });
            } else {
                return err!("not a valid monkey yell: {}", line);
            }
        } else {
            return err!("not a valid monkey: {}", line);
        }
    }
    let result = result.into_iter().map(|m| m.unwrap()).collect();
    Ok((result, index))
}

#[test]
fn example_input() {
    let input = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";
    let (monkeys, index) = parse_input(input).unwrap();
    assert_eq!(monkeys.len(), input.lines().count());
    assert_eq!(152, part1(&monkeys, &index).unwrap());
}
