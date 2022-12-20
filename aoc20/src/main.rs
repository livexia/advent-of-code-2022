use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums = parse_input(&input);
    part1(&nums)?;
    part2(&nums)?;
    Ok(())
}

fn part1(nums: &[(i64, usize)]) -> Result<i64> {
    let start = Instant::now();

    let mut nums = nums.to_vec();
    mixing(&mut nums);
    let zero_index = nums.iter().position(|n| n.0 == 0).unwrap();
    let result = [1000, 2000, 3000].iter().fold(0, |sum, &offset| {
        sum + nums[(zero_index + offset) % nums.len()].0
    });
    writeln!(io::stdout(), "Part1: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(nums: &[(i64, usize)]) -> Result<i64> {
    let start = Instant::now();

    let mut nums: Vec<(i64, usize)> = nums
        .iter()
        .cloned()
        .map(|(n, i)| (n * 811589153, i))
        .collect();
    for _ in 0..10 {
        mixing(&mut nums);
    }
    let zero_index = nums.iter().position(|n| n.0 == 0).unwrap();
    let result = [1000, 2000, 3000].iter().fold(0, |sum, &offset| {
        sum + nums[(zero_index + offset) % nums.len()].0
    });
    writeln!(io::stdout(), "Part1: {}", result)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn mixing(nums: &mut Vec<(i64, usize)>) {
    let length = nums.len();
    let mut cur_index = 0;
    for _ in 0..length {
        for i in 0..length {
            if nums[i].1 == cur_index {
                // never moved
                let (offset, index) = nums[i];
                let next = wrap(i as i64, offset, length as i64);
                nums.remove(i);
                nums.insert(next, (offset, index));
                cur_index += 1;
                break;
            }
        }
    }
}

fn wrap(cur: i64, offset: i64, length: i64) -> usize {
    if cur + offset > 0 && cur + offset < length {
        return (cur + offset) as usize;
    }
    let mut next = cur + offset;
    if next < 0 {
        next = next % (length - 1) + length - 1;
    } else {
        next = next % (length - 1);
    }
    next as usize
}

fn parse_input(input: &str) -> Vec<(i64, usize)> {
    // there are duplicate number
    input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect()
}

#[test]
fn example_input() {
    let input = "1
    2
    -3
    3
    -2
    0
    4";
    let nums = parse_input(input);
    assert_eq!(3, part1(&nums).unwrap());
    assert_eq!(1623178306, part2(&nums).unwrap());

    let mut t = vec![(3, 0), (1, 1), (0, 2)];
    mixing(&mut t);
    assert_eq!(vec![(3, 0), (1, 1), (0, 2)], t);
}
