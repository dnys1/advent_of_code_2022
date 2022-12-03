use std::{path::Path, fs};

#[derive(Debug)]
pub struct Elf {
    pub inventory: Vec<u64>,
}

impl Elf {
    pub fn total(&self) -> u64 {
        self.inventory.iter().sum()
    }
}

impl Eq for Elf {}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.inventory == other.inventory
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let total = self.total();
        let other_total = other.total();
        total.cmp(&other_total)
    }
}

pub fn load(path: &Path) -> Vec<Elf> {
    let input = fs::read_to_string(path).expect("file not found");
    let mut current_elf: Vec<u64> = vec![];
    let mut elves: Vec<Elf> = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            elves.push(Elf { inventory: current_elf });
            current_elf = vec![];
        } else {
            current_elf.push(line.trim().parse().unwrap());
        }
    }
    elves
}

fn max(elves: &[Elf]) -> (usize, &Elf) {
    let mut max_idx = 0;
    let mut max: Option<&Elf> = None;
    for (idx, elf) in elves.iter().enumerate() {
        if let Some(m) = max {
            if elf > m {
                max_idx = idx;
                max = Some(elf);
            }
        } else if let None = max {
            max_idx = idx;
            max = Some(elf);
        }
    }
   (max_idx,  max.unwrap())
}

#[allow(dead_code)]
pub fn star_1() -> u64 {
    let elves = load(Path::new("src/dec01/testdata.txt"));
    max(&elves).1.total()
}

#[allow(dead_code)]
pub fn star_2() -> u64 {
    let mut elves = load(Path::new("src/dec01/testdata.txt"));
    let mut sum = 0;
    for _ in 0..3 {
        let (idx, elf) = max(&elves);
        sum += elf.total();
        elves.remove(idx);
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]  
    fn test_star_1() {
        assert_eq!(star_1(), 74394);
    }

    #[test]  
    fn test_star_2() {
        assert_eq!(star_2(), 212836);
    }
}
