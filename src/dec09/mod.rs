use std::{collections::HashSet, path::Path};

#[derive(Debug, Clone)]
struct Rope {
    head: (isize, isize),
    tail: Option<Box<Rope>>,
}

impl Rope {
    fn move_tail(&mut self) {
        if let Some(tail) = &mut self.tail {
            if self.head.0 == tail.head.0 || self.head.1 == tail.head.1 {
                if self.head.0.abs_diff(tail.head.0) > 1 {
                    tail.head.0 += (self.head.0 - tail.head.0).signum();
                } else if self.head.1.abs_diff(tail.head.1) > 1 {
                    tail.head.1 += (self.head.1 - tail.head.1).signum();
                }
            } else if self.head.0.abs_diff(tail.head.0) > 1 || self.head.1.abs_diff(tail.head.1) > 1
            {
                tail.head.0 += (self.head.0 - tail.head.0).signum();
                tail.head.1 += (self.head.1 - tail.head.1).signum();
            }
            tail.move_tail();
        }
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Motion {
    direction: Direction,
    count: usize,
}

fn load(path: &Path) -> Vec<Motion> {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut motions = Vec::new();
    for line in input.lines() {
        let chars = line.split(' ').collect::<Vec<&str>>();
        motions.push(Motion {
            direction: match chars[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("invalid direction"),
            },
            count: chars[1].parse::<usize>().unwrap(),
        });
    }
    motions
}

#[allow(dead_code)]
pub fn star_1() -> usize {
    let motions = load(Path::new("src/dec09/testdata.txt"));
    let mut visited = HashSet::new();
    let mut rope = Rope {
        head: (0, 0),
        tail: Some(Box::new(Rope {
            head: (0, 0),
            tail: None,
        })),
    };
    for motion in motions {
        for _ in 0..motion.count {
            match motion.direction {
                Direction::Right => rope.head.0 += 1,
                Direction::Left => rope.head.0 -= 1,
                Direction::Up => rope.head.1 += 1,
                Direction::Down => rope.head.1 -= 1,
            }
            rope.move_tail();
            visited.insert(rope.tail.as_ref().map(|t| t.head).unwrap());
        }
    }
    visited.len()
}

#[allow(dead_code)]
pub fn star_2() -> usize {
    let motions = load(Path::new("src/dec09/testdata.txt"));
    let mut visited = HashSet::new();
    let mut rope = Rope {
        head: (0, 0),
        tail: None,
    };
    let mut tail = &mut rope;
    for _ in 0..9 {
        tail.tail = Some(Box::new(Rope {
            head: (0, 0),
            tail: None,
        }));
        tail = tail.tail.as_mut().unwrap();
    }
    for motion in motions {
        for _ in 0..motion.count {
            match motion.direction {
                Direction::Right => rope.head.0 += 1,
                Direction::Left => rope.head.0 -= 1,
                Direction::Up => rope.head.1 += 1,
                Direction::Down => rope.head.1 -= 1,
            }
            rope.move_tail();
            let mut tail: &Rope = rope.tail.as_ref().unwrap();
            while let Some(t) = &tail.tail {
                tail = t;
            }
            visited.insert(tail.head);
        }
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 5930);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 2443);
    }
}
