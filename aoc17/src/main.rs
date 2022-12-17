use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i64, i64);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let jets: Vec<char> = input.trim().chars().map(|c| [c, 'v']).flatten().collect();

    assert_eq!(part1(&jets, 2022)?, 3224);
    assert_eq!(part2(&jets, 1000000)?, 1595973);
    // part2(&jets, 1000000000000)?;
    Ok(())
}

fn part1(jets: &[char], total_rock: i64) -> Result<i64> {
    let start = Instant::now();
    let highest_rock = rock_tower(jets, total_rock)?;
    writeln!(io::stdout(), "Part1: {}", highest_rock)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(highest_rock)
}

fn part2(jets: &[char], total_rock: i64) -> Result<i64> {
    let start = Instant::now();
    let highest_rock = rock_tower(jets, total_rock)?;
    writeln!(io::stdout(), "Part2: {}", highest_rock)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(highest_rock)
}

fn rock_tower(jets: &[char], total_rock: i64) -> Result<i64> {
    let mut chamber: HashSet<Coord> = HashSet::new();

    let cycle = jets.len() as i64 / 2 * 5;
    let mut last_height = 0;

    use RockShape::*;
    let rocks = [Ih, X, J, Iv, O];
    let mut jets = jets.iter().cycle();

    let mut rock_count = 0;
    let mut highest_rock = 0;
    let mut start = Instant::now();
    let mut floor = 0;
    for &shape in rocks.iter().cycle() {
        if (rock_count - 1) % 13 == 0 {
            println!(
                "{} {} {} {}",
                cycle,
                rock_count,
                highest_rock,
                highest_rock - last_height,
            );
            last_height = highest_rock;
            for y in (highest_rock - 12..=highest_rock).rev() {
                for x in 0..7 {
                    if chamber.contains(&(x, y)) {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
                println!()
            }
        }
        if rock_count == total_rock {
            break;
        }
        // if rock_count % 100000 == 0 {
        //     let old_floor = floor;
        //     for y in (floor..=highest_rock).rev() {
        //         if (0..7).all(|x| chamber.contains(&(x, y))) {
        //             floor = y;
        //             break;
        //         }
        //     }
        //     for x in 0..7 {
        //         for y in old_floor..floor {
        //             chamber.remove(&(x, y));
        //         }
        //     }
        //     println!(
        //         "{} {:?} {}",
        //         highest_rock,
        //         Instant::now() - start,
        //         chamber.len()
        //     );
        //     start = Instant::now();
        // }
        let mut rock = Rock::new(shape, highest_rock);
        rock_count += 1;
        while let Some(&movement) = jets.next() {
            match movement {
                '<' => {
                    let next_rock = rock.push_left();
                    if !next_rock.is_left_collided(&chamber) {
                        rock = next_rock;
                    }
                }
                '>' => {
                    let next_rock = rock.push_right();
                    if !next_rock.is_right_collided(&chamber) {
                        rock = next_rock;
                    }
                }
                'v' => {
                    let next_rock = rock.fall_down();
                    if next_rock.is_bottom_collided(&chamber, floor) {
                        rock.occupy().into_iter().for_each(|c| {
                            chamber.insert(c);
                        });
                        highest_rock = highest_rock.max(rock.top_edge() + 1);
                        break;
                    };
                    rock = next_rock;
                }
                _ => unreachable!(),
            };
        }
    }
    Ok(highest_rock)
}

#[derive(Debug)]
struct Rock {
    shape: RockShape,
    top_left_pos: Coord,
}

impl Rock {
    fn new(shape: RockShape, highest_rock: i64) -> Self {
        use RockShape::*;
        match shape {
            Ih => Self {
                shape,
                top_left_pos: (2, highest_rock + 3),
            },
            Iv => Self {
                shape,
                top_left_pos: (2, highest_rock + 6),
            },
            J => Self {
                shape,
                top_left_pos: (2, highest_rock + 5),
            },
            O => Self {
                shape,
                top_left_pos: (2, highest_rock + 4),
            },
            X => Self {
                shape,
                top_left_pos: (2, highest_rock + 5),
            },
        }
    }

    fn occupy(&self) -> Vec<Coord> {
        let (x, y) = self.top_left_pos;
        match self.shape {
            RockShape::Ih => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockShape::Iv => vec![(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
            RockShape::J => vec![
                (x + 2, y),
                (x + 2, y - 1),
                (x, y - 2),
                (x + 1, y - 2),
                (x + 2, y - 2),
            ],
            RockShape::O => vec![(x, y), (x + 1, y), (x, y - 1), (x + 1, y - 1)],
            RockShape::X => {
                vec![
                    (x + 1, y),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x + 2, y - 1),
                    (x + 1, y - 2),
                ]
            }
        }
    }

    fn top_edge(&self) -> i64 {
        self.top_left_pos.1
    }

    fn left_edge(&self) -> i64 {
        self.top_left_pos.0
    }

    fn right_edge(&self) -> i64 {
        self.top_left_pos.0
            + match self.shape {
                RockShape::Ih => 3,
                RockShape::Iv => 0,
                RockShape::J => 2,
                RockShape::O => 1,
                RockShape::X => 2,
            }
    }

    fn bottom_edge(&self) -> i64 {
        self.top_left_pos.1
            - match self.shape {
                RockShape::Ih => 0,
                RockShape::Iv => 3,
                RockShape::J => 2,
                RockShape::O => 1,
                RockShape::X => 2,
            }
    }

    fn push_left(&self) -> Self {
        Self {
            shape: self.shape,
            top_left_pos: (self.top_left_pos.0 - 1, self.top_left_pos.1),
        }
    }

    fn push_right(&self) -> Self {
        Self {
            shape: self.shape,
            top_left_pos: (self.top_left_pos.0 + 1, self.top_left_pos.1),
        }
    }

    fn fall_down(&self) -> Self {
        Self {
            shape: self.shape,
            top_left_pos: (self.top_left_pos.0, self.top_left_pos.1 - 1),
        }
    }

    fn is_bottom_collided(&self, chamber: &HashSet<Coord>, floor: i64) -> bool {
        self.bottom_edge() < floor
            || match self.shape {
                RockShape::Ih => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 3, self.top_left_pos.1))
                }
                RockShape::Iv => chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 3)),
                RockShape::J => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1 - 2))
                }
                RockShape::O => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1 - 1))
                }
                RockShape::X => {
                    chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 1))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1 - 1))
                }
            }
    }

    fn is_left_collided(&self, chamber: &HashSet<Coord>) -> bool {
        self.left_edge() < 0
            || match self.shape {
                RockShape::Ih => chamber.contains(&self.top_left_pos),
                RockShape::Iv => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 1))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 3))
                }
                RockShape::J => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1 - 1))
                }
                RockShape::O => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 1))
                }
                RockShape::X => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1 - 2))
                }
            }
    }

    fn is_right_collided(&self, chamber: &HashSet<Coord>) -> bool {
        self.right_edge() >= 7
            || match self.shape {
                RockShape::Ih => chamber.contains(&(self.top_left_pos.0 + 3, self.top_left_pos.1)),
                RockShape::Iv => {
                    chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 1))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0, self.top_left_pos.1 - 3))
                }
                RockShape::J => {
                    chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1 - 2))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1 - 1))
                }
                RockShape::O => {
                    chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1 - 1))
                }
                RockShape::X => {
                    chamber.contains(&(self.top_left_pos.0 + 2, self.top_left_pos.1 - 1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1))
                        || chamber.contains(&(self.top_left_pos.0 + 1, self.top_left_pos.1 - 2))
                }
            }
    }

    fn valid(&self, chamber: &HashSet<Coord>) -> bool {
        self.left_edge() >= 0
            && self.right_edge() < 7
            && self.bottom_edge() >= 0
            && !self.is_collided(chamber)
    }

    fn is_collided(&self, chamber: &HashSet<Coord>) -> bool {
        let coords = self.occupy();
        coords.iter().any(|c| c.1 < 0 || chamber.contains(c))
    }
}

#[derive(Debug, Clone, Copy)]
enum RockShape {
    Ih,
    Iv,
    J,
    O,
    X,
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_input() {
        use crate::*;

        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets: Vec<_> = input.trim().chars().map(|c| [c, 'v']).flatten().collect();
        assert_eq!(rock_tower(&jets, 1).unwrap(), 1);
        assert_eq!(rock_tower(&jets, 2).unwrap(), 4);
        assert_eq!(rock_tower(&jets, 3).unwrap(), 6);
        assert_eq!(rock_tower(&jets, 4).unwrap(), 7);
        assert_eq!(rock_tower(&jets, 5).unwrap(), 9);
        assert_eq!(rock_tower(&jets, 6).unwrap(), 10);
        assert_eq!(part1(&jets, 2022).unwrap(), 3068);
        assert_eq!(part2(&jets, 10000).unwrap(), 15148);
        // assert_eq!(part2(&jets, 1000000000000).unwrap(), 1514285714288);
    }
}
