use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1()?;
    // part2()?;
    Ok(())
}

fn part1() -> Result<()> {
    let start = Instant::now();

    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    todo!()
}

#[derive(Debug)]
struct Map {
    map: HashMap<Coord, Tile>,
    bottom_right: Coord,
}

impl Map {
    fn new() {}
    fn draw(&self) -> String {
        let mut s = String::new();
        for x in 0..self.bottom_right.0 {
            for y in 0..self.bottom_right.1 {
                if let Some(t) = self.map.get(&(x, y)) {
                    match t {
                        Tile::Open => s.push('.'),
                        Tile::Wall => s.push('#'),
                    }
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }
        s
    }
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug)]
enum Movement {
    S(i32),
    L,
    R,
}

fn parse_input(input: &str) -> Result<(Map, Vec<Movement>)> {
    let mut max_y = 0;
    if let Some((raw_map, raw_path)) = input.split_once("\n\n") {
        let mut map = HashMap::new();
        for (x, row) in raw_map.lines().enumerate() {
            for (y, c) in row.char_indices() {
                match c {
                    '.' => {
                        map.insert((x as i32, y as i32), Tile::Open);
                    }
                    '#' => {
                        map.insert((x as i32, y as i32), Tile::Wall);
                    }
                    ' ' => (),
                    _ => unimplemented!(),
                };
                max_y = max_y.max(y as i32);
            }
        }
        let bottom_right = (raw_map.lines().count() as i32, max_y);
        let map = Map { map, bottom_right };

        let mut path = vec![];
        let mut stright = String::new();
        for c in raw_path.chars() {
            if c.is_numeric() {
                stright.push(c);
            } else if c == 'R' {
                path.push(Movement::S(stright.parse()?));
                stright.clear();
                path.push(Movement::R)
            } else if c == 'L' {
                path.push(Movement::S(stright.parse()?));
                stright.clear();
                path.push(Movement::L)
            } else {
                unreachable!()
            }
        }
        if !stright.is_empty() {
            path.push(Movement::S(stright.parse()?));
        }
        return Ok((map, path));
    }
    err!("not a valid input for day 22")
}

#[test]
fn example_input() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    let (map, path) = parse_input(input).unwrap();
    println!("{}", map.draw());
}
