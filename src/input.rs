//! Advent of Code: puzzle input reading

use itertools::Itertools;
use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

/// Path to puzzle input files
const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input");

/// Puzzle input
#[derive(Debug)]
pub struct Input<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> From<R> for Input<R> {
    fn from(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }
}

impl<'a> From<&'a str> for Input<&'a [u8]> {
    fn from(s: &'a str) -> Self {
        Self::from(s.as_bytes())
    }
}

// Constructors
impl Input<File> {
    /// Open puzzle input for the given day
    ///
    /// # Errors
    /// I/O error
    pub fn day(day: usize) -> io::Result<Self> {
        Self::open(&format!("day{day:02}"))
    }

    /// Open puzzle input with the given name
    ///
    /// # Errors
    /// I/O error
    pub fn open(name: &str) -> io::Result<Self> {
        let mut filename: PathBuf = INPUT_PATH.into();
        filename.push(name);
        filename.set_extension("txt");
        let file = File::open(filename)?;
        Ok(file.into())
    }
}

// Consuming all input
impl<R: Read> Input<R> {
    /// Iterator over lines of this input
    pub fn lines(self) -> impl Iterator<Item = io::Result<String>> {
        self.reader.lines()
    }

    /// Iterator over converted lines of this input
    pub fn lines_into<T>(self) -> impl Iterator<Item = io::Result<T>>
    where
        T: From<String>,
    {
        self.lines().map(|line| line.map(Into::into))
    }

    /// Iterator over parsed lines of this input
    pub fn lines_parse<T>(self) -> impl Iterator<Item = io::Result<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.lines().map(|line| {
            line.and_then(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }

    /// Iterator over blocks of lines of this input
    pub fn blocks(self) -> impl Iterator<Item = io::Result<Vec<String>>> {
        fn is_blank_line(line: &io::Result<String>) -> bool {
            line.as_ref().map(|s| s.trim().is_empty()).unwrap_or(false)
        }
        fn is_not_blank_line(line: &io::Result<String>) -> bool {
            !is_blank_line(line)
        }

        self.reader.lines().batching(|lines| {
            let block: io::Result<Vec<_>> = lines
                .skip_while(is_blank_line)
                .take_while(is_not_blank_line)
                .try_collect();
            match block {
                Ok(ref lines) if !lines.is_empty() => Some(block),
                _ => None,
            }
        })
    }

    /// Iterator over blocks of converted lines of this input
    pub fn blocks_into<T>(self) -> impl Iterator<Item = io::Result<Vec<T>>>
    where
        T: From<String>,
    {
        self.blocks()
            .map(|block| block.map(|b| b.into_iter().map(Into::into).collect()))
    }

    /// Iterator over blocks of parsed lines of this input
    pub fn blocks_parse<T>(self) -> impl Iterator<Item = io::Result<Vec<T>>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.blocks().map(|block| {
            block.and_then(|b| {
                b.into_iter()
                    .map(|line| {
                        line.parse()
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    })
                    .try_collect()
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_NUMBERS: &str = "11\n22\n33\n44\n55\n";
    const INPUT_BLOCKS: &str = "11\n22\n\n33\n44\n\n55\n66\n";

    #[derive(Debug)]
    struct TestItem(String);

    impl From<String> for TestItem {
        fn from(s: String) -> Self {
            Self(s)
        }
    }

    impl PartialEq<&str> for TestItem {
        fn eq(&self, other: &&str) -> bool {
            self.0 == *other
        }
    }

    #[test]
    fn day_one() {
        let mut lines = Input::day(1).unwrap().lines();
        let _line = lines.next().unwrap().unwrap();
    }

    #[test]
    fn lines() {
        let lines: Vec<_> = Input::from(INPUT_NUMBERS).lines().try_collect().unwrap();
        assert_eq!(lines, ["11", "22", "33", "44", "55"]);
    }

    #[test]
    fn lines_into() {
        let items: Vec<TestItem> = Input::from(INPUT_NUMBERS)
            .lines_into()
            .try_collect()
            .unwrap();
        assert_eq!(items, ["11", "22", "33", "44", "55"]);
    }

    #[test]
    fn lines_parse() {
        let items: Vec<u32> = Input::from(INPUT_NUMBERS)
            .lines_parse()
            .try_collect()
            .unwrap();
        assert_eq!(items, [11, 22, 33, 44, 55]);
    }

    #[test]
    fn blocks() {
        let blocks: Vec<_> = Input::from(INPUT_BLOCKS).blocks().try_collect().unwrap();
        assert_eq!(blocks, [["11", "22"], ["33", "44"], ["55", "66"]]);
    }

    #[test]
    fn blocks_into() {
        let blocks: Vec<Vec<TestItem>> = Input::from(INPUT_BLOCKS)
            .blocks_into()
            .try_collect()
            .unwrap();
        assert_eq!(blocks, [["11", "22"], ["33", "44"], ["55", "66"]]);
    }

    #[test]
    fn blocks_parse() {
        let blocks: Vec<Vec<u32>> = Input::from(INPUT_BLOCKS)
            .blocks_parse()
            .try_collect()
            .unwrap();
        assert_eq!(blocks, [[11, 22], [33, 44], [55, 66]]);
    }
}
