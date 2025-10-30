use std::collections::HashSet;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow};
use clap::Parser;

#[cfg(debug_assertions)]
mod metrics;

#[cfg(debug_assertions)]
use metrics::Metrics;

/// Since a Sudoku puzzle is represented by a 9x9 grid, there are 9^2 digits
/// in a puzzle.
const PUZZLE_DIGITS: usize = 9_usize.pow(2);

#[derive(Debug, Eq, PartialEq)]
struct Puzzle {
    data: [Option<u8>; PUZZLE_DIGITS],
}

impl Puzzle {
    fn solve(self) -> Solution {
        let mut solution = Solution::new(self);
        let mut pointer = 0;
        let unsolved: Vec<_> = solution
            .puzzle
            .data
            .iter()
            .enumerate()
            .filter(|(idx, digit)| {
                #[cfg(debug_assertions)]
                solution.metrics.view(*idx);
                digit.is_none()
            })
            .map(|(idx, _)| idx)
            .collect();

        fn decrement(pointer: &mut usize) {
            if *pointer == 0 {
                panic!("unsolvable");
            }
            *pointer -= 1;
        }

        while pointer < unsolved.len() {
            let idx = unsolved[pointer];
            let base = solution.puzzle.data[idx].unwrap_or(0);
            #[cfg(debug_assertions)]
            solution.metrics.view(idx);
            let mut found_valid = false;

            for cand in (base + 1)..=9 {
                solution.puzzle.data[idx] = Some(cand);
                #[cfg(debug_assertions)]
                solution.metrics.edit(idx);
                if solution.valid_digit(idx) {
                    found_valid = true;
                    break;
                }
            }

            if found_valid {
                pointer += 1;
            } else {
                #[cfg(debug_assertions)]
                solution.metrics.edit(idx);
                solution.puzzle.data[idx] = None;
                decrement(&mut pointer);
            }
        }

        solution
    }

    fn serialize(&self) -> String {
        let mut s = String::with_capacity(PUZZLE_DIGITS);
        for idx in 0..PUZZLE_DIGITS {
            let char = match self.data[idx] {
                Some(digit) => char::from_digit(digit as u32, 10).unwrap(),
                None => '0',
            };
            s.push(char);
        }
        s
    }
}

fn row(idx: usize) -> usize {
    idx / 9
}

fn col(idx: usize) -> usize {
    idx % 9
}

fn quad_ranges(quad: usize) -> (Range<usize>, Range<usize>) {
    match quad {
        0 => (0..3, 0..3),
        1 => (0..3, 3..6),
        2 => (0..3, 6..9),
        3 => (3..6, 0..3),
        4 => (3..6, 3..6),
        5 => (3..6, 6..9),
        6 => (6..9, 0..3),
        7 => (6..9, 3..6),
        8 => (6..9, 6..9),
        _ => unreachable!(),
    }
}

fn quad(row: usize, col: usize) -> usize {
    match row {
        0..3 => match col {
            0..3 => 0,
            3..6 => 1,
            6..9 => 2,
            _ => unreachable!(),
        },
        3..6 => match col {
            0..3 => 3,
            3..6 => 4,
            6..9 => 5,
            _ => unreachable!(),
        },
        6..9 => match col {
            0..3 => 6,
            3..6 => 7,
            6..9 => 8,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self { data: std::array::from_fn(|_| None) }
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != PUZZLE_DIGITS {
            return Err(anyhow!("puzzle must have {PUZZLE_DIGITS} digits"));
        }

        let mut puzzle = Puzzle::default();

        for (idx, char) in s.chars().enumerate() {
            match char.to_digit(10).ok_or_else(|| anyhow!("character {char} at index {idx} is not a digit"))? {
                0 => {}
                digit => {
                    puzzle.data[idx] = Some(digit as u8);
                }
            }
        }

        Ok(puzzle)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let horizontal_bar = str::repeat("-", 13);
        for (idx, digit) in self.data.iter().enumerate() {
            if idx % (9 * 3) == 0 {
                writeln!(f, "{horizontal_bar}")?;
            }
            if idx % 3 == 0 {
                write!(f, "|")?;
            }
            match digit {
                Some(digit) => write!(f, "{digit}")?,
                None => write!(f, ".")?,
            }
            if idx % 9 == 9 - 1 {
                writeln!(f, "|")?;
            }
        }
        writeln!(f, "{horizontal_bar}")?;
        Ok(())
    }
}

struct Solution {
    puzzle: Puzzle,

    #[cfg(debug_assertions)]
    metrics: Metrics,
}

impl Solution {
    fn new(puzzle: Puzzle) -> Self {
        Self { puzzle, metrics: Metrics::default() }
    }

    fn valid_digit(&mut self, idx: usize) -> bool {
        if idx >= PUZZLE_DIGITS {
            return false;
        }
        let row = row(idx);
        let col = col(idx);
        let quad = quad(row, col);
        self.valid_row(row) && self.valid_col(col) && self.valid_quad(quad)
    }

    fn valid_row(&mut self, row: usize) -> bool {
        let mut seen = HashSet::new();
        for col in 0..9 {
            let idx = row * 9 + col;
            #[cfg(debug_assertions)]
            self.metrics.view(idx);
            if let Some(digit) = &self.puzzle.data[idx] {
                if !seen.insert(digit) {
                    return false;
                }
            }
        }
        true
    }

    fn valid_col(&mut self, col: usize) -> bool {
        let mut seen = HashSet::new();
        for row in 0..9 {
            let idx = row * 9 + col;
            #[cfg(debug_assertions)]
            self.metrics.view(idx);
            if let Some(digit) = &self.puzzle.data[idx] {
                if !seen.insert(digit) {
                    return false;
                }
            }
        }
        true
    }

    fn valid_quad(&mut self, quad: usize) -> bool {
        let (rowr, colr) = quad_ranges(quad);
        let mut seen = HashSet::new();
        for row in rowr {
            for col in colr.clone() {
                let idx = row * 9 + col;
                #[cfg(debug_assertions)]
                self.metrics.view(idx);
                if let Some(digit) = &self.puzzle.data[idx] {
                    if !seen.insert(digit) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[derive(Parser)]
struct Cli {
    /// 81-digit string representing the puzzle, with unsolved squares as 0s
    puzzle: String,

    /// Pretty print the solved puzzle
    #[arg(long = "pretty")]
    pretty_print: bool,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let puzzle = Puzzle::from_str(&cli.puzzle).unwrap();
    let solution = puzzle.solve();
    if cli.pretty_print {
        print!("{}", solution.puzzle);
    } else {
        println!("{}", solution.puzzle.serialize());
    }

    #[cfg(debug_assertions)]
    solution.metrics.log();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let puzzle =
            Puzzle::from_str("050703060007000800000816000000030000005000100730040086906000204840572093000409000")
                .unwrap();
        let solution = puzzle.solve();

        let expected =
            Puzzle::from_str("158723469367954821294816375619238547485697132732145986976381254841572693523469718")
                .unwrap();

        assert_eq!(solution.puzzle, expected);
    }
}
