use std::{
    collections::{HashSet, VecDeque},
    path::Path,
};

use derive_builder::Builder;

#[derive(Debug, Builder)]
struct Map {
    elevations: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    starting_location: (usize, usize),
    target_location: (usize, usize),
}

impl Map {
    fn shortest_path(&self, starting_location: (usize, usize)) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back((starting_location, 0));
        let mut visited = HashSet::new();
        visited.insert(starting_location);
        while let Some((location, distance)) = queue.pop_front() {
            let (x, y) = location;
            if location == self.target_location {
                return Some(distance);
            }
            let mut neighbors = vec![(x + 1, y), (x, y + 1)];
            if x > 0 {
                neighbors.push((x - 1, y));
            }
            if y > 0 {
                neighbors.push((x, y - 1));
            }
            for neighbor in neighbors {
                let (x, y) = neighbor;
                if x >= self.height || y >= self.width {
                    continue;
                }
                if visited.contains(&neighbor) {
                    continue;
                }
                let elevation = self.elevations[x][y];
                if elevation > self.elevations[location.0][location.1] + 1 {
                    continue;
                }
                visited.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
        None
    }
}

fn load(path: &Path) -> Map {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut builder = MapBuilder::default();
    let mut elevations = Vec::new();
    for line in input.lines() {
        elevations.push(Vec::new());
        for (idx, char) in line.chars().enumerate() {
            let elevation = match char {
                'a'..='z' => char as usize - 'a' as usize,
                'S' => 0,
                'E' => 25,
                _ => panic!("invalid character, {}", char),
            };
            elevations.last_mut().unwrap().push(elevation);
            match char {
                'S' => {
                    builder.starting_location((elevations.len() - 1, idx));
                }
                'E' => {
                    builder.target_location((elevations.len() - 1, idx));
                }
                _ => {}
            }
        }
    }
    builder.height(elevations.len());
    builder.width(elevations[0].len());
    builder.elevations(elevations);
    builder.build().unwrap()
}

#[allow(dead_code)]
pub fn star_1() -> usize {
    let map = load(Path::new("src/dec12/testdata.txt"));
    map.shortest_path(map.starting_location).unwrap()
}

#[allow(dead_code)]
pub fn star_2() -> usize {
    let map = load(Path::new("src/dec12/testdata.txt"));
    let mut min_path = map.shortest_path(map.starting_location).unwrap();
    for (y, row) in map.elevations.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point == 0 {
                if let Some(path) = map.shortest_path((y, x)) {
                    min_path = min_path.min(path);
                }
            }
        }
    }
    min_path
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 437);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 430);
    }
}
