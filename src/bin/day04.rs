use advent_of_code_2022::Input;
use itertools::Itertools;
use std::{error, ops::RangeInclusive, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

fn parse_range(s: &str) -> Result<RangeInclusive<u32>, ParseError> {
    let mut numbers = s.split('-');
    match (numbers.next(), numbers.next()) {
        (Some(start), Some(end)) => Ok(RangeInclusive::new(
            start.parse().map_err(|_e| ParseError)?,
            end.parse().map_err(|_e| ParseError)?,
        )),
        _ => Err(ParseError),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pair(RangeInclusive<u32>, RangeInclusive<u32>);

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s.split(',');
        match (ranges.next(), ranges.next()) {
            (Some(range1), Some(range2)) => Ok(Self(parse_range(range1)?, parse_range(range2)?)),
            _ => Err(ParseError),
        }
    }
}

impl Pair {
    fn fully_contained(&self) -> bool {
        self.0.contains(self.1.start()) && self.0.contains(self.1.end())
            || self.1.contains(self.0.start()) && self.1.contains(self.0.end())
    }

    fn overlap(&self) -> bool {
        self.0.contains(self.1.start())
            || self.0.contains(self.1.end())
            || self.1.contains(self.0.start())
            || self.1.contains(self.0.end())
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let pairs: Vec<Pair> = Input::day(4)?.lines_parse().try_collect()?;

    let num_contained = pairs.iter().filter(|p| p.fully_contained()).count();
    println!("Number of fully contained pairs: {}", num_contained);

    let num_overlap = pairs.iter().filter(|p| p.overlap()).count();
    println!("Number of overlapping pairs: {}", num_overlap);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs() -> Vec<Pair> {
        const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";
        Input::from(INPUT).lines_parse().try_collect().unwrap()
    }

    #[test]
    fn parse() {
        assert_eq!("2-3,6-7".parse::<Pair>().unwrap(), Pair(2..=3, 6..=7));
    }

    #[test]
    fn part_1() {
        let pairs = pairs();
        let contained: Vec<_> = pairs.iter().filter(|p| p.fully_contained()).collect();
        assert_eq!(contained, [&pairs[3], &pairs[4]]);
    }

    #[test]
    fn part_2() {
        let pairs = pairs();
        let overlap: Vec<_> = pairs.iter().filter(|p| p.overlap()).collect();
        assert_eq!(overlap, [&pairs[2], &pairs[3], &pairs[4], &pairs[5]]);
    }
}
