use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

use crate::PUZZLE_DIGITS;

#[derive(Debug, Eq, PartialEq)]
pub struct Puzzle {
    pub data: [Option<u8>; PUZZLE_DIGITS],
    /// Optional tracking of the cells which were initially filled, restricting
    /// edits to them.
    pub initially_filled: Option<[bool; PUZZLE_DIGITS]>,
}

impl Puzzle {
    pub fn get(&self, idx: usize) -> Option<u8> {
        self.data[idx]
    }

    pub fn set(&mut self, idx: usize, value: Option<u8>) {
        if !self.initially_filled.map(|filled| filled[idx]).unwrap_or_default() {
            self.data[idx] = value;
        }
    }

    pub fn track_initial(&mut self) {
        self.initially_filled = Some(std::array::from_fn(|idx| self.data[idx].is_some()));
    }

    /// Render the puzzle to a single line of 81 digits.
    pub fn serialize(&self) -> String {
        let mut buffer = String::with_capacity(PUZZLE_DIGITS);
        for idx in 0..PUZZLE_DIGITS {
            let char = match self.data[idx] {
                Some(digit) => char::from_digit(digit as u32, 10).unwrap(),
                None => '0',
            };
            buffer.push(char);
        }
        buffer
    }

    pub fn prev_empty(&self, index: usize) -> Option<usize> {
        (0..index).into_iter().rev().find(|&i| self.data[i].is_none())
    }

    pub fn next_empty(&self, index: usize) -> Option<usize> {
        (index + 1..self.data.len()).find(|&i| self.data[i].is_none())
    }

    /// Returns true if all of the squares in the puzzle are filled.
    pub fn is_filled_out(&self) -> bool {
        self.data.iter().all(Option::is_some)
    }

    /// Returns a list of invalid square indices if the puzzle state is invalid.
    pub fn validate(&self) -> Result<(), HashSet<usize>> {
        let mut invalid_squares = HashSet::new();

        for row in 0..9 {
            let mut seen = HashMap::new();
            for col in 0..9 {
                let index = row * 9 + col;
                if let Some(value) = self.data[index] {
                    seen.entry(value).or_insert_with(|| Vec::new()).push(index);
                }
            }
            invalid_squares.extend(seen.values().filter(|idxs| idxs.len() > 1).flatten());
        }

        for col in 0..9 {
            let mut seen = HashMap::new();
            for row in 0..9 {
                let index = row * 9 + col;
                if let Some(value) = self.data[index] {
                    seen.entry(value).or_insert_with(|| Vec::new()).push(index);
                }
            }
            invalid_squares.extend(seen.values().filter(|idxs| idxs.len() > 1).flatten());
        }

        for rowr in [0..3, 3..6, 6..9] {
            for colr in [0..3, 3..6, 6..9] {
                let mut seen = HashMap::new();
                for row in rowr.clone() {
                    for col in colr.clone() {
                        let index = row * 9 + col;
                        if let Some(value) = self.data[index] {
                            seen.entry(value).or_insert_with(|| Vec::new()).push(index);
                        }
                    }
                }
                invalid_squares.extend(seen.values().filter(|idxs| idxs.len() > 1).flatten());
            }
        }

        if !invalid_squares.is_empty() {
            return Err(invalid_squares);
        }

        Ok(())
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self { data: std::array::from_fn(|_| None), initially_filled: None }
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        if input.len() != PUZZLE_DIGITS {
            return Err(anyhow!("puzzle must have {PUZZLE_DIGITS} digits"));
        }

        let mut puzzle = Puzzle::default();

        for (idx, char) in input.chars().enumerate() {
            match char.to_digit(10) {
                // We accept both '0' and '.' to mean "empty square"
                Some(0) => {}
                None if char == '.' => {}
                Some(digit) => {
                    puzzle.data[idx] = Some(digit as u8);
                }
                None => return Err(anyhow!("character {char} at index {idx} is not a digit or period")),
            }
        }

        Ok(puzzle)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Clean this code up.
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
