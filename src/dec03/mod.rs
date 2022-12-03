use std::{collections::HashSet, fs, path::Path};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn get_priority(&self) -> u8 {
        match self.0 {
            'a'..='z' => (self.0 as u8) - ('a' as u8) + 1,
            'A'..='Z' => (self.0 as u8) - ('A' as u8) + 27,
            _ => {
                panic!("unexpected item: {}", self.0);
            }
        }
    }
}

struct Backpack(HashSet<Item>, HashSet<Item>);

impl Backpack {
    fn shared_item(&self) -> &Item {
        let intersection: Vec<&Item> = self.0.intersection(&self.1).collect();
        assert_eq!(
            intersection.len(),
            1,
            "no items shared between compartments"
        );
        intersection.last().unwrap()
    }
}

fn load(path: &Path) -> Vec<Backpack> {
    let input = fs::read_to_string(path).expect("file not found");
    let mut backpacks = Vec::new();
    for line in input.lines() {
        let (left, right) = (&line[0..line.len() / 2], &line[line.len() / 2..]);
        let left_cpt = left.chars().fold(HashSet::new(), |mut accum, item| {
            accum.insert(Item(item));
            accum
        });
        let right_cpt = right.chars().fold(HashSet::new(), |mut accum, item| {
            accum.insert(Item(item));
            accum
        });
        backpacks.push(Backpack(left_cpt, right_cpt));
    }
    backpacks
}

#[allow(dead_code)]
pub fn star_1() -> u32 {
    let backpacks = load(Path::new("src/dec03/testdata.txt"));
    let mut sum: u32 = 0;
    for backpack in backpacks {
        let shared_item = backpack.shared_item();
        sum += shared_item.get_priority() as u32;
    }
    sum
}

#[allow(dead_code)]
pub fn star_2() -> u32 {
    let backpacks = load(Path::new("src/dec03/testdata.txt"));
    backpacks.chunks_exact(3).fold(0, |mut accum, group| {
        if let [elf_1, elf_2, elf_3] = group {
            let mut all_items = HashSet::new();
            all_items.extend(('a'..='z').map(|c| Item(c)));
            all_items.extend(('A'..='Z').map(|c| Item(c)));
            all_items = all_items
                .intersection(&elf_1.0.union(&elf_1.1).map(|item| *item).collect())
                .map(|item| *item)
                .collect();
            all_items = all_items
                .intersection(&elf_2.0.union(&elf_2.1).map(|item| *item).collect())
                .map(|item| *item)
                .collect();
            all_items = all_items
                .intersection(&elf_3.0.union(&elf_3.1).map(|item| *item).collect())
                .map(|item| *item)
                .collect();
            assert_eq!(
                all_items.len(),
                1,
                "more than one shared item: {:?}",
                all_items
            );
            let shared_item: &Item = all_items.iter().last().unwrap();
            accum += shared_item.get_priority() as u32;
        } else {
            panic!("unexpected group: {}", group.len());
        }
        accum
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 7903);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 2548);
    }
}
