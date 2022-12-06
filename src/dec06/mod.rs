use std::{collections::HashSet, fs, path::Path};

fn solve(path: &Path, msg_len: usize) -> Vec<usize> {
    let input = fs::read_to_string(path).expect("file not found");
    let mut markers = Vec::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        for (idx, window) in chars.windows(msg_len).enumerate() {
            let set: HashSet<&char> = HashSet::from_iter(window);
            if set.len() == msg_len {
                markers.push(idx + msg_len);
                break;
            }
        }
    }
    markers
}

#[allow(dead_code)]
pub fn star_1() -> Vec<usize> {
    solve(Path::new("src/dec06/testdata.txt"), 4)
}

#[allow(dead_code)]
pub fn star_2() -> Vec<usize> {
    solve(Path::new("src/dec06/testdata.txt"), 14)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), vec![1275]);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), vec![3605]);
    }
}
