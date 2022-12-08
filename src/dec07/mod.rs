use std::{
    collections::{HashMap, VecDeque},
    ops::AddAssign,
    path::Path,
};

fn load(path: &Path) -> HashMap<String, u64> {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut dir_map = HashMap::new();
    let mut curr_path: VecDeque<&str> = VecDeque::new();
    let lines = input.lines();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let first_part = parts[0];
        match first_part.chars().next().unwrap() {
            '$' => {
                let operation = parts[1];
                match operation {
                    "cd" => {
                        let dir = parts[2];
                        if dir == ".." {
                            curr_path.pop_back();
                        } else {
                            curr_path.push_back(dir);
                        }
                    }
                    "ls" => {}
                    _ => {}
                }
            }
            '0'..='9' => {
                let size: u64 = first_part.parse().unwrap();
                let mut absolute_dir = String::new();
                for dir in curr_path.iter() {
                    absolute_dir += dir;
                    dir_map
                        .entry(absolute_dir.clone())
                        .or_insert(0)
                        .add_assign(size);
                }
            }
            _ => {}
        }
    }
    dir_map
}

#[allow(dead_code)]
pub fn star_1() -> u64 {
    let dirs = load(Path::new("src/dec07/testdata.txt"));
    dirs.iter().filter(|(_, size)| **size <= 100_000).map(|(_, size)| size).sum()
}

#[allow(dead_code)]
pub fn star_2() -> u64 {
    let dirs = load(Path::new("src/dec07/testdata.txt"));
    const TOTAL_SPACE: u64 = 70_000_000;
    const NEED_SPACE: u64 = 30_000_000;
    let top_level_dir = dirs["/"];
    let unused_space = TOTAL_SPACE - top_level_dir;
    *dirs.values()
        .filter(|size| **size >= NEED_SPACE - unused_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 1770595);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 2195372);
    }
}
