use std::process;
use std::sync::mpsc::{self, TryRecvError};

use crossterm::event::{KeyCode, KeyEvent};

use crate::puzzle::Puzzle;
use crate::tui::{Direction, KeyHandler, Tui};

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
                    tui.move_cursor(Direction::Left);
                }
                KeyCode::Char('j') => {
                    tui.move_cursor(Direction::Down);
                }
                KeyCode::Char('k') => {
                    tui.move_cursor(Direction::Up);
                }
                KeyCode::Char('l') => {
                    tui.move_cursor(Direction::Right);
                }
                KeyCode::Backspace => {
                    // Safe unwrap since we call Tui::with_cursor at instantiation.
                    puzzle.set(tui.cursor_square_index.unwrap(), None);
                }
                KeyCode::Enter => {
                    if let Err(invalid_squares) = puzzle.validate() {
                        tui.invalid_squares = invalid_squares;
                    } else {
                        tui.invalid_squares.clear();
                    }
                }
                KeyCode::Char('g') => {
                    return Self::GoRow;
                }
                KeyCode::Char(char) => {
                    if let Some(digit) = char.to_digit(10) {
                        // Safe unwrap since we call Tui::with_cursor at instantiation.
                        puzzle.set(tui.cursor_square_index.unwrap(), Some(digit as u8));

                        if puzzle.is_filled_out() {
                            match puzzle.validate() {
                                Ok(_) => {
                                    // TODO: Do something better than just exit here.
                                    process::exit(0);
                                }
                                Err(invalid_squares) => tui.invalid_squares = invalid_squares,
                            }
                        }
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
                            tui.move_cursor(Direction::To { row: row - 1, column: digit as usize - 1 });
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

pub fn play(mut puzzle: Puzzle) {
    puzzle.track_initial();
    let (tx, rx) = mpsc::sync_channel(1);
    let mut tui = Tui::<GameKeys>::init(tx).with_cursor();
    while let Err(TryRecvError::Empty) = rx.try_recv() {
        // TODO: Don't unwrap.
        tui.render(&mut puzzle).unwrap();
    }
}
