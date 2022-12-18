use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = usize;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let cubes = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Cube>>>()?;
    assert_eq!(cubes.len(), input.lines().count());

    part1(&cubes)?;
    // part2()?;
    Ok(())
}

fn part1(cubes: &[Cube]) -> Result<usize> {
    let start = Instant::now();
    let l = cubes.len();
    let mut result = l * 6;
    for i in 0..l {
        let c1 = &cubes[i];
        for j in i + 1..l {
            let c2 = &cubes[j];
            if c1.connected(c2) {
                result -= 2;
            }
        }
    }
    writeln!(io::stdout(), "Part1: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

struct Cube {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Cube {
    fn new(x: Coord, y: Coord, z: Coord) -> Self {
        Self { x, y, z }
    }
    fn connected(&self, other: &Cube) -> bool {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z) == 1
    }

    fn sides(&self) -> Vec<(Coord, Coord)> {
        todo!()
    }
}

impl FromStr for Cube {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let coords: Vec<&str> = s.trim().split(",").collect();
        if coords.len() == 3 {
            return Ok(Cube {
                x: coords[0].parse()?,
                y: coords[1].parse()?,
                z: coords[2].parse()?,
            });
        }
        err!("not a valid position: {}", s)
    }
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
        let cubes: Vec<Cube> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(cubes.len(), input.lines().count());
        assert_eq!(Cube::new(1, 1, 1).connected(&Cube::new(2, 1, 1)), true);
        assert_eq!(Cube::new(2, 1, 1).connected(&Cube::new(2, 2, 1)), true);
        assert_eq!(Cube::new(1, 1, 1).connected(&Cube::new(2, 2, 1)), false);
        assert_eq!(part1(&cubes).unwrap(), 64);
    }
}
