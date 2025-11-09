use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::Duration;

use crate::puzzle::Puzzle;
use crate::solution::Solution;
use crate::solution::base::BaseSolution;
use crate::tui::Tui;

/// Renders the solution to a TUI as it is being found.
///
/// This is great for visualizing how an [`Algorithm`] works, and how different
/// ones compare to each other.
pub struct TuiSolution {
    tui: Tui,
    base: BaseSolution,
}

impl TuiSolution {
    pub fn init(puzzle: Puzzle, kill_channel: SyncSender<()>) -> Self {
        Self { tui: Tui::init(kill_channel), base: BaseSolution::new(puzzle) }
    }

    pub fn into_base(self) -> BaseSolution {
        self.base
    }
}

impl Solution for TuiSolution {
    fn set(&mut self, index: usize, value: Option<u8>) {
        self.base.set(index, value);
        // TODO: Don't unwrap here.
        self.tui.render(&self.base.puzzle).unwrap();
        // TODO: Make this configurable.
        thread::sleep(Duration::from_millis(50));
    }

    fn base(&mut self) -> &mut BaseSolution {
        &mut self.base
    }
}
