use std::{fmt::Display, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Data(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn from_json(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Number(n) => Packet::Data(n.as_u64().unwrap() as usize),
            serde_json::Value::Array(a) => {
                Packet::List(a.into_iter().map(Packet::from_json).collect())
            }
            _ => panic!("unexpected value"),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Data(n) => write!(f, "{}", n),
            Packet::List(packets) => {
                write!(f, "[")?;
                for (idx, packet) in packets.iter().enumerate() {
                    write!(f, "{}", packet)?;
                    if idx < packets.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Data(a), Packet::Data(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.partial_cmp(b) {
                        Some(std::cmp::Ordering::Equal) | None => continue,
                        Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Less),
                        Some(std::cmp::Ordering::Greater) => {
                            return Some(std::cmp::Ordering::Greater)
                        }
                    }
                }
                Some(a.len().cmp(&b.len()))
            }
            (Packet::Data(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Data(*a)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Data(b)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Data(*b)]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

fn load(path: &Path) -> Vec<Pair> {
    let input = std::fs::read_to_string(path).expect("file not found");
    let mut line_no = 0;
    let mut packets = Vec::new();
    let lines = input.lines().collect::<Vec<_>>();
    while line_no < lines.len() {
        let left_json = serde_json::from_str(lines[line_no]).unwrap();
        let left = Packet::from_json(left_json);
        let right_json = serde_json::from_str(lines[line_no + 1]).unwrap();
        let right = Packet::from_json(right_json);
        packets.push(Pair { left, right });
        line_no += 3;
    }
    packets
}

#[allow(dead_code)]
pub fn star_1() -> usize {
    let pairs = load(Path::new("src/dec13/testdata.txt"));
    let mut sum = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if pair.left < pair.right {
            sum += idx + 1;
        }
    }
    sum
}

#[allow(dead_code)]
pub fn star_2() -> usize {
    let pairs = load(Path::new("src/dec13/testdata.txt"));
    let mut packets = pairs.iter().fold(Vec::new(), |mut acc, pair| {
        acc.push(pair.left.clone());
        acc.push(pair.right.clone());
        acc
    });
    // Add divider packets
    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Data(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Data(6)])]);
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();
    (packets.binary_search(&divider_1).unwrap() + 1)
        * (packets.binary_search(&divider_2).unwrap() + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 5340);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 21276);
    }
}
