use std::{fs, path::Path};

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Loss,
    Draw,
}

impl RoundResult {
    fn from_string(s: &str) -> Self {
        match s {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => {
                panic!("Unexpected hand: {}", s);
            }
        }
    }
}

struct Round {
    you: Hand,
    result: RoundResult,
}

enum Strategy {
    Original,
    New,
}

impl Hand {
    fn from_unencrypted(s: &str) -> Self {
        match s {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => {
                panic!("Unexpected hand: {}", s);
            }
        }
    }

    fn from_encrypted_strategy(s: &str) -> Self {
        match s {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => {
                panic!("Unexpected hand: {}", s);
            }
        }
    }
}

impl Round {
    fn from_original_strategy(you: Hand, opponent: Hand) -> Self {
        let result = match opponent {
            Hand::Rock => match you {
                Hand::Rock => RoundResult::Draw,
                Hand::Paper => RoundResult::Win,
                Hand::Scissors => RoundResult::Loss,
            },
            Hand::Paper => match you {
                Hand::Rock => RoundResult::Loss,
                Hand::Paper => RoundResult::Draw,
                Hand::Scissors => RoundResult::Win,
            },
            Hand::Scissors => match you {
                Hand::Rock => RoundResult::Win,
                Hand::Paper => RoundResult::Loss,
                Hand::Scissors => RoundResult::Draw,
            },
        };
        Round { you, result }
    }

    fn from_modified_strategy(opponent: Hand, result: RoundResult) -> Self {
        let you = match opponent {
            Hand::Rock => match result {
                RoundResult::Draw => Hand::Rock,
                RoundResult::Win => Hand::Paper,
                RoundResult::Loss => Hand::Scissors,
            },
            Hand::Paper => match result {
                RoundResult::Loss => Hand::Rock,
                RoundResult::Draw => Hand::Paper,
                RoundResult::Win => Hand::Scissors,
            },
            Hand::Scissors => match result {
                RoundResult::Win => Hand::Rock,
                RoundResult::Loss => Hand::Paper,
                RoundResult::Draw => Hand::Scissors,
            },
        };
        Round { you, result }
    }

    fn score(&self) -> u8 {
        let selected = match self.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
        let outcome = match self.result {
            RoundResult::Win => 6,
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
        };
        selected + outcome
    }
}

fn load(path: &Path, strategy: Strategy) -> Vec<Round> {
    let input = fs::read_to_string(path).expect("file not found");
    let mut rounds: Vec<Round> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").map(|p| p.trim()).collect();
        let round = match strategy {
            Strategy::Original => Round::from_original_strategy(
                Hand::from_encrypted_strategy(parts[1]),
                Hand::from_unencrypted(parts[0]),
            ),
            Strategy::New => Round::from_modified_strategy(
                Hand::from_unencrypted(parts[0]),
                RoundResult::from_string(parts[1]),
            ),
        };
        rounds.push(round);
    }
    rounds
}

#[allow(dead_code)]
pub fn star_1() -> u32 {
    let rounds = load(Path::new("src/dec02/testdata.txt"), Strategy::Original);
    let mut total_score = 0;
    for round in rounds {
        total_score += round.score() as u32;
    }
    total_score
}

#[allow(dead_code)]
pub fn star_2() -> u32 {
    let rounds = load(Path::new("src/dec02/testdata.txt"), Strategy::New);
    let mut total_score = 0;
    for round in rounds {
        total_score += round.score() as u32;
    }
    total_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_1() {
        assert_eq!(star_1(), 8933);
    }

    #[test]
    fn test_star_2() {
        assert_eq!(star_2(), 11998);
    }
}
