use std::sync::mpsc::{self, TryRecvError};

use crossterm::event::KeyCode;

use crate::puzzle::Puzzle;
use crate::tui::{Action, Direction, Tui};

pub fn play(mut puzzle: Puzzle) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut tui = Tui::init(tx).with_cursor().with_key_handler(Box::new(|event| {
        // TODO: This is unideal, but the best I had to satisfy the borrow checker.
        match event.code {
            KeyCode::Char('h') => Some(Action::MoveCursor(Direction::Left)),
            KeyCode::Char('j') => Some(Action::MoveCursor(Direction::Down)),
            KeyCode::Char('k') => Some(Action::MoveCursor(Direction::Up)),
            KeyCode::Char('l') => Some(Action::MoveCursor(Direction::Right)),
            KeyCode::Backspace => Some(Action::Clear),
            KeyCode::Char(char) => char.to_digit(10).and_then(|digit| Some(Action::Set(digit as u8))),
            _ => None,
        }
    }));
    while let Err(TryRecvError::Empty) = rx.try_recv() {
        // TODO: Don't unwrap.
        tui.render(&mut puzzle).unwrap();
    }
}
