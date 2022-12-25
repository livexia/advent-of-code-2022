use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Decimal = i64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let snafu_numbers: Vec<&str> = input.lines().map(|l| l.trim()).collect();

    part1(&snafu_numbers)?;
    // part2()?;
    Ok(())
}

fn part1(snafu_numbers: &[&str]) -> Result<String> {
    let start = Instant::now();
    let result: Decimal = snafu_numbers
        .iter()
        .map(|n| snafu_to_decimal(n))
        .sum::<Result<Decimal>>()?;
    let result = decimal_to_snafu(result);
    writeln!(io::stdout(), "Part1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn snafu_to_decimal(s: &str) -> Result<Decimal> {
    // 2 1 0 - =
    let snafu = s.trim().chars().rev();
    let mut num = 0;
    let mut base = 1;
    for c in snafu {
        num += base
            * match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => return err!("not a valid SNAFU number {}", s),
            };
        base *= 5;
    }

    Ok(num)
}

fn decimal_to_snafu(num: Decimal) -> String {
    let mut snafu = String::new();
    // 4890 => 2=-1=0
    let mut num = num;
    while num > 0 {
        let remain = num % 5;
        num /= 5;
        match remain {
            0 => snafu.push('0'),
            1 => snafu.push('1'),
            2 => snafu.push('2'),
            3 => {
                snafu.push('=');
                num += 1
            }
            4 => {
                snafu.push('-');
                num += 1
            }
            _ => unreachable!(),
        }
    }

    snafu.chars().rev().collect()
}

#[test]
fn example_input() {
    let input = "1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122";
    assert_eq!(snafu_to_decimal("1=-0-2").unwrap(), 1747);
    let snafu_numbers: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    assert_eq!(&part1(&snafu_numbers).unwrap(), "2=-1=0");
}
