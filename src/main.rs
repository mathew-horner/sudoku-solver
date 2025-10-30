use std::str::FromStr;

use clap::Parser;

use crate::algorithms::Backtracking;
use crate::puzzle::Puzzle;

mod algorithms;
#[cfg(debug_assertions)]
mod metrics;
mod puzzle;
mod solution;

/// Since a Sudoku puzzle is represented by a 9x9 grid, there are 9^2 digits
/// in a puzzle.
const PUZZLE_DIGITS: usize = 9_usize.pow(2);

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
    let solution = puzzle.solve::<Backtracking>();
    if cli.pretty_print {
        print!("{}", solution.puzzle);
    } else {
        println!("{}", solution.puzzle.serialize());
    }

    #[cfg(debug_assertions)]
    solution.metrics.log();
}
