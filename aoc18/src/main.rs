use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = i32;
type Position = (Coord, Coord, Coord);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let cubes = input
        .lines()
        .map(|l| from_str(l))
        .collect::<Result<Vec<Position>>>()?;
    assert_eq!(cubes.len(), input.lines().count());

    part1(&cubes)?;
    part2(&cubes)?;
    Ok(())
}

fn part1(cubes: &[Position]) -> Result<i32> {
    let start = Instant::now();
    let result = surface_area(cubes);
    writeln!(io::stdout(), "Part1: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(cubes: &[Position]) -> Result<i32> {
    let start = Instant::now();
    let cubes_pos: HashSet<Position> = HashSet::from_iter(cubes.iter().cloned());
    let points: HashSet<Position> = HashSet::from_iter(
        cubes
            .iter()
            .map(|&c| points(c))
            .flatten()
            .filter(|p| !cubes_pos.contains(p)),
    );
    let range = ranges(&cubes);
    let mut result = surface_area(&cubes);
    let mut visited = HashMap::new();
    for pos in points {
        if !visited.contains_key(&pos) {
            result -= surface_area(&dfs(&cubes_pos, pos, &mut visited, range));
        }
    }

    writeln!(io::stdout(), "Part2: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(
    cubes: &HashSet<Position>,
    pos: Position,
    visited: &mut HashMap<Position, i32>,
    range: (Position, Position),
) -> Vec<Position> {
    if !in_range(range, pos) {
        visited.insert(pos, 0);
        return vec![];
    }
    if visited.contains_key(&pos) {
        unreachable!()
    }
    visited.insert(pos, 1);
    let adjacent_pos = adjacent_position(pos);
    let adjacent_cubes: Vec<Position> = adjacent_pos
        .into_iter()
        .filter(|p| !cubes.contains(p))
        .collect();
    let mut result = vec![pos];
    if adjacent_cubes.len() == 0 {
        // 1x1x1 air cube
        result = vec![pos];
    } else {
        for next in adjacent_cubes {
            if let Some(&r) = visited.get(&next) {
                if r == 0 {
                    result.clear();
                    break;
                }
            } else {
                let r = dfs(cubes, next, visited, range);
                if r.len() == 0 {
                    result.clear();
                    break;
                } else {
                    result.extend(r.iter());
                }
            }
        }
    }
    if result.len() == 0 {
        visited.insert(pos, 0);
    }
    result
}

fn surface_area(cubes: &[Position]) -> i32 {
    let l = cubes.len();
    if l == 0 {
        return 0;
    }
    let mut result = l as i32 * 6;
    for i in 0..l {
        let c1 = cubes[i];
        for j in i + 1..l {
            let c2 = cubes[j];
            if connected(c1, c2) {
                result -= 2;
            }
        }
    }
    result
}
fn connected(p1: Position, p2: Position) -> bool {
    dis(p1, p2) == 1
}

fn points(pos: Position) -> Vec<Position> {
    let (x, y, z) = pos;
    let mut result = vec![
        (x, y, z),
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x - 1, y - 1, z),
        (x - 1, y, z),
        (x, y - 1, z - 1),
        (x - 1, y - 1, z - 1),
    ];
    result.sort();
    // assert_eq!(result.last(), Some(&self.positon()));
    result
}

fn adjacent_position(pos: Position) -> Vec<Position> {
    let (x, y, z) = pos;
    vec![
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),
    ]
}

fn ranges(cubes: &[Position]) -> (Position, Position) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;
    for c in cubes {
        min_x = min_x.min(c.0);
        min_y = min_z.min(c.1);
        min_z = min_z.min(c.2);
        max_x = max_x.max(c.0);
        max_y = max_y.max(c.1);
        max_z = max_z.max(c.2);
    }
    (
        (min_x - 1, min_y - 1, min_z - 1),
        (max_x + 1, max_y + 1, max_z + 1),
    )
}

fn in_range(range: (Position, Position), pos: Position) -> bool {
    dis(range.0, range.1) == dis(pos, range.0) + dis(pos, range.1)
}

fn dis(p1: Position, p2: Position) -> Coord {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2)) as Coord
}

fn from_str(s: &str) -> Result<Position> {
    let coords: Vec<&str> = s.trim().split(",").collect();
    if coords.len() == 3 {
        return Ok((coords[0].parse()?, coords[1].parse()?, coords[2].parse()?));
    }
    err!("not a valid position: {}", s)
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_input() {
        use crate::*;

        let input = "2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5";
        let cubes: Vec<Position> = input.lines().map(|l| from_str(l).unwrap()).collect();
        assert_eq!(cubes.len(), input.lines().count());
        assert_eq!(connected((1, 1, 1), (2, 1, 1)), true);
        assert_eq!(connected((2, 1, 1), (2, 2, 1)), true);
        assert_eq!(connected((1, 1, 1), (2, 2, 1)), false);
        assert_eq!(part1(&cubes).unwrap(), 64);
        assert_eq!(part2(&cubes).unwrap(), 58);
    }

    #[test]
    fn test_one_air_cube() {
        use crate::*;

        let cubes = vec![
            (1, 1, 1),
            (2, 2, 1),
            (3, 1, 1),
            (2, 1, 2),
            (2, 1, 0),
            (2, 0, 1),
        ];
        assert_eq!(part2(&cubes).unwrap(), 30);
    }

    #[test]
    fn test_two_air_cube() {
        use crate::*;

        let cubes = vec![
            (1, 1, 1),
            (2, 1, 2),
            (3, 1, 1),
            (1, 1, 0),
            (3, 1, 0),
            (2, 1, -1),
            (2, 2, 1),
            (2, 2, 0),
            (2, 0, 1),
            (2, 0, 0),
        ];
        assert_eq!(part1(&cubes).unwrap(), 52);
        assert_eq!(part2(&cubes).unwrap(), 42);
    }
}
