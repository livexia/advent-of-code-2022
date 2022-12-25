use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);
type Edge = (Coord, Coord);
type ConnectedEdge = (Edge, Edge, (i32, i32));

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (mut map, path) = parse_input(&input).unwrap();

    part1(&map, &path)?;
    part2(&mut map, &path, 50)?;
    Ok(())
}

fn part1(map: &Map, path: &[Movement]) -> Result<i32> {
    let start = Instant::now();
    let mut state = State::new(map.top_left_open_tile().unwrap(), 0);
    state.follow_the_path(map, path, 1);
    let password = state.password();

    writeln!(io::stdout(), "Part1: {password}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(password)
}

fn part2(map: &mut Map, path: &[Movement], size: i32) -> Result<i32> {
    let start = Instant::now();

    let mut state = State::new(map.top_left_open_tile().unwrap(), 0);
    map.set_cube_size(size);
    state.follow_the_path(map, path, 2);
    let password = state.password();

    writeln!(io::stdout(), "Part2: {password}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(password)
}

fn rev(edge: (Coord, Coord)) -> (Coord, Coord) {
    (edge.1, edge.0)
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

    fn move_with_facing(&self) -> fn(Coord) -> Coord {
        match self.facing {
            0 => |(x, y)| (x, y + 1),
            1 => |(x, y)| (x + 1, y),
            2 => |(x, y)| (x, y - 1),
            3 => |(x, y)| (x - 1, y),
            _ => unreachable!("facing: {} not possible", self.facing),
        }
    }

    // facing
    // > right 0
    // v down 1
    // < left 2
    // ^ up 3
    fn follow_the_path(&mut self, map: &Map, path: &[Movement], part: i32) {
        for m in path {
            match m {
                Movement::S(_) => self.go_stright(m, map, part),
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

    fn go_stright(&mut self, m: &Movement, map: &Map, part: i32) {
        let mut real_path: Vec<Coord> = vec![];
        match m {
            Movement::S(step) => {
                let f = self.move_with_facing();
                if part == 1 {
                    let mut coord = self.coord();
                    let mut cur_step = 0;
                    while cur_step < *step {
                        coord = f(coord);
                        coord = map.part1_wrap(&coord); // part one wrap
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
                } else if part == 2 {
                    self.part2_go_stright(map, *step);
                }
            }
            Movement::L | Movement::R => {
                unreachable!("This movement is a turn, can not go stright")
            }
        }
    }

    fn password(&self) -> i32 {
        (self.row + 1) * 1000 + (self.column + 1) * 4 + self.facing
    }

    fn part2_go_stright(&mut self, map: &Map, step: i32) {
        // manual write edges
        let edges = vec![
            // on the edge
            ((0, 50), (49, 50)),    // 0 face 1 left edge
            ((0, 50), (0, 99)),     // 1 face 1 top edge
            ((0, 100), (0, 149)),   // 2 face 2 top edge
            ((0, 149), (49, 149)),  // 3 face 2 right edge
            ((49, 100), (49, 149)), // 4 face 2 bottom edge
            ((50, 50), (99, 50)),   // 5 face 3 left edge
            ((50, 99), (99, 99)),   // 6 face 3 right edge
            ((100, 0), (149, 0)),   // 7 face 4 left edge
            ((100, 0), (100, 49)),  // 8 face 4 top edge
            ((100, 99), (149, 99)), // 9 face 5 right edge
            ((149, 50), (149, 99)), // 10 face 5 bottom edge
            ((150, 0), (199, 0)),   // 11 face 6 left edge
            ((199, 0), (199, 49)),  // 12 face 6 bottom edge
            ((150, 49), (199, 49)), // 13 face 6 right edge
        ];

        // facing
        // > right 0
        // v down 1
        // < left 2
        // ^ up 3
        let connected_edges = vec![
            (edges[0], rev(edges[7]), (2, 0)), // face 1 left edge = rev(face 4 left edge), left -> right
            (edges[1], edges[11], (3, 0)),     // face 1 top edge == face 6 left edge, up -> right
            (edges[2], edges[12], (3, 3)),     // face 2 top edge == face 6 bottom edge, up -> up
            (edges[3], rev(edges[9]), (0, 2)), // face 2 right edge == rev(face 5 right edge), right -> left
            (edges[4], edges[6], (1, 2)), // face 2 bottom edge == face 3 right edge, down -> left
            (edges[5], edges[8], (2, 1)), // face 3 left edge == face 4 top edge, left -> down
            (edges[6], edges[4], (0, 3)), // face 3 right edge == face 2 bottom edge, right -> up
            (edges[7], rev(edges[0]), (2, 0)), // face 4 left edge == rev(face 1 left edge), left -> right
            (edges[8], edges[5], (3, 0)),      // face 4 top edge == face 3 left edge, up -> right
            (edges[9], rev(edges[3]), (0, 2)), // face 5 right edge == rev(face 2 right edge), right -> left
            (edges[10], edges[13], (1, 2)), // face 5 bottom edge == face 6 right edge, down -> left
            (edges[11], edges[1], (2, 1)),  // face 6 left edge == face 1 top edge, left -> down
            (edges[12], edges[2], (1, 1)),  // face 6 bottom edge == face 2 top edge, down -> down
            (edges[13], edges[10], (0, 3)), // face 6 right edge == face 5 bottom edge, right -> up
        ];

        let mut cur_step = 0;
        while cur_step < step {
            let coord = self.move_with_facing()(self.coord());
            if let Some(t) = map.get(&coord) {
                cur_step += 1;
                match t {
                    Tile::Open => {
                        (self.row, self.column) = coord;
                    }
                    Tile::Wall => break,
                }
            } else {
                // on the edge
                if let Some(flag) = wrap(self, &connected_edges, map) {
                    if flag {
                        cur_step += 1;
                    } else {
                        unreachable!()
                    }
                } else {
                    // on the other face still a wall
                    break;
                }
            }
        }

        fn wrap(state: &mut State, edges: &[ConnectedEdge], map: &Map) -> Option<bool> {
            for (e1, e2, (f1, f2)) in edges {
                if on_edge(&state.coord(), e1) && state.facing == *f1 {
                    let next_coord = wrap_coord(state.coord(), e1, e2);
                    if let Some(t) = map.get(&next_coord) {
                        match t {
                            Tile::Open => {
                                state.facing = *f2;
                                (state.row, state.column) = next_coord;
                                return Some(true);
                            }
                            Tile::Wall => return None,
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            Some(false)
        }

        fn on_edge(c: &Coord, edge: &Edge) -> bool {
            ((c.0 == edge.0 .0 && c.0 == edge.1 .0) || (c.1 == edge.0 .1 && c.1 == edge.1 .1))
                && dis(&edge.0, &edge.1) == dis(c, &edge.0) + dis(c, &edge.1)
        }

        fn row_edge(edge: &Edge) -> bool {
            edge.0 .0 == edge.1 .0
        }

        fn column_edge(edge: &Edge) -> bool {
            edge.0 .1 == edge.1 .1
        }

        fn wrap_coord(c: Coord, src: &Edge, dest: &Edge) -> Coord {
            let d_row = (src.0 .0 - c.0).abs();
            let d_col = (src.0 .1 - c.1).abs();
            let v1;
            let v2;
            if row_edge(src) && row_edge(dest) {
                v1 = (dest.0 .0, dest.0 .1 - d_col);
                v2 = (dest.0 .0, dest.0 .1 + d_col);
            } else if column_edge(src) && column_edge(dest) {
                v1 = (dest.0 .0 - d_row, dest.0 .1);
                v2 = (dest.0 .0 + d_row, dest.0 .1);
            } else {
                v1 = (dest.0 .0 - d_col, dest.0 .1 - d_row);
                v2 = (dest.0 .0 + d_col, dest.0 .1 + d_row);
            }
            if on_edge(&v1, dest) {
                v1
            } else {
                assert!(on_edge(&v2, dest));
                v2
            }
        }

        fn dis(c1: &Coord, c2: &Coord) -> i32 {
            (c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)) as i32
        }
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Coord, Tile>,
    bottom_right: Coord,
    cube_size: i32,
}

impl Map {
    fn get(&self, k: &Coord) -> Option<&Tile> {
        self.map.get(k)
    }

    fn part1_wrap(&self, c: &Coord) -> Coord {
        (
            c.0.rem_euclid(self.bottom_right.0),
            c.1.rem_euclid(self.bottom_right.1),
        )
    }

    fn set_cube_size(&mut self, size: i32) {
        self.cube_size = size;
    }

    #[allow(dead_code)]
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
        err!("There is not open tile on the first row")
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
        let map = Map {
            map,
            bottom_right,
            cube_size: 0,
        };

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
    // assert_eq!(part2(&mut map, &path, 4).unwrap(), 5031); // part 2 only work for my input
}
