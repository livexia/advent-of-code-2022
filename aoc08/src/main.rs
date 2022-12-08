use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::vec;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    part1(&map)?;
    part2(&map)?;
    Ok(())
}

fn visible_at(
    p: (usize, usize),
    map: &[Vec<u8>],
    bound: usize,
    move_fn: fn(usize, usize, usize) -> (usize, usize),
) -> (usize, usize) {
    let mut pp = p;
    let mut np = move_fn(pp.0, pp.1, bound);
    while pp != np && !is_on_edge(pp, map.len(), map[0].len()) {
        if map[p.0][p.1] > map[np.0][np.1] {
            pp = np;
            np = move_fn(np.0, np.1, bound);
        } else {
            break;
        }
    }
    pp
}

fn is_on_edge(p: (usize, usize), height: usize, width: usize) -> bool {
    p.0 == 0 || p.1 == 0 || p.0 == height - 1 || p.1 == width - 1
}

fn move_left(x: usize, y: usize, bound: usize) -> (usize, usize) {
    if y > bound {
        (x, y - 1)
    } else {
        (x, y)
    }
}

fn move_right(x: usize, y: usize, bound: usize) -> (usize, usize) {
    if y + 1 < bound {
        (x, y + 1)
    } else {
        (x, y)
    }
}

fn move_up(x: usize, y: usize, bound: usize) -> (usize, usize) {
    if x > bound {
        (x - 1, y)
    } else {
        (x, y)
    }
}

fn move_down(x: usize, y: usize, bound: usize) -> (usize, usize) {
    if x + 1 < bound {
        (x + 1, y)
    } else {
        (x, y)
    }
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (((p1.0 as i32) - (p2.0 as i32)).abs() + ((p1.1 as i32) - (p2.1 as i32)).abs()) as usize
}

fn part1(map: &[Vec<u8>]) -> Result<()> {
    let height = map.len();
    let width = map[0].len();

    let mut visible = vec![vec![false; width]; height];

    for i in 0..height {
        for j in 0..width {
            if !visible[i][j] && is_on_edge(visible_at((i, j), &map, 0, move_left), height, width) {
                visible[i][j] = true;
            }
            if !visible[i][j]
                && is_on_edge(visible_at((i, j), &map, width, move_right), height, width)
            {
                visible[i][j] = true;
            }
            if !visible[i][j] && is_on_edge(visible_at((i, j), &map, 0, move_up), height, width) {
                visible[i][j] = true;
            }
            if !visible[i][j]
                && is_on_edge(visible_at((i, j), &map, height, move_down), height, width)
            {
                visible[i][j] = true;
            }
        }
    }
    let count = visible.iter().flatten().filter(|&&b| b).count();
    writeln!(
        io::stdout(),
        "how many trees are visible from outside the grid? {}",
        count
    )?;

    Ok(())
}

fn part2(map: &[Vec<u8>]) -> Result<()> {
    let height = map.len();
    let width = map[0].len();

    let mut distances = vec![vec![1; width]; height];
    for i in 0..height {
        distances[i][0] = 0;
        distances[i][width - 1] = 0;
    }
    for j in 0..width {
        distances[0][j] = 0;
        distances[height - 1][j] = 0;
    }

    for i in 0..height {
        for j in 0..width {
            let p = visible_at((i, j), &map, 0, move_left);
            if is_on_edge(p, height, width) {
                distances[i][j] *= distance(p, (i, j))
            } else {
                distances[i][j] *= distance(p, (i, j)) + 1;
            }

            let p = visible_at((i, j), &map, width, move_right);
            if is_on_edge(p, height, width) {
                distances[i][j] *= distance(p, (i, j))
            } else {
                distances[i][j] *= distance(p, (i, j)) + 1;
            }

            let p = visible_at((i, j), &map, 0, move_up);
            if is_on_edge(p, height, width) {
                distances[i][j] *= distance(p, (i, j))
            } else {
                distances[i][j] *= distance(p, (i, j)) + 1;
            }

            let p = visible_at((i, j), &map, height, move_down);
            if is_on_edge(p, height, width) {
                distances[i][j] *= distance(p, (i, j))
            } else {
                distances[i][j] *= distance(p, (i, j)) + 1;
            }
        }
    }
    let max_distance = distances.iter().flatten().max().unwrap();

    writeln!(
        io::stdout(),
        "What is the highest scenic score possible for any tree? {}",
        max_distance
    )?;
    Ok(())
}
