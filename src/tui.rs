use std::io;
use std::sync::mpsc::SyncSender;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::layout::Position;

use crate::puzzle::Puzzle;
use crate::tui::layout::{Cell, LAYOUT, X_CELL_COUNT, Y_CELL_COUNT};

mod layout;

/// Function that owners of [`Tui`] can pass to [`Tui::with_key_handler`] to
/// provide key handling for their use case.
type KeyHandler = Box<dyn Fn(KeyEvent) -> Option<Action>>;

pub struct Tui {
    terminal: DefaultTerminal,
    kill_channel: SyncSender<()>,
    key_handler: KeyHandler,
    cursor_square_index: Option<usize>,
}

impl Tui {
    pub fn init(kill_channel: SyncSender<()>) -> Self {
        Self { terminal: ratatui::init(), kill_channel, key_handler: Box::new(|_| None), cursor_square_index: None }
    }

    pub fn with_key_handler(mut self, key_handler: KeyHandler) -> Self {
        self.key_handler = Box::new(key_handler);
        self
    }

    pub fn with_cursor(mut self) -> Self {
        self.cursor_square_index = Some(0);
        self
    }

    pub fn render(&mut self, puzzle: &mut Puzzle) -> io::Result<()> {
        const IMMEDIATE: Duration = Duration::from_secs(0);

        while let Ok(true) = event::poll(IMMEDIATE) {
            match event::read().unwrap() {
                Event::Key(event) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
                        // TODO: Do we actually need a channel in every use case, or can we return
                        // something?
                        self.kill_channel.send(()).unwrap();
                        return Ok(());
                    }

                    if let Some(action) = (self.key_handler)(event) {
                        match action {
                            Action::MoveCursor(direction) => {
                                self.move_cursor(direction);
                            }
                            Action::Set(digit) => {
                                // TODO: Should not be able to set initially set squares.
                                if let Some(index) = self.cursor_square_index {
                                    puzzle.set(index, Some(digit));
                                }
                            }
                            Action::Clear => {
                                // TODO: Should not be able to clear initially set squares.
                                if let Some(index) = self.cursor_square_index {
                                    puzzle.set(index, None);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let cursor_position = self.cursor_position();

        self.terminal.draw(|frame| {
            if let Some(cursor_position) = cursor_position {
                frame.set_cursor_position(cursor_position);
            }

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

    fn cursor_row_column(&self) -> Option<(usize, usize)> {
        let cursor_square_index = self.cursor_square_index?;
        let row = cursor_square_index / 9;
        let col = cursor_square_index % 9;
        Some((row, col))
    }

    fn cursor_position(&self) -> Option<Position> {
        let (row, col) = self.cursor_row_column()?;
        // TODO: Derive these multipliers from the layout file?
        Some(Position { x: (col * 4 + 2) as u16, y: (row * 2 + 1) as u16 })
    }

    fn move_cursor(&mut self, direction: Direction) -> Option<()> {
        let (mut row, mut col) = self.cursor_row_column()?;
        match direction {
            Direction::Up => row = row.wrapping_sub(1).min(8),
            Direction::Down => row = (row + 1) % 9,
            Direction::Left => col = col.wrapping_sub(1).min(8),
            Direction::Right => col = (col + 1) % 9,
        }
        self.cursor_square_index = Some(row * 9 + col);
        Some(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

pub enum Action {
    MoveCursor(Direction),
    Set(u8),
    Clear,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
