use std::thread;
use std::time::Duration;

use ratatui::DefaultTerminal;

use crate::puzzle::Puzzle;
use crate::solution::Solution;
use crate::solution::base::BaseSolution;

// TODO: This should be able to be SIGINT'd
pub struct TuiSolution {
    terminal: DefaultTerminal,
    base: BaseSolution,
}

impl TuiSolution {
    pub fn init(puzzle: Puzzle) -> Self {
        Self { terminal: ratatui::init(), base: BaseSolution::new(puzzle) }
    }

    pub fn into_base(self) -> BaseSolution {
        self.base
    }
}

impl Solution for TuiSolution {
    fn set(&mut self, index: usize, value: Option<u8>) {
        self.base.set(index, value);
        // TODO: Don't unwrap.
        self.terminal
            .draw(|frame| {
                for x in 0..13_u16 {
                    for y in 0..13_u16 {
                        // TODO: Don't unwrap.
                        let cell = frame.buffer_mut().cell_mut((x, y)).unwrap();
                        if y % 4 == 0 {
                            cell.set_char('-');
                            continue;
                        }
                        if x % 4 == 0 {
                            cell.set_char('|');
                            continue;
                        }
                        let px = x - 1 - x / 4;
                        let py = y - 1 - y / 4;
                        let idx = (py * 9 + px) as usize;
                        let value = self.base.get(idx);
                        // TODO: Use a better way to convert u8 -> char.
                        let chr = value.map(|value| (value + 48) as char).unwrap_or(' ');
                        cell.set_char(chr);
                    }
                }
            })
            .unwrap();

        // TODO: Make this configurable.
        thread::sleep(Duration::from_millis(50));
    }

    fn base(&mut self) -> &mut BaseSolution {
        &mut self.base
    }
}
