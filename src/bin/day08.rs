use advent_of_code_2022::Input;
use std::{collections::HashMap, error, io::Read};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

#[derive(Debug)]
struct Grid {
    max_x: usize,
    max_y: usize,
    heightmap: HashMap<(usize, usize), u8>,
}

impl<R: Read> TryFrom<Input<R>> for Grid {
    type Error = Box<dyn error::Error>;

    fn try_from(input: Input<R>) -> Result<Self, Self::Error> {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut heightmap = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            max_y = max_y.max(y);
            for (x, ch) in line?.chars().enumerate() {
                max_x = max_x.max(x);
                let height = match ch {
                    '0'..='9' => ch as u8,
                    _ => return Err(ParseError.into()),
                };
                heightmap.insert((x, y), height);
            }
        }
        Ok(Self {
            max_x,
            max_y,
            heightmap,
        })
    }
}

impl Grid {
    fn get_height(&self, x: usize, y: usize) -> u8 {
        self.heightmap.get(&(x, y)).copied().unwrap_or(0)
    }

    fn all(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..=self.max_x).flat_map(|x| (0..=self.max_y).map(move |y| (x, y)))
    }

    fn left_of(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..x).rev().map(move |x| (x, y))
    }

    fn right_of(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        (x + 1..=self.max_x).map(move |x| (x, y))
    }

    fn top_of(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..y).rev().map(move |y| (x, y))
    }

    fn bottom_of(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        (y + 1..=self.max_y).map(move |y| (x, y))
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.get_height(x, y);
        let visible = |(x, y)| self.get_height(x, y) < height;
        self.left_of(x, y).all(visible)
            || self.right_of(x, y).all(visible)
            || self.top_of(x, y).all(visible)
            || self.bottom_of(x, y).all(visible)
    }

    fn count_visible(&self) -> usize {
        self.all().filter(|(x, y)| self.is_visible(*x, *y)).count()
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.get_height(x, y);
        let taller = |(x, y)| self.get_height(x, y) >= height;
        let left_dist = self.left_of(x, y).position(taller).map_or(x, |d| d + 1);
        let right_dist = self
            .right_of(x, y)
            .position(taller)
            .map_or(self.max_x - x, |d| d + 1);
        let top_dist = self.top_of(x, y).position(taller).map_or(y, |d| d + 1);
        let bottom_dist = self
            .bottom_of(x, y)
            .position(taller)
            .map_or(self.max_y - y, |d| d + 1);
        left_dist * right_dist * top_dist * bottom_dist
    }

    fn find_best_scenic_score(&self) -> usize {
        self.all()
            .map(|(x, y)| self.scenic_score(x, y))
            .max()
            .unwrap_or(0)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let grid: Grid = Input::day(8)?.try_into()?;

    let num_visible = grid.count_visible();
    println!("Number of visible trees: {}", num_visible);

    let scenic_score = grid.find_best_scenic_score();
    println!("Best scenic score: {}", scenic_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Grid {
        const INPUT: &str = "30373\n25512\n65332\n33549\n35390\n";
        Input::from(INPUT).try_into().unwrap()
    }

    #[test]
    fn part_1() {
        let grid = grid();

        for x in 0..=4 {
            assert!(grid.is_visible(x, 0)); // top row
            assert!(grid.is_visible(x, 4)); // bottom row
        }
        for y in 0..=4 {
            assert!(grid.is_visible(0, y)); // left column
            assert!(grid.is_visible(4, y)); // right column
        }

        assert!(grid.is_visible(1, 1)); // top-left 5
        assert!(grid.is_visible(2, 1)); // top-middle 5
        assert!(!grid.is_visible(3, 1)); // top-right 1
        assert!(grid.is_visible(1, 2)); // left-middle 5
        assert!(!grid.is_visible(2, 2)); // center 3
        assert!(grid.is_visible(3, 2)); // right-middle 3
        assert!(!grid.is_visible(1, 3)); // bottom-left 3
        assert!(grid.is_visible(2, 3)); // bottom-middle 5
        assert!(!grid.is_visible(3, 3)); // bottom-right 4

        assert_eq!(grid.count_visible(), 21);
    }

    #[test]
    fn part_2() {
        let grid = grid();

        assert_eq!(grid.scenic_score(2, 1), 4);
        assert_eq!(grid.scenic_score(2, 3), 8);

        assert_eq!(grid.find_best_scenic_score(), 8);
    }
}
