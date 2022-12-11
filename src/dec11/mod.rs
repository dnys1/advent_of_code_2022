use std::{collections::VecDeque, path::Path};

use derive_builder::Builder;
use regex::Regex;

#[derive(Debug, Clone, Builder)]
struct Monkey {
    items: VecDeque<u64>,
    items_inspected: u64,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
struct Operation {
    op: String,
    rhs: String,
}

#[derive(Debug, Clone)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

enum Reduction {
    Divide(u64),
    Modulo(u64),
}

impl Monkey {
    fn round(&mut self, reduction: Reduction) -> Option<(u64, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.items_inspected += 1;
            let rhs = || self.operation.rhs.parse::<u64>().unwrap();
            let mut result = match self.operation.op.as_str() {
                "+" => {
                    item + match self.operation.rhs.as_str() {
                        "old" => item,
                        _ => rhs(),
                    }
                }
                "-" => {
                    item - match self.operation.rhs.as_str() {
                        "old" => item,
                        _ => rhs(),
                    }
                }
                "*" => {
                    item * match self.operation.rhs.as_str() {
                        "old" => item,
                        _ => rhs(),
                    }
                }
                "/" => {
                    item / match self.operation.rhs.as_str() {
                        "old" => item,
                        _ => rhs(),
                    }
                }
                _ => panic!("invalid operation"),
            };
            match reduction {
                Reduction::Divide(divisor) => result /= divisor,
                Reduction::Modulo(divisor) => result %= divisor,
            }
            if result % self.test.divisible_by == 0 {
                Some((result, self.test.if_true))
            } else {
                Some((result, self.test.if_false))
            }
        } else {
            None
        }
    }
}

fn load(path: &Path) -> Vec<Monkey> {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut monkeys = Vec::new();
    let digit = Regex::new(r"\d+").unwrap();
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut monkey = MonkeyBuilder::default();
    let mut line_no = 0;
    while line_no < lines.len() {
        let line = lines[line_no];
        let mut items = VecDeque::new();
        let words = line.trim().split(' ').collect::<Vec<&str>>();
        match words[0] {
            "" => {
                monkeys.push(monkey.build().unwrap());
            }
            "Monkey" => {
                monkey = MonkeyBuilder::default();
                monkey.items_inspected(0);
            }
            "Starting" => {
                for item in digit.find_iter(line) {
                    items.push_back(item.as_str().parse::<u64>().unwrap());
                }
                monkey.items(items);
            }
            "Operation:" => {
                let op = words[4];
                let rhs = words[5];
                monkey.operation(Operation {
                    op: op.to_string(),
                    rhs: rhs.to_string(),
                });
            }
            "Test:" => {
                let divisible_by = words[3].parse::<u64>().unwrap();
                let if_true = lines[line_no + 1].trim().split(' ').collect::<Vec<&str>>()[5]
                    .parse::<usize>()
                    .unwrap();
                let if_false = lines[line_no + 2].trim().split(' ').collect::<Vec<&str>>()[5]
                    .parse::<usize>()
                    .unwrap();
                monkey.test(Test {
                    divisible_by,
                    if_true,
                    if_false,
                });
                line_no += 2;
            }
            _ => panic!("invalid line"),
        }
        line_no += 1;
    }
    monkeys.push(monkey.build().unwrap());
    monkeys
}

#[allow(dead_code)]
pub fn star_1() -> u64 {
    let mut monkeys = load(Path::new("src/dec11/testdata.txt"));
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let monkey = monkeys.get_mut(m).unwrap();
            let mut results = Vec::new();
            while let Some(result) = monkey.round(Reduction::Divide(3)) {
                results.push(result);
            }
            for (result, m) in results {
                monkeys[m].items.push_back(result);
            }
        }
    }
    let mut max_monkeys = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<u64>>();
    max_monkeys.sort();
    max_monkeys[monkeys.len() - 2..].iter().product()
}

#[allow(dead_code)]
pub fn star_2() -> u64 {
    let mut monkeys = load(Path::new("src/dec11/testdata.txt"));
    let lcm = monkeys.iter().map(|m| m.test.divisible_by).product();
    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            let monkey = monkeys.get_mut(m).unwrap();
            let mut results = Vec::new();
            while let Some(result) = monkey.round(Reduction::Modulo(lcm)) {
                results.push(result);
            }
            for (result, m) in results {
                monkeys[m].items.push_back(result);
            }
        }
    }
    let mut max_monkeys = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<u64>>();
    max_monkeys.sort();
    max_monkeys[monkeys.len() - 2..].iter().product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 66124);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 19309892877);
    }
}
