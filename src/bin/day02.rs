use advent_of_code_2022::Input;
use itertools::Itertools;
use std::cmp::Ordering;
use std::error;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(ParseError),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::Rock, Self::Rock) => Ordering::Equal,
            (Self::Rock, Self::Paper) => Ordering::Less,
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Paper, Self::Paper) => Ordering::Equal,
            (Self::Paper, Self::Scissors) => Ordering::Less,
            (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Scissors, Self::Paper) => Ordering::Greater,
            (Self::Scissors, Self::Scissors) => Ordering::Equal,
        })
    }
}

impl Hand {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn score_against(&self, opponent: Hand) -> u32 {
        self.score()
            + if *self > opponent {
                6
            } else if *self == opponent {
                3
            } else {
                0
            }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

impl FromStr for Strategy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(ParseError),
        }
    }
}

impl Strategy {
    // Part 1 stupid strategy: select X->Rock, Y->Paper, Z->Scissors
    fn select_stupid(&self) -> Hand {
        match self {
            Self::Lose => Hand::Rock,
            Self::Draw => Hand::Paper,
            Self::Win => Hand::Scissors,
        }
    }

    // Part 2 smart strategy: select for desired outcome
    fn select_smart(&self, opponent: Hand) -> Hand {
        match (self, opponent) {
            (Self::Lose, Hand::Rock) => Hand::Scissors,
            (Self::Lose, Hand::Paper) => Hand::Rock,
            (Self::Lose, Hand::Scissors) => Hand::Paper,
            (Self::Draw, _) => opponent,
            (Self::Win, Hand::Rock) => Hand::Paper,
            (Self::Win, Hand::Paper) => Hand::Scissors,
            (Self::Win, Hand::Scissors) => Hand::Rock,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Hand,
    strategy: Strategy,
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(char::is_whitespace);
        Ok(Self {
            opponent: parts.next().ok_or(ParseError)?.parse()?,
            strategy: parts.next().ok_or(ParseError)?.parse()?,
        })
    }
}

impl Round {
    fn score_stupid(&self) -> u32 {
        self.strategy.select_stupid().score_against(self.opponent)
    }

    fn score_smart(&self) -> u32 {
        self.strategy
            .select_smart(self.opponent)
            .score_against(self.opponent)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let rounds: Vec<Round> = Input::day(2)?.parsed_lines().try_collect().unwrap();

    let score_stupid: u32 = rounds.iter().map(Round::score_stupid).sum();
    println!("Total score (stupid): {}", score_stupid);

    let score_smart: u32 = rounds.iter().map(Round::score_smart).sum();
    println!("Total score (smart): {}", score_smart);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn part_1() {
        let rounds: Vec<Round> = Input::from(INPUT).parsed_lines().try_collect().unwrap();
        let scores: Vec<_> = rounds.iter().map(Round::score_stupid).collect();
        assert_eq!(scores, [8, 1, 6]);
    }

    #[test]
    fn part_2() {
        let rounds: Vec<Round> = Input::from(INPUT).parsed_lines().try_collect().unwrap();
        let scores: Vec<_> = rounds.iter().map(Round::score_smart).collect();
        assert_eq!(scores, [4, 1, 7]);
    }
}
