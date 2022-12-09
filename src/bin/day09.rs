use advent_of_code_2022::Input;
use itertools::Itertools;
use std::{collections::HashSet, error, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "U" => Ok(Self::Up),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    distance: usize,
}

impl FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        Ok(Self {
            direction: parts.next().ok_or(ParseError)?.parse()?,
            distance: parts
                .next()
                .ok_or(ParseError)?
                .parse()
                .map_err(|_| ParseError)?,
        })
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); num_knots],
            visited: HashSet::new(),
        }
    }

    fn from_motions(num_knots: usize, motions: &[Motion]) -> Self {
        let mut rope = Self::new(num_knots);
        rope.apply(motions);
        rope
    }

    fn apply(&mut self, motions: &[Motion]) {
        for motion in motions {
            for _ in 0..motion.distance {
                match motion.direction {
                    Direction::Left => self.knots[0].0 -= 1,
                    Direction::Right => self.knots[0].0 += 1,
                    Direction::Down => self.knots[0].1 -= 1,
                    Direction::Up => self.knots[0].1 += 1,
                }
                for k in 1..self.knots.len() {
                    let dx = self.knots[k - 1].0 - self.knots[k].0;
                    let dy = self.knots[k - 1].1 - self.knots[k].1;
                    if dx.abs() > 1 || (dx.abs() > 0 && dy.abs() > 1) {
                        self.knots[k].0 += dx.signum();
                    }
                    if dy.abs() > 1 || (dy.abs() > 0 && dx.abs() > 1) {
                        self.knots[k].1 += dy.signum();
                    }
                }
                self.visited.insert(*self.knots.last().unwrap());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let motions: Vec<Motion> = Input::day(9)?.lines_parse().try_collect()?;

    let rope = Rope::from_motions(2, &motions);
    println!("Positions visited (2 knots): {}", rope.visited.len());

    let rope = Rope::from_motions(10, &motions);
    println!("Positions visited (10 knots): {}", rope.visited.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rope2() -> Rope {
        const INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        let motions: Vec<Motion> = Input::from(INPUT).lines_parse().try_collect().unwrap();
        Rope::from_motions(2, &motions)
    }

    fn rope10() -> Rope {
        const INPUT: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";
        let motions: Vec<Motion> = Input::from(INPUT).lines_parse().try_collect().unwrap();
        Rope::from_motions(10, &motions)
    }

    #[test]
    fn part_1() {
        let rope = rope2();
        assert_eq!(rope.visited.len(), 13);
        assert!(rope.visited.contains(&(1, 0)));
        assert!(rope.visited.contains(&(2, 0)));
        assert!(rope.visited.contains(&(3, 0)));
        assert!(rope.visited.contains(&(4, 1)));
        assert!(rope.visited.contains(&(1, 2)));
        assert!(rope.visited.contains(&(2, 2)));
        assert!(rope.visited.contains(&(3, 2)));
        assert!(rope.visited.contains(&(4, 2)));
        assert!(rope.visited.contains(&(3, 3)));
        assert!(rope.visited.contains(&(4, 3)));
        assert!(rope.visited.contains(&(2, 4)));
        assert!(rope.visited.contains(&(3, 4)));
    }

    #[test]
    fn part_2() {
        let rope = rope10();
        assert_eq!(rope.visited.len(), 36);
    }
}
