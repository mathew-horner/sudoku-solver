use std::io;
use std::sync::mpsc::SyncSender;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::puzzle::Puzzle;
use crate::tui::layout::{Cell, LAYOUT, X_CELL_COUNT, Y_CELL_COUNT};

mod layout;

pub struct Tui {
    terminal: DefaultTerminal,
    kill_channel: SyncSender<()>,
}

impl Tui {
    pub fn init(kill_channel: SyncSender<()>) -> Self {
        Self { terminal: ratatui::init(), kill_channel }
    }

    pub fn render(&mut self, puzzle: &Puzzle) -> io::Result<()> {
        const IMMEDIATE: Duration = Duration::from_secs(0);

        while let Ok(true) = event::poll(IMMEDIATE) {
            match event::read().unwrap() {
                Event::Key(event) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
                        self.kill_channel.send(()).unwrap();
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

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
        })?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
