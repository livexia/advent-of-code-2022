use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<_> = input.lines().collect();
    let map: HashMap<(i32, i32), u8> = input
        .iter()
        .enumerate()
        .map(|(x, l)| {
            l.bytes()
                .enumerate()
                .map(|(y, b)| ((x as i32, y as i32), b - b'0'))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let height = input.len() as i32; // x
    let width = input[0].len() as i32; // y

    use Direction::*;

    let mut left_visible = HashMap::new();
    let mut right_visible = HashMap::new();
    let mut top_visible = HashMap::new();
    let mut bottom_visible = HashMap::new();
    let mut visible = HashSet::new();
    for i in 0..height {
        left_visible.insert((i, 0), (map.get(&(i, 0)).unwrap(), 0));
        right_visible.insert((i, width - 1), (map.get(&(i, width - 1)).unwrap(), 0));
        visible.insert((i, 0));
        visible.insert((i, width - 1));
    }
    for j in 0..width {
        top_visible.insert((0, j), (map.get(&(0, j)).unwrap(), 0));
        bottom_visible.insert((height - 1, j), (map.get(&(height - 1, j)).unwrap(), 0));
        visible.insert((0, j));
        visible.insert((height - 1, j));
    }

    for x in 0..height {
        for y in 0..width {
            let h = map.get(&(x, y)).unwrap();
            for (d, v) in [(Left, &mut left_visible), (Up, &mut top_visible)] {
                if v.contains_key(&(x, y)) {
                    continue;
                }
                let p = d.next(x, y);
                let ph = map.get(&p).unwrap();
                if let Some(&(vh, count)) = v.get(&p) {
                    let new_count = if h > ph { count + 1 } else { 1 };
                    if vh < h {
                        visible.insert((x, y));
                        v.insert((x, y), (h, new_count));
                    } else {
                        v.insert((x, y), (vh, new_count));
                    }
                } else {
                    unreachable!("{:?} {:?}", (x, y), d);
                }
            }
        }
    }

    for x in (0..height).rev() {
        for y in (0..width).rev() {
            let h = map.get(&(x, y)).unwrap();
            for (d, v) in [(Right, &mut right_visible), (Down, &mut bottom_visible)] {
                if v.contains_key(&(x, y)) {
                    continue;
                }
                let p = d.next(x, y);
                let ph = map.get(&p).unwrap();
                if let Some(&(vh, count)) = v.get(&p) {
                    let new_count = if h > ph { count + 1 } else { 1 };
                    if vh < h {
                        visible.insert((x, y));
                        v.insert((x, y), (h, new_count));
                    } else {
                        v.insert((x, y), (vh, new_count));
                    }
                } else {
                    unreachable!("{:?} {:?}", (x, y), d);
                }
            }
        }
    }

    for i in 0..height {
        for j in 0..width {
            if visible.contains(&(i, j)) {
                print!("~");
            } else {
                print!("x");
            }
        }
        println!();
    }

    dbg!(visible.len());

    let mut max_distance = 0;
    for x in 0..height {
        for y in 0..width {
            let mut distance = 1;
            let p = (x, y);
            for v in [&left_visible, &right_visible, &top_visible, &bottom_visible] {
                if let Some(&(_, count)) = v.get(&p) {
                    distance *= count;
                }
            }
            if p == (1, 3) {
                dbg!(p);
                dbg!(map.get(&p));
                dbg!(left_visible.get(&p).unwrap().1);
                dbg!(top_visible.get(&p).unwrap().1);
                dbg!(right_visible.get(&p).unwrap().1);
                dbg!(bottom_visible.get(&p).unwrap().1);
            }
            max_distance = distance.max(max_distance);
        }
    }
    dbg!(max_distance);
    part1()?;
    // part2()?;
    Ok(())
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn next(&self, x: i32, y: i32) -> (i32, i32) {
        use self::*;
        match self {
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
        }
    }
}

fn part1() -> Result<()> {
    Ok(())
}
