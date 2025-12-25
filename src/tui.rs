use std::collections::HashSet;
use std::sync::mpsc::SyncSender;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::layout::Position;
use ratatui::style::Color;

use crate::puzzle::Puzzle;
use crate::tui::layout::{Cell, LAYOUT, X_CELL_COUNT, Y_CELL_COUNT};
use crate::util::{DigitChar, DivRem};

mod layout;

pub trait KeyHandler: Clone + Default + Sized {
    fn handle_key(self, _tui: &mut Tui<Self>, _puzzle: &mut Puzzle, _key: KeyEvent) -> Self {
        self
    }
}

impl KeyHandler for () {}

pub struct Tui<K: KeyHandler = ()> {
    pub cursor_square_index: Option<usize>,
    pub invalid_squares: HashSet<usize>,

    terminal: DefaultTerminal,
    kill_channel: SyncSender<()>,
    key_handler: K,
}

impl<K: KeyHandler> Tui<K> {
    pub fn init(kill_channel: SyncSender<()>) -> Self {
        Self {
            terminal: ratatui::init(),
            kill_channel,
            cursor_square_index: None,
            key_handler: K::default(),
            invalid_squares: HashSet::new(),
        }
    }

    pub fn with_cursor(mut self) -> Self {
        self.cursor_square_index = Some(0);
        self
    }

    pub fn render(&mut self, puzzle: &mut Puzzle) -> Result<()> {
        const IMMEDIATE: Duration = Duration::from_secs(0);

        while let Ok(true) = event::poll(IMMEDIATE) {
            match event::read()? {
                Event::Key(event) => {
                    // BUG: This doesn't work if we have large values for --animation-delay-ms...
                    if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
                        self.kill_channel.send(())?;
                        return Ok(());
                    }

                    self.key_handler = self.key_handler.clone().handle_key(self, puzzle, event);
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
                    let cell = frame.buffer_mut().cell_mut((x as u16, y as u16)).unwrap();
                    match LAYOUT[y][x] {
                        Cell::Glyph(glyph) => {
                            cell.set_char(glyph);
                        }
                        Cell::Space => {
                            cell.set_char(' ');
                        }
                        Cell::Square(idx) => {
                            let char = puzzle.get(idx).and_then(DigitChar::digit_char).unwrap_or(' ');
                            cell.set_char(char);
                            if let Some(initially_filled) = &puzzle.initially_filled {
                                cell.set_fg(if initially_filled[idx] { Color::Gray } else { Color::LightBlue });
                            }
                            if self.invalid_squares.contains(&idx) {
                                if !puzzle
                                    .initially_filled
                                    .map(|initially_filled| initially_filled[idx])
                                    .unwrap_or_default()
                                {
                                    cell.set_fg(Color::Red);
                                }
                            }
                        }
                    };
                }
            }
        })?;
        Ok(())
    }

    fn cursor_position(&self) -> Option<Position> {
        let index = self.cursor_square_index?;
        let (row, col) = index.div_rem(9);
        // TODO: Derive these multipliers from the layout file?
        Some(Position { x: (col * 4 + 2) as u16, y: (row * 2 + 1) as u16 })
    }

    pub fn move_cursor(&mut self, direction: Movement) -> Option<()> {
        let index = self.cursor_square_index?;
        let (mut row, mut col) = index.div_rem(9);
        match direction {
            Movement::Up => row = row.wrapping_sub(1).min(8),
            Movement::Down => row = (row + 1) % 9,
            Movement::Left => col = col.wrapping_sub(1).min(8),
            Movement::Right => col = (col + 1) % 9,
            Movement::To { row: r, column: c } => {
                row = r;
                col = c;
            }
        }
        self.cursor_square_index = Some(row * 9 + col);
        Some(())
    }
}

impl<K: KeyHandler> Drop for Tui<K> {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

pub enum Movement {
    To { row: usize, column: usize },
    Up,
    Down,
    Left,
    Right,
}
