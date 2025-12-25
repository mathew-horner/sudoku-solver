use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::Duration;

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::solution::Solution;
use crate::solution::base::BaseSolution;
use crate::tui::Tui;

const DEFAULT_DELAY_MS: u64 = 50;

/// Renders the solution to a TUI as it is being found.
///
/// This is great for visualizing how an [`Algorithm`] works, and how different
/// ones compare to each other.
pub struct TuiSolution {
    tui: Tui,
    base: BaseSolution,
    delay: Duration,
}

impl TuiSolution {
    pub fn init(puzzle: Puzzle, kill_channel: SyncSender<()>, delay_ms: Option<u64>) -> Self {
        let delay = Duration::from_millis(delay_ms.unwrap_or(DEFAULT_DELAY_MS));
        Self { tui: Tui::init(kill_channel), base: BaseSolution::new(puzzle), delay }
    }

    pub fn into_base(self) -> BaseSolution {
        self.base
    }
}

impl Solution for TuiSolution {
    fn set(&mut self, index: usize, value: Option<u8>) -> Result<()> {
        self.base.set(index, value);
        self.tui.render(&mut self.base.puzzle)?;
        thread::sleep(self.delay);
        Ok(())
    }

    fn base(&mut self) -> &mut BaseSolution {
        &mut self.base
    }
}
