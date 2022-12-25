use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = i32;
type Vertex = (Coord, Coord, Coord);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let cubes = input
        .lines()
        .map(from_str)
        .collect::<Result<Vec<Vertex>>>()?;
    assert_eq!(cubes.len(), input.lines().count());

    part1(&cubes)?;
    part2(&cubes)?;
    part2_with_flood_fill(&cubes)?;
    Ok(())
}

fn part1(cubes: &[Vertex]) -> Result<i32> {
    let start = Instant::now();
    let result = surface_area(cubes);
    writeln!(io::stdout(), "Part1: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(cubes: &[Vertex]) -> Result<i32> {
    let start = Instant::now();
    let cubes_set: HashSet<Vertex> = HashSet::from_iter(cubes.iter().cloned());
    let vertices: HashSet<Vertex> = HashSet::from_iter(
        cubes
            .iter()
            .flat_map(|&c| adjacent_vertices(c))
            .filter(|p| !cubes_set.contains(p)),
    );
    let range = ranges(cubes);
    let mut result = surface_area(cubes);
    let mut visited = HashMap::new();
    for vertex in vertices {
        if !visited.contains_key(&vertex) {
            result -= surface_area(&dfs(&cubes_set, vertex, &mut visited, range));
        }
    }
    writeln!(io::stdout(), "Part2: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(
    cubes: &HashSet<Vertex>,
    vertex: Vertex,
    visited: &mut HashMap<Vertex, i32>,
    range: (Vertex, Vertex),
) -> Vec<Vertex> {
    if !in_range(range, vertex) {
        visited.insert(vertex, 0);
        return vec![];
    }
    if visited.contains_key(&vertex) {
        return vec![];
    }
    visited.insert(vertex, 1);
    let mut result = vec![vertex];
    let adjacent_cubes: Vec<Vertex> = adjacent_vertices(vertex)
        .into_iter()
        .filter(|p| !cubes.contains(p))
        .collect();
    for next in adjacent_cubes {
        if let Some(&r) = visited.get(&next) {
            if r == 0 {
                result.clear();
                break;
            }
        } else {
            let r = dfs(cubes, next, visited, range);
            if r.is_empty() {
                result.clear();
                break;
            } else {
                result.extend(r.iter());
            }
        }
    }
    if result.is_empty() {
        visited.insert(vertex, 0);
    }
    result
}

fn part2_with_flood_fill(cubes: &[Vertex]) -> Result<i32> {
    let start = Instant::now();
    let cubes_set: HashSet<Vertex> = HashSet::from_iter(cubes.iter().cloned());
    let range = ranges(cubes);
    let result = flood(range.0, &cubes_set, range);

    writeln!(io::stdout(), "Part2 with flood fill: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn flood(start: Vertex, cubes: &HashSet<Vertex>, range: (Vertex, Vertex)) -> i32 {
    let mut stack = Vec::new();
    let mut sum = 0;
    let mut visited = HashSet::new();
    stack.push(start);
    while let Some(cur) = stack.pop() {
        if visited.insert(cur) && in_range(range, cur) {
            for next in adjacent_vertices(cur) {
                if cubes.contains(&next) {
                    sum += 1;
                } else {
                    stack.push(next);
                }
            }
        }
    }
    sum
}

fn surface_area(cubes: &[Vertex]) -> i32 {
    let l = cubes.len();
    if l == 0 {
        return 0;
    }
    let mut result = l as i32 * 6;
    for i in 0..l {
        let c1 = cubes[i];
        for &c2 in cubes.iter().skip(i + 1) {
            if connected(c1, c2) {
                result -= 2;
            }
        }
    }
    result
}
fn connected(v1: Vertex, v2: Vertex) -> bool {
    dis(v1, v2) == 1
}

fn adjacent_vertices(vertex: Vertex) -> Vec<Vertex> {
    let (x, y, z) = vertex;
    vec![
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),
    ]
}

fn ranges(cubes: &[Vertex]) -> (Vertex, Vertex) {
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

fn in_range(range: (Vertex, Vertex), vertex: Vertex) -> bool {
    dis(range.0, range.1) == dis(vertex, range.0) + dis(vertex, range.1)
}

fn dis(p1: Vertex, p2: Vertex) -> Coord {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2)) as Coord
}

fn from_str(s: &str) -> Result<Vertex> {
    let coords: Vec<&str> = s.trim().split(',').collect();
    if coords.len() == 3 {
        return Ok((coords[0].parse()?, coords[1].parse()?, coords[2].parse()?));
    }
    err!("not a valid vertex: {}", s)
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
        let cubes: Vec<Vertex> = input.lines().map(|l| from_str(l).unwrap()).collect();
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
