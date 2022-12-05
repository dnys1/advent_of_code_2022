use std::{collections::VecDeque, fmt::Write, fs, path::Path};

use regex::Regex;

#[derive(Debug, Clone)]
struct Stack(VecDeque<char>);

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Problem {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

enum CrateMoverVersion {
    CrateMover9000,
    CrateMover9001,
}

impl Problem {
    /// Runs the procedure specified by [moves] according to the rules of [version].
    fn process(&mut self, version: CrateMoverVersion) -> String {
        for m in &self.moves {
            let mut to_move = Vec::new();
            let from = self.stacks.get_mut(m.from).unwrap();
            for _ in 0..m.count {
                to_move.push(from.0.pop_front().unwrap());
            }
            let to = self.stacks.get_mut(m.to).unwrap();
            if let CrateMoverVersion::CrateMover9001 = version {
                to_move.reverse();
            }
            for c in to_move {
                to.0.push_front(c);
            }
        }
        self.stacks.iter().fold(String::new(), |mut accum, stack| {
            accum.write_char(*stack.0.front().unwrap()).unwrap();
            accum
        })
    }
}

fn load(path: &Path) -> Problem {
    let input = fs::read_to_string(path).expect("file not found");
    let mut stacks = Vec::new();
    let crate_matcher = Regex::new(r"(\[\w\]|\s{3})\s?").unwrap();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        for (stack, m) in crate_matcher.captures_iter(line).enumerate() {
            match m.get(0).unwrap().as_str().trim() {
                "" => {}
                _ => {
                    if stacks.len() < stack + 1 {
                        stacks.resize(stack + 1, Stack(VecDeque::new()));
                    }
                    let stack = stacks.get_mut(stack).unwrap();
                    let group = m.get(1).unwrap().as_str();
                    stack
                        .0
                        .push_back(group.chars().nth(1).expect("unexpected format"));
                }
            }
        }
    }

    let move_matcher = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut moves = Vec::new();
    for line in lines {
        for m in move_matcher.captures_iter(line) {
            moves.push(Move {
                count: m[1].parse().unwrap(),
                from: m[2].parse::<usize>().unwrap() - 1,
                to: m[3].parse::<usize>().unwrap() - 1,
            })
        }
    }
    Problem { stacks, moves }
}

#[allow(dead_code)]
pub fn star_1() -> String {
    let mut problem = load(Path::new("src/dec05/testdata.txt"));
    problem.process(CrateMoverVersion::CrateMover9000)
}

#[allow(dead_code)]
pub fn star_2() -> String {
    let mut problem = load(Path::new("src/dec05/testdata.txt"));
    problem.process(CrateMoverVersion::CrateMover9001)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), String::from("TBVFVDZPN"));
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), String::from("VLCWHTDSZ"));
    }
}
