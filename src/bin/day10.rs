use advent_of_code_2022::Input;
use itertools::Itertools;
use std::{error, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    NoOp,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let mut token = move || -> Result<&str, ParseError> { parts.next().ok_or(ParseError) };
        let arg = |s: &str| -> Result<i32, ParseError> { s.parse().map_err(|_| ParseError) };
        match token()? {
            "addx" => Ok(Instruction::AddX(arg(token()?)?)),
            "noop" => Ok(Instruction::NoOp),
            _ => Err(ParseError),
        }
    }
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::AddX(_) => 2,
            Self::NoOp => 1,
        }
    }
}

#[derive(Debug)]
struct Cpu<'a> {
    instructions: &'a [Instruction],
    pc: usize,
    cycles: usize,
    x: i32,
}

impl<'a> Cpu<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            pc: 0,
            cycles: 0,
            x: 1,
        }
    }

    fn next_instruction(&self) -> Option<&Instruction> {
        self.instructions.get(self.pc)
    }

    fn next_instruction_cycles(&self) -> usize {
        self.next_instruction()
            .map(Instruction::cycles)
            .unwrap_or(0)
    }

    fn step(&mut self) -> usize {
        let instruction = match self.instructions.get(self.pc) {
            Some(instruction) => instruction,
            None => return 0,
        };
        match instruction {
            Instruction::NoOp => (),
            Instruction::AddX(a) => self.x += *a,
        }
        let cycles = instruction.cycles();
        self.pc += 1;
        self.cycles += cycles;
        cycles
    }

    fn run(&mut self) -> (i32, String) {
        let mut signal_strength = 0;
        let mut crt = String::new();
        loop {
            let next_cycles = self.next_instruction_cycles();
            if (self.cycles + 20) / 40 < (self.cycles + next_cycles + 20) / 40 {
                let cycle = (self.cycles + 20) / 40 * 40 + 20;
                signal_strength += cycle as i32 * self.x;
            }
            for c in 0..next_cycles {
                let pos = (self.cycles + c) % 40;
                if (self.x - pos as i32).abs() <= 1 {
                    crt.push('#');
                } else {
                    crt.push(' ');
                }
                if pos == 39 {
                    crt.push('\n');
                }
            }
            if self.step() == 0 {
                return (signal_strength, crt);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let instructions: Vec<Instruction> = Input::day(10)?.lines_parse().try_collect()?;

    let mut cpu = Cpu::new(&instructions);
    let (signal_strength, crt) = cpu.run();
    println!("Sum of signal strengths: {}", signal_strength);
    println!("CRT output:\n{}", crt);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instructions() -> Vec<Instruction> {
        const INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\n\
            addx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\n\
            addx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\n\
            noop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\n\
            addx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\n\
            noop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\n\
            addx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\n\
            noop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\n\
            addx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\n\
            addx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\n\
            addx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\n\
            noop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\n\
            addx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\n\
            noop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";
        Input::from(INPUT).lines_parse().try_collect().unwrap()
    }

    #[test]
    fn part_1() {
        let instructions = instructions();
        let mut cpu = Cpu::new(&instructions);
        assert_eq!(cpu.run().0, 13140);
    }

    #[test]
    fn part_2() {
        let instructions = instructions();
        let mut cpu = Cpu::new(&instructions);
        assert_eq!(
            cpu.run().1,
            "\
            ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n\
            ###   ###   ###   ###   ###   ###   ### \n\
            ####    ####    ####    ####    ####    \n\
            #####     #####     #####     #####     \n\
            ######      ######      ######      ####\n\
            #######       #######       #######     \n"
        );
    }
}
