use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Num = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let pairs: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<Vec<Packet>>>()?;
    part1(&pairs)?;
    part2(&pairs)?;
    Ok(())
}

fn part1(pairs: &[Packet]) -> Result<usize> {
    let start = Instant::now();

    let sum: usize = pairs
        .chunks(2)
        .enumerate()
        .filter(|(_, p)| p[0].le(&p[1]))
        .map(|(i, _)| i + 1)
        .sum();
    writeln!(
        io::stdout(),
        "What is the sum of the indices of those pairs? {:?}",
        sum
    )?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

fn part2(pairs: &[Packet]) -> Result<usize> {
    let start = Instant::now();
    let mut pairs: Vec<Packet> = pairs.to_vec();
    pairs.sort();
    let p1 = "[[2]]".parse().unwrap();
    let index1 = match pairs.binary_search(&&p1) {
        Err(n) => n,
        Ok(n) => n,
    };
    pairs.insert(index1, p1);
    let p2 = "[[6]]".parse().unwrap();
    let index2 = match pairs.binary_search(&&p2) {
        Err(n) => n,
        Ok(n) => n,
    };
    let result = (index2 + 1) * (index1 + 1);
    writeln!(
        io::stdout(),
        "What is the decoder key for the distress signal? {:?}",
        result
    )?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(Num),
}

impl Packet {
    fn from_list(l: Vec<Packet>) -> Self {
        Self::List(l)
    }

    fn to_list(&self) -> Result<Self> {
        match self {
            Packet::List(_) => err!("Already a list"),
            Packet::Integer(n) => Ok(Self::List(vec![Self::Integer(*n)])),
        }
    }

    fn len(&self) -> usize {
        match self {
            Packet::List(l) => l.len(),
            Packet::Integer(_) => 1,
        }
    }

    fn get_list(&self) -> Result<&Vec<Packet>> {
        match self {
            Packet::List(l) => Ok(l),
            Packet::Integer(_) => err!("Not a list: {:?}", self),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(_), Packet::List(_)) => {
                for (l, r) in self
                    .get_list()
                    .unwrap()
                    .iter()
                    .zip(other.get_list().unwrap().iter())
                {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                self.len().cmp(&other.len())
            }
            (Packet::List(_), Packet::Integer(_)) => self.cmp(&other.to_list().unwrap()),
            (Packet::Integer(_), Packet::List(_)) => self.to_list().unwrap().cmp(other),
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Packet {}

impl FromStr for Packet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(num) = s.parse() {
            return Ok(Packet::Integer(num));
        }
        if s.starts_with("[") {
            let mut chars: Vec<char> = s.chars().filter(|&c| c != ' ').collect();
            let mut stack: Vec<Option<Packet>> = vec![]; // true finish packet, false un finish packet
            while let Some(c) = chars.pop() {
                match c {
                    ']' => {
                        // new list packet
                        stack.push(None);
                    }
                    '[' => {
                        // this packet is done
                        let mut temp = vec![];
                        while let Some(Some(p)) = stack.pop() {
                            temp.push(p);
                        }
                        stack.push(Some(Packet::from_list(temp)));
                    }
                    ',' => (), // next packet
                    _ => {
                        if c.is_numeric() {
                            // create a num packet
                            let mut num = vec![c];
                            while let Some(next_c) = chars.pop() {
                                if next_c.is_numeric() {
                                    num.push(next_c);
                                } else {
                                    chars.push(next_c);
                                    break;
                                }
                            }
                            let num = String::from_iter(num.iter().rev());
                            stack.push(Some(num.parse()?));
                        } else {
                            unreachable!() // not possible
                        }
                    }
                }
            }
            if let Some(Some(p)) = stack.pop() {
                if stack.is_empty() {
                    return Ok(p);
                }
            }
        }
        err!("This is not a valid packet data: {}", s)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(v) => {
                write!(f, "[")?;
                for i in 0..v.len() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v[i])?;
                }
                write!(f, "]")
            }
            Packet::Integer(n) => write!(f, "{}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_order() {
        use crate::Packet;
        use std::cmp::Ordering;
        assert_eq!(
            Packet::cmp(&Packet::Integer(7), &Packet::Integer(9)),
            Ordering::Less,
        );
        assert_eq!(
            Packet::cmp(
                &"[9]".parse::<Packet>().unwrap(),
                &"[[8, 7 ,6]]".parse::<Packet>().unwrap()
            ),
            Ordering::Greater,
        );
        assert_eq!(
            Packet::cmp(
                &"[[4,4],4,4]".parse::<Packet>().unwrap(),
                &"[[4,4],4,4,4]".parse::<Packet>().unwrap()
            ),
            Ordering::Less,
        );
        assert_eq!(
            Packet::cmp(
                &"[[1],[2,3,4]]".parse::<Packet>().unwrap(),
                &"[[1], 4]".parse::<Packet>().unwrap()
            ),
            Ordering::Less,
        );
        assert_eq!(
            Packet::cmp(
                &"[7, 7, 7, 7]".parse::<Packet>().unwrap(),
                &"[7, 7, 7]".parse::<Packet>().unwrap()
            ),
            Ordering::Greater,
        );
        assert_eq!(
            Packet::cmp(
                &"[7]".parse::<Packet>().unwrap(),
                &"[7]".parse::<Packet>().unwrap()
            ),
            Ordering::Equal,
        );
        assert_eq!(
            Packet::cmp(
                &"[[[]]]".parse::<Packet>().unwrap(),
                &"[]".parse::<Packet>().unwrap()
            ),
            Ordering::Greater,
        );
        assert_eq!(
            Packet::cmp(
                &"[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>().unwrap(),
                &"[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>().unwrap()
            ),
            Ordering::Greater,
        );
        assert_eq!(
            Packet::cmp(
                &"[0, 0, 0]".parse::<Packet>().unwrap(),
                &"[2]".parse::<Packet>().unwrap()
            ),
            Ordering::Less,
        );
    }

    #[test]
    fn test_example() {
        use crate::{part1, part2, Packet};
        let input = "[1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]";
        let pairs: Vec<Packet> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim().parse().unwrap())
            .collect::<Vec<Packet>>();
        assert_eq!(13, part1(&pairs).unwrap());
        assert_eq!(140, part2(&pairs).unwrap());
    }
}
