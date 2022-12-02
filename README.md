# Advent of Code 2022

[![CI](https://github.com/zargony/advent-of-code-2022/workflows/CI/badge.svg)](https://github.com/zargony/advent-of-code-2022/actions)

My solutions to the [Advent of Code 2022](http://adventofcode.com/2022) puzzles, written in [Rust](http://rust-lang.org).

## Goals

My main goal is not to create the quickest / shortest / fastest solution ever, but to create nice,
idiomatic, readable and maintainable Rust code with suitable performance.

Each day's puzzle solver is implemented in a separate binary. All puzzle solvers are developed in
a test-driven approach, i.e. examples from puzzle descriptions are used in unit tests to verify
correct implementation (`cargo test`). Running a solver uses the (personalized) puzzle input to
find the solution (`cargo run`).

## Previous years

- [2021](https://github.com/zargony/advent-of-code-2021) - Rust 2021 using std iterators
- [2020](https://github.com/zargony/advent-of-code-2020) - Rust 2018 using std iterators
- [2019](https://github.com/zargony/advent-of-code-2019) - Rust 2018 using async futures and streams with async-std v1
- [2017](https://github.com/zargony/advent-of-code-2017) - Rust 2015 using nom v3
- [2016](https://github.com/zargony/advent-of-code-2016) - Rust 2015 using nom v2, onig v1
- [2015](https://github.com/zargony/advent-of-code-2015) - Rust 2015 using nom v1, onig v1
