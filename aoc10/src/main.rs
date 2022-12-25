use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let cpu = Cpu::new(
        input
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>>>()?,
    );

    part1(cpu.clone())?;
    part2(cpu)?;
    Ok(())
}

fn part1(mut cpu: Cpu) -> Result<()> {
    let mut strengths = 0;
    for c in 1..300 {
        // during cycles start with 1
        // after cycles start with 0
        if [20, 60, 100, 140, 180, 220].contains(&c) {
            strengths += c * cpu.register;
        }
        cpu.cycle();
    }
    writeln!(
        io::stdout(),
        "Part1: What is the sum of these six signal strengths? {strengths}",
    )?;
    Ok(())
}

fn part2(mut cpu: Cpu) -> Result<()> {
    let mut crt = Crt::new();
    for _ in 0..300 {
        // draw a single pixel during each cycle
        // need draw before cycle
        crt.draw(cpu.register);
        cpu.cycle();
    }
    crt.draw(cpu.register);
    writeln!(io::stdout(), "Part2: \n{}", crt.show())?;
    Ok(())
}

struct Crt {
    screen: [[bool; 40]; 6],
    cur_row: usize,
    cur_pos: usize,
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: [[false; 40]; 6],
            cur_row: 0,
            cur_pos: 0,
        }
    }

    fn draw(&mut self, register: i32) {
        if self.cur_row == 6 {
            // all pixels are drawed
            return;
        }
        if (register - self.cur_pos as i32).abs() < 2 {
            // check current position is inside the sprite range
            self.screen[self.cur_row][self.cur_pos] = true;
        }
        self.cur_pos += 1;
        if self.cur_pos == 40 {
            // change row and reset positon
            self.cur_pos = 0;
            self.cur_row += 1;
        }
    }

    fn show(&self) -> String {
        let mut s = String::new();
        for i in 0..6 {
            for j in 0..40 {
                if self.screen[i][j] {
                    s.push('#')
                } else {
                    s.push('.')
                }
            }
            s.push('\n')
        }
        s
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    register: i32,
    program: Vec<Instruction>,
    pc: usize,   // program counter
    cycle: bool, // is current instruction still need one more cycle to run
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Cpu {
            register: 1,
            program: instructions,
            pc: 0,
            cycle: false, // default fasle
        }
    }

    fn cycle(&mut self) {
        let instr = &self.program[self.pc % self.program.len()];
        match instr {
            Instruction::Addx(n) => {
                if self.cycle {
                    // if current instruction is addx and cycle is ture,
                    // after this cycle the instruction will be finished
                    self.register += n;
                    self.cycle = false;
                    self.pc += 1;
                } else {
                    // if current instruction is addx and cycle is false,
                    // this instruction need one more cycle to run,
                    // set cycle to true, don't increase pc
                    self.cycle = true;
                }
            }
            Instruction::Noop => self.pc += 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s.trim() == "noop" {
            return Ok(Instruction::Noop);
        }
        if let Some((instr, op)) = s.split_once(' ') {
            if instr.trim() == "addx" {
                let op: i32 = op.parse()?;
                return Ok(Instruction::Addx(op));
            }
        }
        err!("This is not a valid instruction: {}", s)
    }
}
