use std::{
    collections::{HashMap, HashSet},
    ops::MulAssign,
    path::Path,
};

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn load(path: &Path) -> Grid {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut rows = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        rows.push(row);
    }
    Grid {
        rows: rows.len() as usize,
        cols: rows[0].len() as usize,
        data: rows,
    }
}

#[allow(dead_code)]
pub fn star_1() -> usize {
    let grid = load(Path::new("src/dec08/testdata.txt"));
    let mut visible = HashSet::new();
    for direction in vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut height = 0;
        for n in 0..grid.rows * grid.cols {
            let row = match direction {
                Direction::Left | Direction::Right => n / grid.cols,
                Direction::Up => grid.rows - 1 - n % grid.rows,
                Direction::Down => n % grid.rows,
            };
            let col = match direction {
                Direction::Left => n % grid.cols,
                Direction::Right => grid.cols - 1 - n % grid.cols,
                Direction::Up | Direction::Down => n / grid.rows,
            };
            let reset = match direction {
                Direction::Left => col == 0,
                Direction::Right => col == grid.cols - 1,
                Direction::Up => row == grid.rows - 1,
                Direction::Down => row == 0,
            };
            let tree = grid.data[row][col];
            if tree > height || reset {
                visible.insert((row, col));
                height = tree;
            }
        }
    }
    visible.len()
}

#[allow(dead_code)]
pub fn star_2() -> u64 {
    let grid = load(Path::new("src/dec08/testdata.txt"));
    let mut scores: HashMap<(usize, usize), u64> = HashMap::new();
    for row in 1..grid.rows - 1 {
        for col in 1..grid.cols - 1 {
            for direction in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let (mut x, mut y) = (row, col);
                let height = grid.data[row][col];
                let mut score = 0;
                loop {
                    let reached_edge = match direction {
                        Direction::Up => x == 0,
                        Direction::Down => x == grid.rows - 1,
                        Direction::Left => y == grid.cols - 1,
                        Direction::Right => y == 0,
                    };
                    if reached_edge {
                        break;
                    }
                    match direction {
                        Direction::Up => x -= 1,
                        Direction::Down => x += 1,
                        Direction::Left => y += 1,
                        Direction::Right => y -= 1,
                    }
                    score += 1;
                    let tree = grid.data[x][y];
                    if tree >= height {
                        break;
                    }
                }
                scores.entry((row, col)).or_insert(1).mul_assign(score);
            }
        }
    }
    *scores.values().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 1546);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 519064);
    }
}
