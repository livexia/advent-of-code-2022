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
    part2(&monkeys, &index)?;
    Ok(())
}

fn part1(monkeys: &[Monkey], index: &HashMap<&str, Integer>) -> Result<Integer> {
    let start = Instant::now();
    let r = dfs(monkeys, index.get("root").unwrap(), &mut HashMap::new(), 1)
        .unwrap()
        .unwrap();

    writeln!(io::stdout(), "Part1: {r}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(r)
}

fn part2(monkeys: &[Monkey], index: &HashMap<&str, Integer>) -> Result<Integer> {
    let start = Instant::now();

    let &root_id = index.get("root").unwrap();
    let (m1, m2) = match &monkeys[root_id as usize].yell {
        Yell::Number(_) => return err!("not a valid root: {:?}", monkeys[root_id as usize]),
        Yell::Operation((m1, _, m2)) => (*m1, *m2),
    };

    let mut memo = HashMap::new();
    dfs(monkeys, &root_id, &mut memo, 2);
    let f1 = memo.get(&m1).unwrap();
    let f2 = memo.get(&m2).unwrap();
    let humn = solve(f1, f2)?;

    // let &humn_id = index.get("humn").unwrap();
    // let mut humn = monkeys[humn_id as usize].yell.unwrap().unwrap();
    // let base1 = f1.calc(humn);
    // let base2 = f2.calc(humn);
    // let (mut left, mut right) = (0, 0);
    // loop {
    //     let r1 = f1.calc(humn);
    //     let r2 = f2.calc(humn);
    //     if r1 == r2 {
    //         break;
    //     } else if r1 <= base1 && r1 > base2 {
    //         humn *= 2
    //     } else {
    //         left = humn / 2;
    //         right = humn;
    //         break;
    //     }
    // }
    // let mut mid = (left + right) / 2;
    // while mid < right {
    //     if f1.calc(mid) == f2.calc(mid) {
    //         humn = mid;
    //         break;
    //     } else if f1.calc(mid) < f2.calc(mid) {
    //         right = mid
    //     } else {
    //         left = mid
    //     }
    //     mid = (left + right) / 2;
    // }

    writeln!(io::stdout(), "Part2: {humn}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(humn)
}

fn dfs(
    monkeys: &[Monkey],
    root: &Integer,
    memo: &mut HashMap<Integer, Formula>,
    part: u8,
) -> Formula {
    // no cycle
    if part == 2 && monkeys[*root as usize].name == "humn" {
        return Formula::List(vec![]);
    }
    if let Some(r) = memo.get(root) {
        return r.clone();
    }
    let yell = &monkeys[*root as usize].yell;
    match yell {
        Yell::Number(n) => {
            memo.insert(*root, Formula::Number(*n));
            Formula::Number(*n)
        }
        Yell::Operation((m1, op, m2)) => {
            let op1 = dfs(monkeys, m1, memo, part);
            let op2 = dfs(monkeys, m2, memo, part);
            let r = match (op1, op2) {
                (Formula::Number(op1), Formula::Number(op2)) => Formula::Number(calc(op, op1, op2)),
                (Formula::Number(op1), Formula::List(f2)) => Formula::List(vec![
                    Formula::Number(op1),
                    Formula::Operation(*op),
                    Formula::List(f2),
                ]),
                (Formula::List(f1), Formula::Number(op2)) => Formula::List(vec![
                    Formula::List(f1),
                    Formula::Operation(*op),
                    Formula::Number(op2),
                ]),
                (Formula::List(f1), Formula::List(f2)) => Formula::List(vec![
                    Formula::List(f1),
                    Formula::Operation(*op),
                    Formula::List(f2),
                ]),
                (Formula::Operation(_), _) => unreachable!(),
                (_, Formula::Operation(_)) => unreachable!(),
            };
            memo.insert(*root, r.clone());
            r
        }
    }
}

fn solve(f1: &Formula, f2: &Formula) -> Result<Integer> {
    // eprintln!("Solve: {} = {}", f1.pretty(), f2.pretty());
    match (f1, f2) {
        (Formula::List(v), Formula::Number(op1)) => {
            if v.is_empty() {
                return Ok(*op1);
            }
            let op = v[1].get_op().unwrap();
            let mut op2 = None;
            let mut next_f = None;
            let mut op_index = 0;
            for (i, f) in v.iter().enumerate() {
                match f {
                    Formula::Number(n) => {
                        op2 = {
                            op_index = i;
                            Some(*n)
                        }
                    }
                    Formula::Operation(_) => (),
                    Formula::List(_) => next_f = Some(f),
                }
            }
            let mut op1 = *op1;
            if let Some(op2) = op2 {
                if op == '*' || op == '+' || op_index == 2 {
                    op1 = calc_rev(&op, op1, op2)
                } else if op_index == 0 {
                    if op == '/' {
                        op1 = op2 / op1
                    } else if op == '-' {
                        op1 = op2 - op1
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
                if let Some(f) = next_f {
                    return solve(f, &Formula::Number(op1));
                }
            }
            err!("can not solve {}={}", f1.pretty(), f2.pretty())
        }
        (Formula::Number(_), Formula::List(_)) => solve(f2, f1),
        (Formula::List(_), Formula::List(_)) => {
            err!("can not solve {}={}", f1.pretty(), f2.pretty())
        }
        (Formula::Number(_), Formula::Number(_)) => {
            err!("can not solve {}={}", f1.pretty(), f2.pretty())
        }
        (Formula::Operation(_), _) => err!("can not solve {}={}", f1.pretty(), f2.pretty()),
        (_, Formula::Operation(_)) => err!("can not solve {}={}", f1.pretty(), f2.pretty()),
    }
}

fn calc(op: &char, op1: Integer, op2: Integer) -> Integer {
    match op {
        '+' => op1 + op2,
        '-' => op1 - op2,
        '*' => op1 * op2,
        '/' => op1 / op2,
        _ => unreachable!(),
    }
}

fn calc_rev(op: &char, op1: Integer, op2: Integer) -> Integer {
    match op {
        '+' => op1 - op2,
        '-' => op1 + op2,
        '*' => op1 / op2,
        '/' => op1 * op2,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
enum Formula {
    Number(Integer),
    Operation(char),
    List(Vec<Formula>),
}

impl Formula {
    fn unwrap(&self) -> Option<Integer> {
        match self {
            Formula::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn get_op(&self) -> Option<char> {
        match self {
            Formula::Number(_) => None,
            Formula::Operation(c) => Some(*c),
            Formula::List(_) => None,
        }
    }

    fn pretty(&self) -> String {
        let mut s = String::new();
        match self {
            Formula::Number(n) => s.push_str(&format!("{n}")),
            Formula::Operation(op) => s.push_str(&format!(" {op} ")),
            Formula::List(v) => {
                if v.is_empty() {
                    s.push_str("humn");
                }
                for f in v {
                    match f {
                        Formula::Number(_) | Formula::Operation(_) => s.push_str(&f.pretty()),
                        Formula::List(_) => {
                            s.push('(');
                            s.push_str(&f.pretty());
                            s.push(')');
                        }
                    }
                }
            }
        }
        s
    }

    fn _simplify(self) -> Formula {
        // not working
        match &self {
            Formula::Number(_) => self,
            Formula::Operation(_) => self,
            Formula::List(v) => {
                let l = v.len();
                if l == 0 {
                    self
                } else if l == 3 {
                    let f1 = v[0].clone();
                    let f2 = v[2].clone();
                    let op = v[1].get_op().unwrap();
                    if op == '+' || op == '-' {
                        return self;
                    }
                    match (f1, f2) {
                        (Formula::Number(op1), Formula::Number(op2)) => {
                            Formula::Number(calc(&op, op1, op2))
                        }
                        (Formula::Number(op1), Formula::List(inner_v)) => Formula::List(
                            inner_v
                                .iter()
                                .cloned()
                                .map(move |f| match f {
                                    Formula::Number(op2) => Formula::Number(calc(&op, op1, op2)),
                                    Formula::Operation(_) => f.clone(),
                                    Formula::List(inner_v_v) => Formula::List(vec![
                                        Formula::Number(op1),
                                        Formula::Operation(op),
                                        Formula::List(inner_v_v),
                                    ]),
                                })
                                .collect(),
                        )
                        ._simplify(),
                        (Formula::List(inner_v), Formula::Number(op2)) => Formula::List(
                            inner_v
                                .iter()
                                .cloned()
                                .map(move |f| match f {
                                    Formula::Number(op1) => Formula::Number(calc(&op, op1, op2)),
                                    Formula::Operation(_) => f.clone(),
                                    Formula::List(inner_v_v) => Formula::List(vec![
                                        Formula::List(inner_v_v),
                                        Formula::Operation(op),
                                        Formula::Number(op2),
                                    ]),
                                })
                                .collect(),
                        )
                        ._simplify(),
                        (Formula::List(v1), Formula::List(v2)) => {
                            return Formula::List(vec![
                                Formula::List(v1)._simplify(),
                                Formula::Operation(op),
                                Formula::List(v2)._simplify(),
                            ]);
                        }
                        (Formula::Operation(_), _) => unreachable!(),
                        (_, Formula::Operation(_)) => unreachable!(),
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    #[allow(dead_code)]
    fn calc(&self, unknow: Integer) -> Integer {
        match self {
            Formula::Number(n) => *n,
            Formula::Operation(_) => unreachable!(),
            Formula::List(v) => {
                if v.is_empty() {
                    return unknow;
                }
                let op1 = v[0].calc(unknow);
                let op2 = v[2].calc(unknow);
                let op = v[1].get_op().unwrap();
                calc(&op, op1, op2)
            }
        }
    }
}

#[derive(Debug)]
enum Yell {
    Number(Integer),
    Operation((Integer, char, Integer)),
}

impl Yell {
    #[allow(dead_code)]
    fn unwrap(&self) -> Option<Integer> {
        match self {
            Yell::Number(n) => Some(*n),
            Yell::Operation(_) => None,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
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
            let yell = yell.split(' ').collect::<Vec<_>>();
            if yell.len() == 1 {
                result[id as usize] = Some(Monkey {
                    name: name.to_string(),
                    yell: Yell::Number(yell[0].parse()?),
                });
            } else if yell.len() == 3 {
                let id1 = get_moneky_id(&mut index, &mut result, &mut max_id, yell[0]);
                let op = yell[1].trim().chars().next().unwrap();
                let id2 = get_moneky_id(&mut index, &mut result, &mut max_id, yell[2]);
                if !['+', '-', '*', '/'].contains(&op) {
                    return err!("not a valid monkey yell: {}", line);
                }
                result[id as usize] = Some(Monkey {
                    name: name.to_string(),
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
    assert_eq!(301, part2(&monkeys, &index).unwrap());
}
