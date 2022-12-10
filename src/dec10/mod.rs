use std::{fmt::Display, path::Path};

struct Cpu {
    register: isize,
    cycle: usize,
    signals: [isize; 6],
    pixel: isize,
    bitmap: [bool; 40 * 6],
}

impl Cpu {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 0,
            signals: [0; 6],
            pixel: 0,
            bitmap: [false; 40 * 6],
        }
    }

    fn increment_cycle(&mut self) {
        if (self.register - 1..=self.register + 1).contains(&self.pixel) {
            self.bitmap[self.cycle % 240] = true;
        }
        self.cycle += 1;
        self.pixel += 1;
        self.pixel %= 40;
        let index = match self.cycle {
            20 => 0,
            60 => 1,
            100 => 2,
            140 => 3,
            180 => 4,
            220 => 5,
            _ => return,
        };
        self.signals[index] = self.register * self.cycle as isize;
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                if self.bitmap[y * 40 + x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Instruction {
    AddX(isize),
    Noop,
}

fn load(path: &Path) -> Vec<Instruction> {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut instructions = Vec::new();
    for line in input.lines() {
        let chars = line.split(' ').collect::<Vec<&str>>();
        instructions.push(match chars[0] {
            "addx" => Instruction::AddX(chars[1].parse::<isize>().unwrap()),
            "noop" => Instruction::Noop,
            _ => panic!("invalid instruction"),
        });
    }
    instructions
}

#[allow(dead_code)]
pub fn star_1() -> isize {
    let instructions = load(Path::new("src/dec10/testdata.txt"));
    let mut cpu = Cpu::new();
    for instruction in instructions {
        match instruction {
            Instruction::AddX(x) => {
                cpu.increment_cycle();
                cpu.increment_cycle();
                cpu.register += x;
            }
            Instruction::Noop => {
                cpu.increment_cycle();
            }
        }
    }
    cpu.signals.iter().sum()
}

#[allow(dead_code)]
pub fn star_2() -> String {
    let instructions = load(Path::new("src/dec10/testdata.txt"));
    let mut cpu = Cpu::new();
    for instruction in instructions {
        match instruction {
            Instruction::AddX(x) => {
                cpu.increment_cycle();
                cpu.increment_cycle();
                cpu.register += x;
            }
            Instruction::Noop => {
                cpu.increment_cycle();
            }
        }
    }
    format!("{}", cpu)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 13140);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(
            star_2(),
            "###..####.#..#.###..###..#....#..#.###..\n\
            #..#.#....#..#.#..#.#..#.#....#..#.#..#.\n\
            #..#.###..####.#..#.#..#.#....#..#.###..\n\
            ###..#....#..#.###..###..#....#..#.#..#.\n\
            #.#..#....#..#.#....#.#..#....#..#.#..#.\n\
            #..#.####.#..#.#....#..#.####..##..###..\n"
        );
    }
}
