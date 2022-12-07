use advent_of_code_2022::Input;
use std::{collections::HashMap, error, io::Read};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

#[derive(Debug)]
enum Entry {
    Directory(HashMap<String, usize>),
    File(usize),
}

impl Entry {
    fn new_directory() -> Entry {
        Self::Directory(HashMap::new())
    }

    const fn new_file(size: usize) -> Entry {
        Self::File(size)
    }
}

#[derive(Debug)]
struct Filesystem {
    entries: Vec<Entry>,
    stack: Vec<usize>,
    cwd: usize,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            entries: vec![Entry::new_directory()],
            stack: vec![],
            cwd: 0,
        }
    }

    fn cwd(&mut self) -> &mut HashMap<String, usize> {
        match self.entries[self.cwd] {
            Entry::Directory(ref mut dir) => dir,
            _ => unreachable!(),
        }
    }

    fn cd(&mut self, name: &str) {
        match name {
            "/" => {
                self.stack.clear();
                self.cwd = 0;
            }
            ".." => self.cwd = self.stack.pop().unwrap(),
            _ => {
                self.stack.push(self.cwd);
                self.cwd = *self.cwd().get(name).unwrap();
            }
        }
    }

    fn mknode(&mut self, name: &str, entry: Entry) {
        self.entries.push(entry);
        let id = self.entries.len() - 1;
        self.cwd().insert(name.to_string(), id);
    }

    fn mkdir(&mut self, name: &str) {
        self.mknode(name, Entry::new_directory());
    }

    fn mkfile(&mut self, name: &str, size: usize) {
        self.mknode(name, Entry::new_file(size));
    }

    fn du_id(&self, id: usize) -> usize {
        match self.entries[id] {
            Entry::Directory(ref dir) => dir.values().map(|id| self.du_id(*id)).sum(),
            Entry::File(size) => size,
        }
    }

    #[allow(unused)]
    fn du(&self) -> usize {
        self.du_id(self.cwd)
    }

    fn dir_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(id, entry)| matches!(entry, Entry::Directory(_)).then_some(id))
            .map(|id| self.du_id(id))
    }

    fn sum_of_dir_sizes(&self, max_dir_size: usize) -> usize {
        self.dir_sizes()
            .filter(|size| *size <= max_dir_size)
            .sum::<usize>()
    }

    fn size_of_dir_to_delete(&self, total_size: usize, desired_free_size: usize) -> usize {
        let free_size = total_size - self.du_id(0);
        let size_to_free_up = desired_free_size - free_size;
        self.dir_sizes()
            .filter(|size| *size >= size_to_free_up)
            .min()
            .unwrap_or(0)
    }
}

fn parse<R: Read>(input: Input<R>) -> Result<Filesystem, Box<dyn error::Error>> {
    let mut fs = Filesystem::new();
    for line in input.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some("$"), Some("cd"), Some(name)) => fs.cd(name),
            (Some("$"), Some("ls"), None) => (),
            (Some("dir"), Some(name), None) => fs.mkdir(name),
            (Some(size), Some(name), None) => {
                fs.mkfile(name, size.parse().map_err(|_| ParseError)?)
            }
            _ => return Err(ParseError.into()),
        }
    }
    fs.cd("/");
    Ok(fs)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let fs = parse(Input::day(7)?)?;

    let size = fs.sum_of_dir_sizes(100_000);
    println!("Sum of dir sizes at most 100k: {}", size);

    let size = fs.size_of_dir_to_delete(70_000_000, 30_000_000);
    println!("Size of dir to delete to free up 30M: {}", size);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fs() -> Filesystem {
        const INPUT: &str = "$ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k";
        parse(Input::from(INPUT)).unwrap()
    }

    #[test]
    fn part_1() {
        let mut fs = fs();
        fs.cd("a");
        fs.cd("e");
        assert_eq!(fs.du(), 584);
        fs.cd("..");
        assert_eq!(fs.du(), 94853);
        fs.cd("..");
        fs.cd("d");
        assert_eq!(fs.du(), 24933642);
        fs.cd("..");
        assert_eq!(fs.du(), 48381165);

        assert_eq!(fs.sum_of_dir_sizes(100_000), 95437);
    }

    #[test]
    fn part_2() {
        let fs = fs();
        assert_eq!(fs.size_of_dir_to_delete(70_000_000, 30_000_000), 24933642);
    }
}
