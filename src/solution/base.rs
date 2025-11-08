use std::collections::HashSet;
use std::ops::Range;

use crate::PUZZLE_DIGITS;
#[cfg(debug_assertions)]
use crate::metrics::Metrics;
use crate::puzzle::Puzzle;
use crate::solution::Solution;

pub struct BaseSolution {
    pub puzzle: Puzzle,
    #[cfg(debug_assertions)]
    pub metrics: Metrics,
}

impl BaseSolution {
    pub fn new(puzzle: Puzzle) -> Self {
        Self { puzzle, metrics: Metrics::default() }
    }

    pub fn get(&mut self, idx: usize) -> Option<u8> {
        #[cfg(debug_assertions)]
        self.metrics.view(idx);
        self.puzzle.data[idx]
    }

    pub fn set(&mut self, idx: usize, value: Option<u8>) {
        #[cfg(debug_assertions)]
        self.metrics.edit(idx);
        self.puzzle.data[idx] = value;
    }

    pub fn iter_puzzle(&mut self) -> impl Iterator<Item = (usize, &Option<u8>)> {
        self.puzzle.data.iter().enumerate().inspect(|(idx, _)| {
            #[cfg(debug_assertions)]
            self.metrics.view(*idx);
        })
    }

    pub fn valid_digit(&mut self, idx: usize) -> bool {
        if idx >= PUZZLE_DIGITS {
            return false;
        }
        let row = row(idx);
        let col = col(idx);
        let (rowr, colr) = quad_ranges(row, col);
        self.valid_row(row) && self.valid_col(col) && self.valid_quad(rowr, colr)
    }

    fn valid_row(&mut self, row: usize) -> bool {
        let mut seen = HashSet::new();
        for col in 0..9 {
            let idx = row * 9 + col;
            if let Some(digit) = self.get(idx) {
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
            if let Some(digit) = self.get(idx) {
                if !seen.insert(digit) {
                    return false;
                }
            }
        }
        true
    }

    fn valid_quad(&mut self, rowr: Range<usize>, colr: Range<usize>) -> bool {
        let mut seen = HashSet::new();
        for row in rowr {
            for col in colr.clone() {
                let idx = row * 9 + col;
                if let Some(digit) = self.get(idx) {
                    if !seen.insert(digit) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Solution for BaseSolution {
    fn set(&mut self, index: usize, value: Option<u8>) {
        self.set(index, value)
    }

    fn base(&mut self) -> &mut BaseSolution {
        self
    }
}

fn row(idx: usize) -> usize {
    idx / 9
}

fn col(idx: usize) -> usize {
    idx % 9
}

fn quad_ranges(row: usize, col: usize) -> (Range<usize>, Range<usize>) {
    match row {
        0..3 => match col {
            0..3 => (0..3, 0..3),
            3..6 => (0..3, 3..6),
            6..9 => (0..3, 6..9),
            _ => unreachable!(),
        },
        3..6 => match col {
            0..3 => (3..6, 0..3),
            3..6 => (3..6, 3..6),
            6..9 => (3..6, 6..9),
            _ => unreachable!(),
        },
        6..9 => match col {
            0..3 => (6..9, 0..3),
            3..6 => (6..9, 3..6),
            6..9 => (6..9, 6..9),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
