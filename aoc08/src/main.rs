use std::error::Error;
use std::io::{self, Read, Write};
use std::vec;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type MoveFn = fn(usize, usize, usize) -> (usize, usize);

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

fn part1(map: &[Vec<u8>]) -> Result<()> {
    let height = map.len();
    let width = map[0].len();

    let mut visible = vec![vec![false; width]; height];

    for i in 0..height {
        for j in 0..width {
            visible[i][j] |= is_on_edge(visible_at((i, j), map, 0, move_left), height, width)
                | is_on_edge(visible_at((i, j), map, width, move_right), height, width)
                | is_on_edge(visible_at((i, j), map, 0, move_up), height, width)
                | is_on_edge(visible_at((i, j), map, height, move_down), height, width);
        }
    }
    let count = visible.iter().flatten().filter(|&&b| b).count();
    writeln!(
        io::stdout(),
        "how many trees are visible from outside the grid? {count}",
    )?;

    Ok(())
}

fn part2(map: &[Vec<u8>]) -> Result<()> {
    let height = map.len();
    let width = map[0].len();

    let mut scores = vec![vec![1; width]; height];

    for i in 0..height {
        for j in 0..width {
            if scores[i][j] == 0 {
                continue;
            }
            let p = visible_at((i, j), map, 0, move_left);
            scores[i][j] *= visible_tree((i, j), p, height, width);

            let p = visible_at((i, j), map, width, move_right);
            scores[i][j] *= visible_tree((i, j), p, height, width);

            let p = visible_at((i, j), map, 0, move_up);
            scores[i][j] *= visible_tree((i, j), p, height, width);

            let p = visible_at((i, j), map, height, move_down);
            scores[i][j] *= visible_tree((i, j), p, height, width);
        }
    }
    let max_score = scores.iter().flatten().max().unwrap();

    writeln!(
        io::stdout(),
        "What is the highest scenic score possible for any tree? {max_score}",
    )?;
    Ok(())
}

fn visible_tree(src: (usize, usize), dest: (usize, usize), height: usize, width: usize) -> usize {
    distance(src, dest)
        + if is_on_edge(dest, height, width) {
            0
        } else {
            1
        }
}

fn visible_at(p: (usize, usize), map: &[Vec<u8>], bound: usize, move_fn: MoveFn) -> (usize, usize) {
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
