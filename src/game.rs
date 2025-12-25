use std::sync::mpsc::{self, TryRecvError};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use crate::math::DivRem;
use crate::puzzle::Puzzle;
use crate::tui::{KeyHandler, Movement, Tui};

#[derive(Clone, Default)]
enum GameKeys {
    #[default]
    Normal,
    GoRow,
    GoColumn {
        row: usize,
    },
}

impl KeyHandler for GameKeys {
    fn handle_key(self, tui: &mut Tui<Self>, puzzle: &mut Puzzle, key: KeyEvent) -> Self {
        match self {
            Self::Normal => match key.code {
                KeyCode::Char('h') => {
                    tui.move_cursor(Movement::Left);
                }
                KeyCode::Char('j') => {
                    tui.move_cursor(Movement::Down);
                }
                KeyCode::Char('k') => {
                    tui.move_cursor(Movement::Up);
                }
                KeyCode::Char('l') => {
                    tui.move_cursor(Movement::Right);
                }
                KeyCode::Backspace => {
                    puzzle.set(tui.cursor_square_index.unwrap(), None);
                }
                KeyCode::Char(',') => {
                    let index = tui.cursor_square_index.unwrap();
                    if let Some(prev_index) = puzzle.prev_empty(index) {
                        let (row, column) = prev_index.div_rem(9);
                        tui.move_cursor(Movement::To { row, column });
                    }
                }
                KeyCode::Char('.') => {
                    let index = tui.cursor_square_index.unwrap();
                    if let Some(next_index) = puzzle.next_empty(index) {
                        let (row, column) = next_index.div_rem(9);
                        tui.move_cursor(Movement::To { row, column });
                    }
                }
                KeyCode::Char('g') => {
                    return Self::GoRow;
                }
                KeyCode::Char(char) => {
                    if let Some(digit) = char.to_digit(10) {
                        puzzle.set(tui.cursor_square_index.unwrap(), Some(digit as u8));
                    }
                }
                _ => {}
            },
            Self::GoRow => match key.code {
                KeyCode::Esc => {
                    return Self::Normal;
                }
                KeyCode::Char(char) => {
                    if let Some(digit) = char.to_digit(10) {
                        return Self::GoColumn { row: digit as usize };
                    }
                }
                _ => {}
            },
            Self::GoColumn { row } => match key.code {
                KeyCode::Esc => {
                    return Self::Normal;
                }
                KeyCode::Char(char) => {
                    if let Some(digit) = char.to_digit(10) {
                        // We 1-index the g-<row>-<column> command, so g-0-<column> and g-<row>-0 are
                        // non-sensical.
                        if row > 0 && digit > 0 {
                            tui.move_cursor(Movement::To { row: row - 1, column: digit as usize - 1 });
                        }
                        return Self::Normal;
                    }
                }
                _ => {}
            },
        }
        self
    }
}

pub fn play(mut puzzle: Puzzle) -> Result<()> {
    puzzle.track_initial();
    let (tx, rx) = mpsc::sync_channel(1);
    let mut tui = Tui::<GameKeys>::init(tx).with_cursor();
    while let Err(TryRecvError::Empty) = rx.try_recv() {
        tui.render(&mut puzzle)?;
    }
    Ok(())
}
