use std::collections::HashSet;
use std::ops::Range;

use anyhow::Result;

use crate::PUZZLE_DIGITS;
#[cfg(debug_assertions)]
use crate::metrics::Metrics;
use crate::puzzle::Puzzle;
use crate::solution::Solution;
use crate::util::DivRem;

/// The base solution that is used by other [`Solution`] implementors.
///
/// [`BaseSolution`] is itself a [`Solution`] as well. Its implementation of
/// which is trivial. It is used when no special behavior is needed when finding
/// a solution to a puzzle.
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
        self.metrics.record_view(idx);
        self.puzzle.get(idx)
    }

    pub fn set(&mut self, idx: usize, value: Option<u8>) {
        #[cfg(debug_assertions)]
        self.metrics.record_edit(idx);
        self.puzzle.set(idx, value);
    }

    pub fn iter_puzzle(&mut self) -> impl Iterator<Item = (usize, &Option<u8>)> {
        self.puzzle.data.iter().enumerate().inspect(|(idx, _)| {
            #[cfg(debug_assertions)]
            self.metrics.record_view(*idx);
        })
    }

    pub fn is_valid_digit(&mut self, idx: usize) -> bool {
        if idx >= PUZZLE_DIGITS {
            return false;
        }
        let (row, col) = idx.div_rem(9);
        let (rowr, colr) = box_ranges(row, col);

        // Per the rules of Sudoku, every square must have a unique value among its row,
        // column, and box.
        self.is_valid_row(row) && self.is_valid_col(col) && self.is_valid_box(rowr, colr)
    }

    fn is_valid_row(&mut self, row: usize) -> bool {
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

    fn is_valid_col(&mut self, col: usize) -> bool {
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

    fn is_valid_box(&mut self, rowr: Range<usize>, colr: Range<usize>) -> bool {
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
    fn set(&mut self, index: usize, value: Option<u8>) -> Result<()> {
        self.set(index, value);
        Ok(())
    }

    fn base(&mut self) -> &mut BaseSolution {
        self
    }
}

/// For the box which the given `row` and `col` lie within, return a tuple of
/// the (row range, column range) which describes that box.
fn box_ranges(row: usize, col: usize) -> (Range<usize>, Range<usize>) {
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
