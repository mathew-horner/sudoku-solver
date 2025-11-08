use std::io;

use ratatui::{CompletedFrame, DefaultTerminal};

use crate::puzzle::Puzzle;
use crate::tui::layout::{Cell, LAYOUT, X_CELL_COUNT, Y_CELL_COUNT};

mod layout;

pub struct Tui {
    // TODO: This should be able to be SIGINT'd
    terminal: DefaultTerminal,
}

impl Tui {
    pub fn init() -> Self {
        Self { terminal: ratatui::init() }
    }

    pub fn draw(&mut self, puzzle: &Puzzle) -> io::Result<CompletedFrame<'_>> {
        self.terminal.draw(|frame| {
            for y in 0..Y_CELL_COUNT {
                for x in 0..X_CELL_COUNT {
                    // TODO: Don't unwrap.
                    let cell = frame.buffer_mut().cell_mut((x as u16, y as u16)).unwrap();
                    let chr = match LAYOUT[y][x] {
                        Cell::Glyph(glyph) => glyph,
                        Cell::Space => ' ',
                        Cell::Square(idx) => {
                            // TODO: Use a better way to convert u8 -> char.
                            puzzle.get(idx).map(|value| (value + 48) as char).unwrap_or(' ')
                        }
                    };
                    cell.set_char(chr);
                }
            }
        })
    }
}
