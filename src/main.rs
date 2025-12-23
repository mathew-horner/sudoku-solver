use std::str::FromStr;
use std::sync::mpsc;

use clap::{Parser, ValueEnum};

use crate::algorithms::{Algorithm, Backtracking};
use crate::puzzle::Puzzle;
use crate::solution::base::BaseSolution;
use crate::solution::tui::TuiSolution;

mod algorithms;
#[cfg(debug_assertions)]
mod metrics;
mod puzzle;
mod solution;
mod tui;

const PUZZLE_DIGITS: usize = 9_usize.pow(2);

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
    /// 81-digit string representing the puzzle, with unsolved squares as 0s
    puzzle: String,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Algorithmically solve the given puzzle
    Solve {
        /// How to output the solution
        #[arg(value_enum, short, long, default_value_t)]
        output: Output,
    },
    /// Play the given puzzle
    Play,
}

#[derive(Clone, Default, ValueEnum)]
enum Output {
    /// Print the solution in standard puzzle notation
    #[default]
    Standard,
    /// Pretty print the solution
    Pretty,
    /// Render a TUI that shows the solution being found
    Animation,
}

fn main() {
    const ALGORITHM: Backtracking = Backtracking;

    env_logger::init();
    let cli = Cli::parse();
    let puzzle = Puzzle::from_str(&cli.puzzle).unwrap();

    match cli.subcommand {
        Subcommand::Solve { output } => {
            let solution = match output {
                Output::Standard => {
                    let mut solution = BaseSolution::new(puzzle);
                    ALGORITHM.solve(&mut solution, None);
                    print!("{}", solution.puzzle.serialize());
                    solution
                }
                Output::Pretty => {
                    let mut solution = BaseSolution::new(puzzle);
                    ALGORITHM.solve(&mut solution, None);
                    print!("{}", solution.puzzle);
                    solution
                }
                Output::Animation => {
                    let (tx, rx) = mpsc::sync_channel(1);
                    let mut tui = TuiSolution::init(puzzle, tx);
                    ALGORITHM.solve(&mut tui, Some(rx));
                    tui.into_base()
                }
            };

            #[cfg(debug_assertions)]
            solution.metrics.write_logs();
        }
        Subcommand::Play => todo!(),
    }
}
