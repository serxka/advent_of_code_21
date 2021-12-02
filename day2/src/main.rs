use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Up,
    Down,
}

struct Instruction {
    pub direction: Direction,
    pub distance: isize,
}

fn parse_input<B: BufRead>(buf: B) -> Vec<Instruction> {
    let mut buf = buf.lines();
    let mut instr = Vec::new();
    while let Some(Ok(line)) = buf.next() {
        let mut parts = line.split(' ');
        let dir = match parts.next() {
            Some("forward") => Direction::Forward,
            Some("up") => Direction::Up,
            Some("down") => Direction::Down,
            _ => {
                eprintln!("invalid instruction {:?}", line);
                continue;
            }
        };
        let dis = match parts.last().map(|n| n.parse()) {
            Some(Ok(n)) => n,
            _ => {
                eprintln!("invalid instruction {:?}", line);
                continue;
            }
        };

        instr.push(Instruction {
            direction: dir,
            distance: dis,
        });
    }

    instr
}

struct Submarine {
    pub distance: isize,
    pub depth: isize,
    pub aim: isize,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            distance: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn exec(&mut self, instrs: &[Instruction]) {
        for instr in instrs.iter() {
            match instr.direction {
                Direction::Forward => self.distance += instr.distance,
                Direction::Up => self.depth -= instr.distance,
                Direction::Down => self.depth += instr.distance,
            }
        }
    }

    pub fn exec_aim(&mut self, instrs: &[Instruction]) {
        for instr in instrs.iter() {
            match instr.direction {
                Direction::Forward => {
                    self.distance += instr.distance;
                    self.depth += instr.distance * self.aim;
                }
                Direction::Up => self.aim -= instr.distance,
                Direction::Down => self.aim += instr.distance,
            }
        }
    }

    pub fn output(&self) -> isize {
        self.distance * self.depth
    }
}

fn main() {
    let file = File::open("day2/input").unwrap();
    let d = parse_input(BufReader::new(file));

    let mut sub = Submarine::new();
    sub.exec(&d);
    println!("position: {}", sub.output());

    let mut sub = Submarine::new();
    sub.exec_aim(&d);
    println!("position: {}", sub.output());
}
