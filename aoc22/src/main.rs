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
    let (map, path) = parse_input(&input).unwrap();

    part1(&map, &path)?;
    // part2()?;
    Ok(())
}

fn part1(map: &Map, path: &[Movement]) -> Result<i32> {
    let start = Instant::now();
    let mut state = State::new(map.top_left_open_tile().unwrap(), 0);
    state.follow_the_path(map, path);
    let password = state.password();

    writeln!(io::stdout(), "Part1: {:?}", password)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(password)
}

#[derive(Debug, Clone)]
struct State {
    row: i32,
    column: i32,
    facing: i32,
}

impl State {
    fn new(coord: Coord, facing: i32) -> Self {
        Self {
            row: coord.0,
            column: coord.1,
            facing,
        }
    }

    fn coord(&self) -> Coord {
        (self.row, self.column)
    }

    // facing
    // > right 0
    // v down 1
    // < left 2
    // ^ up 3
    fn follow_the_path(&mut self, map: &Map, path: &[Movement]) {
        for m in path {
            match m {
                Movement::S(_) => self.go_stright(m, map),
                Movement::L | Movement::R => self.turn(m),
            }
        }
    }

    fn turn(&mut self, m: &Movement) {
        let mut facing = self.facing;
        match m {
            Movement::S(_) => unreachable!("Not a turn"),
            Movement::L => {
                facing -= 1;
                if facing == -1 {
                    facing = 3
                }
            }
            Movement::R => {
                facing += 1;
                if facing == 4 {
                    facing = 0
                }
            }
        }
        self.facing = facing
    }

    fn go_stright(&mut self, m: &Movement, map: &Map) {
        let facing = self.facing;
        let mut real_path: Vec<Coord> = vec![];
        match m {
            Movement::S(step) => {
                let f: fn(Coord) -> Coord = match facing {
                    0 => |(x, y)| (x, y + 1),
                    1 => |(x, y)| (x + 1, y),
                    2 => |(x, y)| (x, y - 1),
                    3 => |(x, y)| (x - 1, y),
                    _ => unreachable!("facing: {} not possible", facing),
                };
                let mut coord = self.coord();
                let mut cur_step = 0;
                while cur_step < *step {
                    coord = f(coord);
                    coord = (
                        coord.0.rem_euclid(map.bottom_right.0),
                        coord.1.rem_euclid(map.bottom_right.1),
                    ); // wrap
                    if let Some(t) = map.get(&coord) {
                        cur_step += 1;
                        match t {
                            Tile::Open => {
                                real_path.push(coord);
                                (self.row, self.column) = coord;
                            }
                            Tile::Wall => break,
                        }
                    } else {
                        continue;
                    }
                }
            }
            Movement::L | Movement::R => unreachable!("This movemen is a turn, can not go stright"),
        }
        // println!("{:?}", real_path);
    }

    fn password(&self) -> i32 {
        (self.row + 1) * 1000 + (self.column + 1) * 4 + self.facing
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Coord, Tile>,
    bottom_right: Coord,
}

impl Map {
    fn get(&self, k: &Coord) -> Option<&Tile> {
        self.map.get(k)
    }
}

impl Map {
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

    fn top_left_open_tile(&self) -> Result<Coord> {
        for y in 0..self.bottom_right.1 {
            if let Some(Tile::Open) = self.get(&(0, y)) {
                return Ok((0, y));
            }
        }
        return err!("There is not open tile on the first row");
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
    let mut max_y = 1;
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
                max_y = max_y.max(y as i32 + 1);
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
    assert_eq!(part1(&map, &path).unwrap(), 6032);
}
