use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    with_tree(&input)?;
    with_stack(&input)?;
    Ok(())
}

fn with_stack(input: &str) -> Result<()> {
    let mut sizes = HashMap::new();
    let mut sub_dirs = HashMap::new();
    sizes.insert("/".to_string(), 0);
    let mut pwd = vec![];
    for line in input.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let dir_name = line.split(" ").last().unwrap();
                if dir_name == ".." {
                    pwd.pop();
                } else if dir_name == "." {
                    unimplemented!("unimplemented for path .")
                } else {
                    pwd.push(dir_name);
                }
            } else if line.starts_with("$ ls") {
                continue;
            } else {
                return err!("command not found: {:?}", line);
            }
        } else {
            let path = pwd.join("/");
            if line.starts_with("dir") {
                sub_dirs
                    .entry(path)
                    .or_insert(vec![])
                    .push(line.split_once(" ").unwrap().1);
            } else {
                *sizes.entry(path).or_insert(0) +=
                    line.split_once(" ").unwrap().0.parse::<usize>().unwrap();
            }
        }
    }
    let total_size = compute_dir_size(&sub_dirs, &mut sizes, "/");

    // Part 1
    let result: usize = sizes.values().filter(|&&s| s <= 100000).sum();
    writeln!(
        io::stdout(),
        "What is the sum of the total sizes of those directories? {result}",
    )?;

    // Part 2
    let unused = 70000000 - total_size;
    let result: usize = *sizes
        .values()
        .filter(|&&s| unused + s >= 30000000)
        .min()
        .unwrap();
    writeln!(
        io::stdout(),
        "What is the total size of that directory? {result}",
    )?;
    Ok(())
}

fn compute_dir_size(
    sub_dirs: &HashMap<String, Vec<&str>>,
    sizes: &mut HashMap<String, usize>,
    path: &str,
) -> usize {
    if let Some(dirs) = sub_dirs.get(path) {
        *sizes.entry(path.to_string()).or_insert(0) += dirs
            .iter()
            .map(|dir| compute_dir_size(sub_dirs, sizes, &format!("{}/{}", path, dir)))
            .sum::<usize>()
    }
    *sizes.get(path).unwrap()
}

fn with_tree(input: &str) -> Result<()> {
    let mut dirs = Dirs::new();
    let mut cur_dir_index = 0;
    for line in input.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let next_dir_name = line.split(" ").last().unwrap();
                if next_dir_name == "/" {
                    cur_dir_index = 0;
                } else if next_dir_name == ".." {
                    let p = dirs.dirs[cur_dir_index].parent;
                    cur_dir_index = p;
                } else if next_dir_name == "." {
                    unimplemented!("unimplemented for path .")
                } else {
                    let cur_dir = &dirs.dirs[cur_dir_index];
                    if cur_dir.has_dir(next_dir_name) {
                        cur_dir_index = *cur_dir.table.get(next_dir_name).unwrap();
                    } else {
                        return err!("no such file or directory: {}", next_dir_name);
                    }
                }
            } else if line.starts_with("$ ls") {
                continue;
            } else {
                return err!("command not found: {:?}", line);
            }
        } else {
            let id = dirs.dirs[cur_dir_index].id;
            if line.starts_with("dir") {
                if let Some(name) = line.split(" ").last() {
                    dirs.add_dir(id, name.to_string());
                } else {
                    return err!("not a vaild ls out put for sub dir: {:?}", line);
                }
            } else {
                if let Some((size, name)) = line.split_once(" ") {
                    dirs.add_file(id, name.to_string(), size.parse().unwrap());
                } else {
                    return err!("not a vaild ls out put for file: {:?}", line);
                }
            }
        }
    }
    part1(&dirs, 100000)?;
    part2(&dirs)?;
    Ok(())
}

fn part1(dirs: &Dirs, threshold: usize) -> Result<()> {
    let result: usize = dirs
        .dirs
        .iter()
        .map(|d| dirs.get_size(d.id))
        .filter(|&s| s <= threshold)
        .sum();
    writeln!(
        io::stdout(),
        "What is the sum of the total sizes of those directories? {result}",
    )?;
    Ok(())
}

fn part2(dirs: &Dirs) -> Result<()> {
    let unused = 70000000 - dirs.get_size(0);
    let result: usize = dirs
        .dirs
        .iter()
        .map(|d| dirs.get_size(d.id))
        .filter(|&s| unused + s >= 30000000)
        .min()
        .unwrap();
    writeln!(
        io::stdout(),
        "What is the total size of that directory? {result}",
    )?;
    Ok(())
}

#[derive(Debug)]
struct Dirs {
    dirs: Vec<Dir>,
    next_index: usize,
}

impl Dirs {
    fn new() -> Self {
        Dirs {
            dirs: vec![Dir::new(0, 0)],
            next_index: 1,
        }
    }

    fn get_size(&self, id: usize) -> usize {
        let files_size: usize = self.dirs[id].files.iter().map(|(_, f)| f).sum();
        let sub_dirs_size: usize = self.dirs[id]
            .sub_dir
            .iter()
            .map(|&d| self.get_size(d))
            .sum();
        files_size + sub_dirs_size
    }

    fn add_dir(&mut self, id: usize, name: String) {
        let dir = &mut self.dirs[id];
        if !dir.table.contains_key(&name) {
            dir.table.insert(name.to_string(), self.next_index);
            dir.sub_dir.push(self.next_index);
            self.dirs.push(Dir::new(self.next_index, id));
            self.next_index += 1;
        }
    }

    fn add_file(&mut self, id: usize, name: String, size: usize) {
        let dir = &mut self.dirs[id];
        if !dir.files.contains_key(&name) {
            dir.files.insert(name, size);
        }
    }
}

#[derive(Debug)]
struct Dir {
    id: usize,
    sub_dir: Vec<usize>,
    table: HashMap<String, usize>,
    files: HashMap<String, usize>,
    parent: usize,
}

impl Dir {
    fn has_dir(&self, name: &str) -> bool {
        self.table.contains_key(name)
    }

    fn new(id: usize, parent: usize) -> Self {
        Dir {
            id,
            sub_dir: Vec::new(),
            table: HashMap::new(),
            files: HashMap::new(),
            parent,
        }
    }
}
