use std::collections::HashMap;
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

    assert_eq!(part1(&jets, 277)?, 439);
    assert_eq!(part1(&jets, 2022)?, 3224);
    assert_eq!(part1(&jets, 10000)?, 15984);
    assert_eq!(part1(&jets, 100000)?, 159620);
    assert_eq!(part2(&jets, 1000000)?, 1595973);
    assert_eq!(part2(&jets, 5000000)?, 7979964);
    part2(&jets, 1000000000000)?;
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

    use RockShape::*;
    let rocks = [Ih, X, J, Iv, O];
    let mut jets = jets.iter().enumerate().cycle().peekable();

    let mut memorization: HashMap<(usize, usize, Vec<i64>), (i64, i64)> = HashMap::new();
    let mut highests = vec![-1; 7];
    let mut last_cycle = 0;

    let mut rock_count = 0;
    let mut highest_rock = 0;
    for (shape_id, &shape) in rocks.iter().enumerate().cycle().peekable() {
        if rock_count == total_rock {
            break;
        }

        let mut rock = Rock::new(shape, highest_rock);
        rock_count += 1;

        let key = (
            shape_id,
            jets.peek().unwrap().0,
            highests.iter().map(|h| highest_rock - h).collect(),
        );
        if let Some((last_rock, last_highest)) = memorization.get(&key) {
            let cycle = rock_count - last_rock;
            let cycle_inc = highest_rock - last_highest;
            let cycle_mod = total_rock % cycle;
            let remain_rock = total_rock - rock_count;
            let cycle_count = remain_rock / cycle;
            if cycle_mod == rock_count % cycle {
                return Ok(cycle_count * cycle_inc + highest_rock);
            }
            if last_cycle != cycle {
                println!("{} {} {}", rock_count, last_rock, cycle);
                memorization.clear();
                last_cycle = cycle;
            }
        }
        while let Some((_, &movement)) = jets.next() {
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
                    if next_rock.is_bottom_collided(&chamber) {
                        rock.occupy().into_iter().for_each(|c| {
                            chamber.insert(c);
                        });
                        highest_rock =
                            highest_rock.max(rock.top().iter().map(|(_, y)| y).max().unwrap() + 1);
                        for (x, y) in rock.top() {
                            highests[x as usize] = highests[x as usize].max(y + 1);
                        }
                        memorization.insert(key, (rock_count, highest_rock));
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

fn print_last_ten(chamber: &HashSet<Coord>, height: i64) {
    for y in (height - 10..=height).rev() {
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

    fn top(&self) -> Vec<Coord> {
        let (x, y) = self.top_left_pos;
        match self.shape {
            RockShape::Ih => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockShape::Iv => vec![(x, y)],
            RockShape::J => vec![(x + 2, y), (x, y - 2), (x + 1, y - 2)],
            RockShape::O => vec![(x, y), (x + 1, y)],
            RockShape::X => {
                vec![(x + 1, y), (x, y - 1), (x + 2, y - 1)]
            }
        }
    }

    fn left(&self) -> Vec<Coord> {
        let (x, y) = self.top_left_pos;
        match self.shape {
            RockShape::Ih => vec![(x, y)],
            RockShape::Iv => vec![(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
            RockShape::J => vec![(x, y - 2), (x + 2, y), (x + 2, y - 1)],
            RockShape::O => vec![(x, y), (x, y - 1)],
            RockShape::X => {
                vec![(x + 1, y), (x, y - 1), (x + 1, y - 2)]
            }
        }
    }

    fn right(&self) -> Vec<Coord> {
        let (x, y) = self.top_left_pos;
        match self.shape {
            RockShape::Ih => vec![(x + 3, y)],
            RockShape::Iv => vec![(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
            RockShape::J => vec![(x + 2, y), (x + 2, y - 1), (x + 2, y - 2)],
            RockShape::O => vec![(x + 1, y), (x + 1, y - 1)],
            RockShape::X => {
                vec![(x + 1, y), (x + 2, y - 1), (x + 1, y - 2)]
            }
        }
    }

    fn bottom(&self) -> Vec<Coord> {
        let (x, y) = self.top_left_pos;
        match self.shape {
            RockShape::Ih => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockShape::Iv => vec![(x, y - 3)],
            RockShape::J => vec![(x, y - 2), (x + 1, y - 2), (x + 2, y - 2)],
            RockShape::O => vec![(x, y - 1), (x + 1, y - 1)],
            RockShape::X => {
                vec![(x, y - 1), (x + 2, y - 1), (x + 1, y - 2)]
            }
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

    fn is_bottom_collided(&self, chamber: &HashSet<Coord>) -> bool {
        self.bottom().iter().any(|c| c.1 < 0 || chamber.contains(c))
    }

    fn is_left_collided(&self, chamber: &HashSet<Coord>) -> bool {
        self.left().iter().any(|c| c.0 < 0 || chamber.contains(c))
    }

    fn is_right_collided(&self, chamber: &HashSet<Coord>) -> bool {
        self.right().iter().any(|c| c.0 > 6 || chamber.contains(c))
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
        // assert_eq!(part1(&jets, 2022).unwrap(), 3068);
        // assert_eq!(part2(&jets, 10000).unwrap(), 15148);
        let cycle = 35;
        let r = part2(&jets, cycle).unwrap();
        assert_eq!(60, r);
        assert_eq!(r + 53 * 3, part2(&jets, cycle * 4).unwrap());
        assert_eq!(272, part2(&jets, cycle * 5).unwrap());
        assert_eq!(151434, part2(&jets, 100000).unwrap());
        assert_eq!(1514288, part2(&jets, 1000000).unwrap());
        assert_eq!(15142861, part2(&jets, 10000000).unwrap());
        assert_eq!(1514285714288, part2(&jets, 1000000000000).unwrap());
    }
}
