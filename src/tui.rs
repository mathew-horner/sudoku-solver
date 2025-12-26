use std::collections::HashSet;
use std::sync::mpsc::SyncSender;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Offset, Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;

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
        let invalid_squares = &self.invalid_squares;

        self.terminal.draw(|frame| {
            let horizontal = Constraint::Length(LAYOUT[0].len() as u16);
            let vertical = Constraint::Length(LAYOUT.len() as u16);
            let grid_rect = frame.area().centered(horizontal, vertical);

            if let Some(cursor_position) = cursor_position {
                frame.set_cursor_position(cursor_position + Offset { x: grid_rect.x as i32, y: grid_rect.y as i32 });
            }

            frame.render_widget(GridWidget { puzzle, invalid_squares }, grid_rect);
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

struct GridWidget<'a> {
    puzzle: &'a Puzzle,
    invalid_squares: &'a HashSet<usize>,
}

impl GridWidget<'_> {
    fn render_square(&self, index: usize, cell: &mut ratatui::buffer::Cell) {
        let char = self.puzzle.get(index).and_then(DigitChar::digit_char).unwrap_or(' ');
        cell.set_char(char);

        if let Some(initially_filled) = &self.puzzle.initially_filled {
            cell.set_fg(if initially_filled[index] { Color::Gray } else { Color::LightBlue });
        }

        if self.invalid_squares.contains(&index) {
            if !self.puzzle.initially_filled.map(|initially_filled| initially_filled[index]).unwrap_or_default() {
                cell.set_fg(Color::Red);
            }
        }
    }
}

impl Widget for GridWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        for y in 0..Y_CELL_COUNT {
            for x in 0..X_CELL_COUNT {
                let cell = buf.cell_mut((area.x + x as u16, area.y + y as u16)).unwrap();
                match LAYOUT[y][x] {
                    Cell::Glyph(glyph) => {
                        cell.set_char(glyph);
                    }
                    Cell::Space => {
                        cell.set_char(' ');
                    }
                    Cell::Square(index) => {
                        self.render_square(index, cell);
                    }
                };
            }
        }
    }
}
