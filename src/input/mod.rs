use crate::Result;
use crossterm::event::{Event as CrosstermEvent, KeyCode, poll, read};
use std::{collections::HashMap, time::Duration};

mod action;

pub use action::Action;

pub struct Input {
    pub bindings: HashMap<KeyCode, Action>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            bindings: default_bindings(),
        }
    }

    fn get_action(&self, key: &KeyCode) -> Option<Action> {
        self.bindings.get(key).copied()
    }

    pub fn read_event(&self, timeout: Duration) -> Result<Option<InputEvent>> {
        if !poll(timeout)? {
            return Ok(None);
        }

        match read()? {
            CrosstermEvent::Key(key) if key.is_press() => {
                Ok(self.get_action(&key.code).map(InputEvent::Action))
            }

            CrosstermEvent::Resize(width, height) => Ok(Some(InputEvent::Resize { width, height })),

            _ => Ok(None),
        }
    }
}

fn default_bindings() -> HashMap<KeyCode, Action> {
    HashMap::from([
        (KeyCode::Char('q'), Action::Quit),
        (KeyCode::Char('6'), Action::PowSix),
        (KeyCode::Char('7'), Action::PowSeven),
        (KeyCode::Char('c'), Action::ToggleColors),
        (KeyCode::Char('s'), Action::ToggleStyles),
        (KeyCode::Char('+'), Action::AddSpeed),
        (KeyCode::Char('-'), Action::ReduceSpeed),
        (KeyCode::F(1), Action::ToggleInfo),
    ])
}

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    Action(Action),
    Resize { width: u16, height: u16 },
}
