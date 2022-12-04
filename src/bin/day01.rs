use advent_of_code_2022::Input;
use itertools::Itertools;
use std::{
    error,
    io::{self, Read},
};

fn parse<R: Read>(input: Input<R>) -> io::Result<Vec<Vec<u32>>> {
    input.blocks_parse().try_collect()
}

fn max_calories<C: AsRef<[u32]>>(calories: &[C]) -> u32 {
    calories
        .iter()
        .map(|c| c.as_ref().iter().sum())
        .max()
        .unwrap_or(0)
}

fn top_calories<C: AsRef<[u32]>>(calories: &[C], n: usize) -> u32 {
    let mut calories_totals: Vec<u32> = calories.iter().map(|c| c.as_ref().iter().sum()).collect();
    calories_totals.sort_unstable();
    calories_totals.iter().rev().take(n).sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let calories = parse(Input::day(1)?)?;

    println!("Max calories: {}", max_calories(&calories));

    println!("Top 3 calories: {}", top_calories(&calories, 3));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calories() -> Vec<Vec<u32>> {
        const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        parse(Input::from(INPUT)).unwrap()
    }

    #[test]
    fn part_1() {
        assert_eq!(max_calories(&calories()), 24000);
    }

    #[test]
    fn part_2() {
        assert_eq!(top_calories(&calories(), 3), 45000);
    }
}
