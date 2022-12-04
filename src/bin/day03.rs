use advent_of_code_2022::Input;
use itertools::Itertools;
use std::error;

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

#[derive(Debug)]
struct Rucksack {
    items: String,
}

impl From<String> for Rucksack {
    fn from(items: String) -> Self {
        Self { items }
    }
}

impl Rucksack {
    fn compartments(&self) -> (&str, &str) {
        self.items.split_at(self.items.len() / 2)
    }

    fn common_item(&self) -> Option<char> {
        let (a, b) = self.compartments();
        a.chars().find(|&item| b.contains(item))
    }

    fn common_item_priority(&self) -> u32 {
        self.common_item().map(item_priority).unwrap_or(0)
    }
}

fn find_badge_item(rucksacks: &[Rucksack]) -> Option<char> {
    rucksacks[0]
        .items
        .chars()
        .find(|&item| rucksacks[1..].iter().all(|r| r.items.contains(item)))
}

fn find_badge_item_priority(rucksacks: &[Rucksack]) -> u32 {
    find_badge_item(rucksacks).map(item_priority).unwrap_or(0)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let rucksacks: Vec<Rucksack> = Input::day(3)?.lines_into().try_collect()?;

    let common_item_sum: u32 = rucksacks.iter().map(Rucksack::common_item_priority).sum();
    println!("Sum of common item priorities: {}", common_item_sum);

    let badge_item_sum: u32 = rucksacks.chunks(3).map(find_badge_item_priority).sum();
    println!("Sum of badge item priorities: {}", badge_item_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rucksacks() -> Vec<Rucksack> {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
        Input::from(INPUT).lines_into().try_collect().unwrap()
    }

    #[test]
    fn item_priorities() {
        assert_eq!(item_priority('p'), 16);
        assert_eq!(item_priority('L'), 38);
        assert_eq!(item_priority('P'), 42);
        assert_eq!(item_priority('v'), 22);
        assert_eq!(item_priority('t'), 20);
        assert_eq!(item_priority('s'), 19);
    }

    #[test]
    fn part_1() {
        let priorities: Vec<_> = rucksacks()
            .iter()
            .map(Rucksack::common_item_priority)
            .collect();
        assert_eq!(priorities, [16, 38, 42, 22, 20, 19]);
    }

    #[test]
    fn part_2() {
        let priorities: Vec<_> = rucksacks()
            .chunks(3)
            .map(find_badge_item_priority)
            .collect();
        assert_eq!(priorities, [18, 52]);
    }
}
