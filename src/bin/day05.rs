use advent_of_code_2022::Input;
use itertools::Itertools;
use std::{error, io::Read, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

#[derive(Debug, Clone)]
struct Supply(Vec<Vec<char>>);

impl TryFrom<Vec<String>> for Supply {
    type Error = ParseError;

    fn try_from(drawing: Vec<String>) -> Result<Self, Self::Error> {
        let mut stacks = Vec::new();
        for line in drawing.into_iter().rev().skip(1) {
            for (i, item) in line.chars().chunks(4).into_iter().enumerate() {
                while stacks.len() < i + 1 {
                    stacks.push(Vec::new());
                }
                if let Some(('[', ch, ']')) = item.take(3).collect_tuple() {
                    stacks[i].push(ch)
                }
            }
        }
        Ok(Self(stacks))
    }
}

impl Supply {
    fn top_items(&self) -> Vec<char> {
        self.0
            .iter()
            .filter_map(|stack| stack.last().copied())
            .collect()
    }

    fn apply_steps_single(&mut self, steps: &[Step]) {
        for step in steps {
            for _ in 0..step.count {
                let item = self.0[step.from - 1].pop().expect("stack empty");
                self.0[step.to - 1].push(item);
            }
        }
    }

    fn apply_steps_multi(&mut self, steps: &[Step]) {
        for step in steps {
            let from_stack = &mut self.0[step.from - 1];
            let mut items = from_stack.split_off(from_stack.len() - step.count);
            self.0[step.to - 1].append(&mut items);
        }
    }
}

#[derive(Debug)]
struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace();
        if let Some(("move", count, "from", from, "to", to)) = words.collect_tuple() {
            Ok(Self {
                count: count.parse().map_err(|_| ParseError)?,
                from: from.parse().map_err(|_| ParseError)?,
                to: to.parse().map_err(|_| ParseError)?,
            })
        } else {
            Err(ParseError)
        }
    }
}

fn parse<R: Read>(input: Input<R>) -> Result<(Supply, Vec<Step>), Box<dyn error::Error>> {
    let mut blocks = input.blocks();
    let supply: Supply = blocks.next().ok_or(ParseError)??.try_into()?;
    let steps = blocks
        .next()
        .ok_or(ParseError)??
        .iter()
        .map(|s| s.parse())
        .try_collect()?;
    Ok((supply, steps))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let (mut supply1, steps) = parse(Input::day(5)?)?;
    let mut supply2 = supply1.clone();

    supply1.apply_steps_single(&steps);
    let top_items: String = supply1.top_items().iter().collect();
    println!("Top items (single crate steps): {}", top_items);

    supply2.apply_steps_multi(&steps);
    let top_items: String = supply2.top_items().iter().collect();
    println!("Top items (multi crate steps): {}", top_items);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn supply_and_steps() -> (Supply, Vec<Step>) {
        const INPUT: &str = "    [D]\n[N] [C]\n[Z] [M] [P]\n1   2   3\n
            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        ";
        parse(Input::from(INPUT)).unwrap()
    }

    #[test]
    fn part_1() {
        let (mut supply, steps) = supply_and_steps();
        supply.apply_steps_single(&steps);
        assert_eq!(supply.0[0], ['C']);
        assert_eq!(supply.0[1], ['M']);
        assert_eq!(supply.0[2], ['P', 'D', 'N', 'Z']);
        assert_eq!(supply.top_items(), ['C', 'M', 'Z']);
    }

    #[test]
    fn part_2() {
        let (mut supply, steps) = supply_and_steps();
        supply.apply_steps_multi(&steps);
        assert_eq!(supply.0[0], ['M']);
        assert_eq!(supply.0[1], ['C']);
        assert_eq!(supply.0[2], ['P', 'Z', 'N', 'D']);
        assert_eq!(supply.top_items(), ['M', 'C', 'D']);
    }
}
