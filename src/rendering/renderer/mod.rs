use crate::{
    Result,
    rendering::{Effects, Rect},
    scene::Scene,
};
use std::{
    io::{Stdout, Write, stdout},
    time::Duration,
};

mod rect;

use crossterm::{
    queue,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
};

pub struct Renderer {
    out: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { out: stdout() }
    }

    pub(super) fn begin_frame(&mut self) -> Result<()> {
        queue!(self.out, BeginSynchronizedUpdate)?;

        Ok(())
    }

    pub(super) fn end_frame(&mut self) -> Result<()> {
        queue!(self.out, EndSynchronizedUpdate)?;

        self.flush()?;

        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.out.flush()?;
        Ok(())
    }

    pub fn redraw_screen(
        &mut self,
        screen: &Rect,
        scene: &Scene,
        effects: &Effects,
        speed: Duration,
        show_info: bool,
    ) -> Result<()> {
        self.begin_frame()?;

        self.clear_rect(screen)?;

        self.draw_placed_art(&scene.six)?;
        self.draw_placed_art(&scene.seven)?;

        if show_info {
            self.draw_info_contents(scene, effects, speed)?;
        }

        self.end_frame()
    }
}
