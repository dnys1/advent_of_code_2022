use std::{ops::RangeInclusive, path::Path};

struct Pair(RangeInclusive<u16>, RangeInclusive<u16>);

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        fn parse_range(s: &str) -> RangeInclusive<u16> {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            start..=end
        }
        let mut parts = s.split(',');
        let elf_1 = parse_range(parts.next().unwrap());
        let elf_2 = parse_range(parts.next().unwrap());
        Pair(elf_1, elf_2)
    }
}

fn load(path: &Path) -> Vec<Pair> {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut pairs: Vec<Pair> = vec![];
    for line in input.lines() {
        pairs.push(Pair::from(line));
    }
    pairs
}

#[allow(dead_code)]
pub fn star_1() -> u16 {
    let pairs = load(Path::new("src/dec04/testdata.txt"));
    let mut num_pairs = 0;
    for pair in pairs {
        let elf_1 = pair.0;
        let elf_2 = pair.1;
        if elf_1.start() <= elf_2.start() && elf_1.end() >= elf_2.end() ||
            elf_2.start() <= elf_1.start() && elf_2.end() >= elf_1.end() {
            num_pairs += 1;
        }
    }
    num_pairs
}

#[allow(dead_code)]
pub fn star_2() -> u16 {
    let pairs = load(Path::new("src/dec04/testdata.txt"));
    let mut num_pairs = 0;
    for pair in pairs {
        let elf_1 = pair.0;
        let elf_2 = pair.1;
        if elf_1.start() <= elf_2.end() && elf_1.end() >= elf_2.start() ||
            elf_2.start() <= elf_1.end() && elf_2.end() >= elf_1.start() {
            num_pairs += 1;
        }
    }
    num_pairs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 305);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 811);
    }
}
