use crate::{Result, rendering::Rect};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use std::io::{Stdout, stdout};

pub struct Terminal {
    out: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Self { out: stdout() }
    }

    pub fn screen_rect(&self) -> Result<Rect> {
        let (width, height) = size()?;

        Ok(Rect::new(0, 0, width, height))
    }

    pub fn enter(&mut self) -> Result<()> {
        enable_raw_mode()?;

        execute!(
            self.out,
            EnterAlternateScreen,
            Hide,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        Ok(())
    }

    pub fn leave(&mut self) -> Result<()> {
        execute!(self.out, Show, LeaveAlternateScreen)?;

        disable_raw_mode()?;

        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.leave();
    }
}
