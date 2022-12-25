use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

use regex::Regex;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type CoordType = i64;
type Coord = (CoordType, CoordType);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut sensors = parse_report(&input)?;
    sensors.sort_by_key(|s| s.min_x());

    part1(&sensors, 2000000)?;
    part2(&sensors, 4000000)?;
    part1_with_interval(&sensors, 2000000)?;
    part2_with_interval(&sensors, 4000000)?;
    Ok(())
}

fn part1(sensors: &[Sensor], y: CoordType) -> Result<CoordType> {
    let start = Instant::now();
    let min_x = sensors.iter().map(|s| s.min_x()).min().unwrap();
    let max_x = sensors.iter().map(|s| s.max_x()).max().unwrap();
    let result = (min_x..=max_x)
        .filter(|&x| {
            // a position isn't a beacon for a sensor then it must not be a beacon
            sensors.iter().any(|s| !s.could_be_beacon((x, y)))
        })
        .count();
    writeln!(io::stdout(), "Part1: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result as CoordType)
}

fn part2(sensors: &[Sensor], max: CoordType) -> Result<CoordType> {
    let start = Instant::now();
    let mut result = 0;
    let mut y = -1;
    while y <= max && result == 0 {
        y += 1;
        let mut x = 0;
        let mut flag;
        while x <= max && result == 0 {
            flag = false;
            for s in sensors {
                if !s.could_be_beacon((x, y)) {
                    flag = true;
                    x = s.furthest_horizontal((x, y)).0 + 1;
                    break;
                }
            }
            if !flag {
                result = x * 4000000 + y;
            }
        }
    }

    writeln!(io::stdout(), "Part2: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part1_with_interval(sensors: &[Sensor], y: CoordType) -> Result<CoordType> {
    let start = Instant::now();
    let result = intervals_at(sensors, y)
        .0
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum::<CoordType>()
        - sensors
            .iter()
            .map(|s| s.beacon)
            .filter(|b| b.1 == y)
            .collect::<HashSet<_>>()
            .len() as CoordType;
    writeln!(io::stdout(), "Part1 with interval: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2_with_interval(sensors: &[Sensor], max: CoordType) -> Result<CoordType> {
    let start = Instant::now();
    let mut result = 0;
    let mut y = 0;
    while y <= max {
        let (intervals, temp) = intervals_at(sensors, y);
        if intervals.len() > 1 {
            result = y + 4000000 * (intervals[0].1 + 1);
            break;
        }
        y += (temp + 1) / 2;
    }
    writeln!(io::stdout(), "Part2 with interval: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn intervals_at(sensors: &[Sensor], y: CoordType) -> (Vec<(CoordType, CoordType)>, CoordType) {
    let mut raw: Vec<_> = sensors.iter().filter_map(|s| s.interval_at(y)).collect();
    raw.sort();
    let mut intervals = vec![raw[0]];
    let mut min_overlap = CoordType::MAX;
    for &interval in &raw[1..] {
        let last = intervals.last_mut().unwrap();
        if interval.0 > last.1 + 1 {
            min_overlap = 0;
            intervals.push(interval)
        } else {
            min_overlap = min_overlap.min(last.1 - interval.0 + 1);
            last.1 = last.1.max(interval.1)
        }
    }
    (intervals, min_overlap)
}

#[derive(Debug)]
struct Sensor {
    coord: Coord,
    beacon: Coord,
    closest_dis: CoordType,
}

impl Sensor {
    fn min_x(&self) -> CoordType {
        self.coord.0 - self.closest_dis
    }
    fn max_x(&self) -> CoordType {
        self.coord.0 + self.closest_dis
    }
    #[allow(dead_code)]
    fn min_y(&self) -> CoordType {
        self.coord.1 - self.closest_dis
    }
    #[allow(dead_code)]
    fn max_y(&self) -> CoordType {
        self.coord.1 + self.closest_dis
    }

    fn dis(&self, c: Coord) -> CoordType {
        (self.coord.0 - c.0).abs() + (self.coord.1 - c.1).abs()
    }

    fn could_be_beacon(&self, c: Coord) -> bool {
        // There is never a tie where two beacons are the same distance to a sensor.
        self.dis(c) > self.closest_dis || c == self.beacon
    }

    fn furthest_horizontal(&self, c: Coord) -> Coord {
        let y = c.1;
        let x1 = self.closest_dis - (y - self.coord.1).abs() + self.coord.0;
        let x2 = -(self.closest_dis - (y - self.coord.1).abs()) + self.coord.0;
        (x1.max(x2), y)
    }

    #[allow(dead_code)]
    fn furthest_vertical(&self, c: Coord) -> Coord {
        let x = c.0;
        let y1 = self.closest_dis - (x - self.coord.0).abs() + self.coord.1;
        let y2 = -(self.closest_dis - (x - self.coord.0).abs()) + self.coord.1;
        (x, y1.max(y2))
    }

    fn interval_at(&self, y: CoordType) -> Option<(CoordType, CoordType)> {
        if (y - self.coord.1).abs() > self.closest_dis {
            return None;
        }
        let x1 = self.closest_dis - (y - self.coord.1).abs() + self.coord.0;
        let x2 = -(self.closest_dis - (y - self.coord.1).abs()) + self.coord.0;
        Some((x1.min(x2), x1.max(x2)))
    }
}

fn parse_report(s: &str) -> Result<Vec<Sensor>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }
    let reports: Vec<Sensor> = RE
        .captures_iter(s)
        .filter_map(|cap| {
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
            match groups {
                (Some(s_x), Some(s_y), Some(b_x), Some(b_y)) => {
                    let sensor = (
                        s_x.as_str().parse::<CoordType>().unwrap(),
                        s_y.as_str().parse::<CoordType>().unwrap(),
                    );
                    let beacon = (
                        b_x.as_str().parse::<CoordType>().unwrap(),
                        b_y.as_str().parse::<CoordType>().unwrap(),
                    );
                    Some(Sensor {
                        coord: sensor,
                        beacon,
                        closest_dis: (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs(),
                    })
                }
                _ => None,
            }
        })
        .collect();
    assert_eq!(
        reports.len(),
        s.lines().filter(|s| !s.trim().is_empty()).count()
    );
    Ok(reports)
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_input() {
        use crate::{parse_report, part1, part1_with_interval, part2, part2_with_interval};

        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let sensors = parse_report(input).unwrap();

        assert_eq!(sensors[6].coord, (8, 7));
        assert_eq!(sensors[6].could_be_beacon((2, 10)), true);
        assert_eq!(sensors[6].could_be_beacon((3, 10)), false);
        assert_eq!(sensors[6].could_be_beacon((8, 16)), false);
        assert_eq!(26, part1(&sensors, 10).unwrap());
        assert_eq!(26, part1_with_interval(&sensors, 10).unwrap());
        assert_eq!(56000011, part2(&sensors, 20).unwrap());
        assert_eq!(56000011, part2_with_interval(&sensors, 20).unwrap());
    }
}
