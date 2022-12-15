use std::{fmt::Display, path::Path};

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    min: (usize, usize),
    max: (usize, usize),
    has_floor: bool,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            min: (500, 0),
            max: (500, 0),
            has_floor: false,
        }
    }

    fn update_bounds(&mut self, (x, y): (usize, usize)) {
        if x < self.min.0 {
            self.min.0 = x;
        }
        if x > self.max.0 {
            self.max.0 = x;
        }
        if y < self.min.1 {
            self.min.1 = y;
        }
        if y > self.max.1 {
            self.max.1 = y;
        }
    }

    fn insert(&mut self, x: usize, y: usize, data: char) -> bool {
        if self.data.len() <= y {
            self.data.resize_with(y + 1, Vec::new);
        }
        let row = self.data.get_mut(y).unwrap();
        if row.len() <= x {
            row.resize(x + 1, '.');
        }
        match row.get(x) {
            Some(n) if *n == data => false,
            _ => {
                row[x] = data;
                true
            }
        }
    }

    fn add_sand_grain(&mut self) -> Option<(usize, usize)> {
        let mut x = 500;
        let mut y = 0;
        loop {
            if !self.has_floor {
                if x < self.min.0 || x > self.max.0 {
                    return None;
                }
                if y > self.max.1 {
                    return None;
                }
            }
            let mut below = self.data.get(y + 1).and_then(|row| row.get(x));
            if self.has_floor && y > self.max.1 {
                below = below.or(Some(&'#'));
            }
            match below {
                None | Some('.') => {
                    y += 1;
                }
                Some('o' | '#') => {
                    let mut left = self.data.get(y + 1).and_then(|row| row.get(x - 1));
                    let mut right = self.data.get(y + 1).and_then(|row| row.get(x + 1));
                    if self.has_floor && y > self.max.1 {
                        left = left.or(Some(&'#'));
                        right = right.or(Some(&'#'));
                    }
                    match (left, right) {
                        (Some('o' | '#'), Some('o' | '#')) => {
                            return Some((x, y));
                        }
                        (Some('.') | None, _) => {
                            x -= 1;
                            y += 1;
                        }
                        (_, Some('.') | None) => {
                            x += 1;
                            y += 1;
                        }
                        _ => panic!("unexpected value"),
                    }
                }
                _ => panic!("unexpected value"),
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.data.iter().enumerate() {
            if y < self.min.1 || y > self.max.1 {
                continue;
            }
            for (x, cell) in row.iter().enumerate() {
                if x < self.min.0 || x > self.max.0 {
                    continue;
                }
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "min: {:?}", self.min)?;
        writeln!(f, "max: {:?}", self.max)?;
        Ok(())
    }
}

fn load(path: &Path) -> Grid {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut grid = Grid::new();
    for line in input.lines() {
        let path = line.split(' ').collect::<Vec<&str>>();
        let mut starting_point = None;
        for (_, segment) in path.iter().enumerate().filter(|(idx, _)| idx % 2 == 0) {
            let point = segment.split(',').collect::<Vec<&str>>();
            let x = point[0].parse::<usize>().unwrap();
            let y = point[1].parse::<usize>().unwrap();
            match starting_point {
                None => {}
                Some((prev_x, prev_y)) => {
                    if x == prev_x {
                        for y in (prev_y..=y).chain(y..=prev_y) {
                            grid.insert(x, y, '#');
                        }
                    } else {
                        for x in (prev_x..=x).chain(x..=prev_x) {
                            grid.insert(x, y, '#');
                        }
                    }
                }
            }
            grid.update_bounds((x, y));
            starting_point = Some((x, y));
        }
    }
    grid
}

#[allow(dead_code)]
pub fn star_1() -> usize {
    let mut grid = load(Path::new("src/dec14/testdata.txt"));
    println!("{}", grid);
    let mut num_grains = 0;
    while let Some((x, y)) = grid.add_sand_grain() {
        if !grid.insert(x, y, 'o') {
            break;
        }
        num_grains += 1;
    }
    num_grains
}

#[allow(dead_code)]
pub fn star_2() -> usize {
    let mut grid = load(Path::new("src/dec14/testdata.txt"));
    grid.has_floor = true;
    let mut num_grains = 0;
    while let Some((x, y)) = grid.add_sand_grain() {
        if !grid.insert(x, y, 'o') {
            break;
        }
        num_grains += 1;
    }
    num_grains
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 614);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 93);
    }
}
