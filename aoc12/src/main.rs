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
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    part1(&grid)?;
    part2(&grid)?;
    part2_with_reverse_bfs(&grid)?;
    Ok(())
}

fn part1(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();
    // bfs
    let (x, y) = find_char(grid, 'S').unwrap();
    let result = bfs(grid, x, y)?;
    writeln!(io::stdout(), "Part1: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();
    // bfs
    let mut possible = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' || grid[i][j] == 'a' {
                possible.push((i, j));
            }
        }
    }
    let mut result = usize::MAX;
    for (x, y) in possible {
        result = result.min(bfs(grid, x, y).ok().unwrap_or(usize::MAX));
    }
    writeln!(io::stdout(), "Part2: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2_with_reverse_bfs(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();
    let (x, y) = find_char(grid, 'E').unwrap();
    let result = reverse_bfs(grid, x, y)?;
    writeln!(io::stdout(), "Part2 with reverse bfs: {result}",)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn bfs(grid: &[Vec<char>], x: usize, y: usize) -> Result<usize> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![vec![false; width]; height];

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    let mut depth = 0;
    while !queue.is_empty() {
        let count = queue.len();
        for _ in 0..count {
            let (i, j) = queue.pop_front().unwrap();
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            let c = grid[i][j];
            if is_dest(c) {
                return Ok(depth);
            }
            if i > 0 && reachable(c, grid[i - 1][j]) {
                queue.push_back((i - 1, j))
            }
            if i + 1 < height && reachable(c, grid[i + 1][j]) {
                queue.push_back((i + 1, j))
            }
            if j > 0 && reachable(c, grid[i][j - 1]) {
                queue.push_back((i, j - 1))
            }
            if j + 1 < width && reachable(c, grid[i][j + 1]) {
                queue.push_back((i, j + 1))
            }
        }
        depth += 1;
    }
    err!("There is not valid router found")
}

fn reverse_bfs(grid: &[Vec<char>], x: usize, y: usize) -> Result<usize> {
    // from E to S
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![vec![false; width]; height];

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    let mut depth = 0;
    while !queue.is_empty() {
        let count = queue.len();
        for _ in 0..count {
            let (i, j) = queue.pop_front().unwrap();
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            let c = grid[i][j];
            if c == 'S' || c == 'a' {
                return Ok(depth);
            }
            if i > 0 && reachable(grid[i - 1][j], c) {
                queue.push_back((i - 1, j))
            }
            if i + 1 < height && reachable(grid[i + 1][j], c) {
                queue.push_back((i + 1, j))
            }
            if j > 0 && reachable(grid[i][j - 1], c) {
                queue.push_back((i, j - 1))
            }
            if j + 1 < width && reachable(grid[i][j + 1], c) {
                queue.push_back((i, j + 1))
            }
        }
        depth += 1;
    }
    err!("There is not valid router found")
}

fn find_char(grid: &[Vec<char>], dest: char) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(i, row)| {
        row.iter()
            .enumerate()
            .find_map(|(j, &c)| if c == dest { Some((i, j)) } else { None })
    })
}

fn is_dest(c: char) -> bool {
    c == 'E'
}

fn reachable(c: char, n: char) -> bool {
    if c == 'S' {
        n == 'a' || n == 'b'
    } else if n == 'E' {
        c == 'z' || c == 'y'
    } else if n == 'S' {
        false
    } else {
        (n as u8).saturating_sub(c as u8) <= 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reachable() {
        use crate::reachable;
        assert_eq!(reachable('a', 'b'), true);
        assert_eq!(reachable('a', 'd'), false);
        assert_eq!(reachable('f', 'a'), true);
        assert_eq!(reachable('v', 'E'), false);
    }

    #[test]
    fn test_part1() {
        use crate::part1;
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let r = part1(&grid).unwrap();
        assert_eq!(r, 31);
    }

    #[test]
    fn test_part2() {
        use crate::part2;
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let r = part2(&grid).unwrap();
        assert_eq!(r, 29);
    }

    #[test]
    fn test_part2_reverse_bfs() {
        use crate::part2_with_reverse_bfs;
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let r = part2_with_reverse_bfs(&grid).unwrap();
        assert_eq!(r, 29);
    }
}
