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
        let mut row = match direction {
            Direction::Left | Direction::Down | Direction::Right => 0,
            Direction::Up => grid.rows - 1,
        };
        let mut col = match direction {
            Direction::Left | Direction::Up | Direction::Down => 0,
            Direction::Right => grid.cols - 1,
        };
        let mut height = 0;
        for _ in 0..grid.rows * grid.cols {
            let tree = grid.data[row][col];
            if tree > height || row == 0 || col == 0 || row == grid.rows - 1 || col == grid.cols - 1
            {
                visible.insert((row, col));
            }
            height = tree.max(height);
            match direction {
                Direction::Up => {
                    if row == 0 {
                        row = grid.rows - 1;
                        col += 1;
                        height = 0;
                    } else {
                        row -= 1;
                    }
                }
                Direction::Down => {
                    if row == grid.rows - 1 {
                        row = 0;
                        col += 1;
                        height = 0;
                    } else {
                        row += 1;
                    }
                }
                Direction::Left => {
                    if col == grid.cols - 1 {
                        row += 1;
                        col = 0;
                        height = 0;
                    } else {
                        col += 1;
                    }
                }
                Direction::Right => {
                    if col == 0 {
                        row += 1;
                        col = grid.cols - 1;
                        height = 0;
                    } else {
                        col -= 1;
                    }
                }
            };
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
                match direction {
                    Direction::Up => {
                        while x > 0 {
                            x -= 1;
                            score += 1;
                            let tree = grid.data[x][y];
                            if tree >= height {
                                break;
                            }
                        }
                    }
                    Direction::Down => {
                        while x < grid.rows - 1 {
                            x += 1;
                            score += 1;
                            let tree = grid.data[x][y];
                            if tree >= height {
                                break;
                            }
                        }
                    }
                    Direction::Left => {
                        while y < grid.cols - 1 {
                            y += 1;
                            score += 1;
                            let tree = grid.data[x][y];
                            if tree >= height {
                                break;
                            }
                        }
                    }
                    Direction::Right => {
                        while y > 0 {
                            y -= 1;
                            score += 1;
                            let tree = grid.data[x][y];
                            if tree >= height {
                                break;
                            }
                        }
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
