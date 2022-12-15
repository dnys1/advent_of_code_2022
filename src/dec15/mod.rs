use std::{
    collections::HashMap,
    path::Path, ops::RangeInclusive,
};

use range_union_find::{IntRangeUnionFind, OverlapType};
use regex::Regex;

#[derive(Debug)]
struct Grid {
    sensors: Vec<(isize, isize)>,
    beacons: Vec<(isize, isize)>,
    ranges: HashMap<isize, IntRangeUnionFind<isize>>,
}

impl Grid {
    fn new(sensors: Vec<(isize, isize)>, beacons: Vec<(isize, isize)>) -> Self {
        Grid { sensors, beacons, ranges: HashMap::new() }
    }

    fn mark_all(&mut self) -> (isize, isize) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        for i in 0..self.sensors.len() {
            let sensor = self.sensors[i];
            let closest_beacon = self.beacons[i];
            println!("Sensor: {:?}, Beacon: {:?}", sensor, closest_beacon);
            let distance = (sensor.0 - closest_beacon.0).abs() + (sensor.1 - closest_beacon.1).abs();
            if distance == 0 {
                continue;
            }
            let mut dx = distance;
            for dy in 0..=distance {
                self.insert_range(sensor.1 - dy, &(sensor.0 - dx..=sensor.0 + dx));
                self.insert_range(sensor.1 + dy, &(sensor.0 - dx..=sensor.0 + dx));
                while dx + dy >= distance {
                    dx -= 1;
                }
                min_x = min_x.min(sensor.0 - dx);
                max_x = max_x.max(sensor.0 + dx);
            }
        }
        (min_x, max_x)
    }

    fn insert_range(&mut self, y: isize, range: &RangeInclusive<isize>) {
        self.ranges
            .entry(y)
            .or_default()
            .insert_range(range)
            .unwrap();
    }

    fn remove_point(&mut self, point: (isize, isize)) {
        self.ranges
            .entry(point.1)
            .or_default()
            .remove_range(&(point.0..=point.0))
            .unwrap();
    }
}

fn load(path: &Path) -> Grid {
    let input = std::fs::read_to_string(path).expect("file not found");
    let sensor_regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+)").unwrap();
    let beacon_regex = Regex::new(r"closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for line in input.lines() {
        if let Some(caps) = sensor_regex.captures(line) {
            sensors.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
        }
        if let Some(caps) = beacon_regex.captures(line) {
            beacons.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
        }
    }
    Grid::new(sensors, beacons)
}

#[allow(dead_code)]
pub fn star_1() -> usize {
    let mut grid = load(Path::new("src/dec15/testdata.txt"));
    let (min_x, max_x) = grid.mark_all();
    for i in 0..grid.beacons.len() {
        grid.remove_point(grid.beacons[i]);
    }
    match grid.ranges[&2_000_000].has_range(&(min_x..=max_x)).unwrap() {
        OverlapType::Partial(n) => n as usize,
        _ => panic!("Unexpected overlap type"),
    }
}

#[allow(dead_code)]
pub fn star_2() -> isize {
    let mut grid = load(Path::new("src/dec15/testdata.txt"));
    grid.mark_all();
    for y in 0..=4_000_000 {
        if let Some(range) = grid.ranges.get(&y) {
            match range.has_range(&(0..=4_000_000)).unwrap() {
                OverlapType::Partial(4_000_000) => {
                    for x in 0..=4_000_000 {
                        if range.find_range_with_element(&x).is_err() {
                            return x * 4_000_000 + y;
                        }
                    }
                }
                _ => continue,
            }
        }
    }
    panic!("no solution found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 4724228);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 13622251246513);
    }
}
