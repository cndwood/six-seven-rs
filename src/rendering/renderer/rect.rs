use std::time::Duration;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, ResetColor, SetForegroundColor},
};

use crate::{
    Result,
    rendering::{Damage, Effects, art::Art, rect::Rect, renderer::Renderer},
    scene::{PlacedArt, Scene},
};

impl Renderer {
    pub fn draw_art_in_rect(&mut self, art: &Art, rect: &Rect) -> Result<()> {
        for (row, line) in art.text().lines().take(rect.height as usize).enumerate() {
            queue!(
                self.out,
                MoveTo(rect.left(), rect.top() + row as u16),
                Print(line)
            )?;
        }

        Ok(())
    }

    pub fn draw_text_lines(&mut self, text: &str, rect: &Rect) -> Result<()> {
        for (row, line) in text.lines().take(rect.height as usize).enumerate() {
            let clipped: String = line.chars().take(rect.width as usize).collect();

            queue!(
                self.out,
                MoveTo(rect.left(), rect.top() + row as u16),
                Print(clipped)
            )?;
        }

        Ok(())
    }

    pub(super) fn draw_info_contents(
        &mut self,
        scene: &Scene,
        effects: &Effects,
        speed: Duration,
    ) -> Result<()> {
        let keys = "\
    Keys
    F1  Hide info
    6   POW six
    7   POW seven
    c   Toggle colors
    s   Toggle styles
    +/- Change speed
    q   Quit";

        let status = format!(
            "\
    Status
    Style: {}
    Color: {:?}
    Styles: {}
    Colors: {}
    Speed: {} ms",
            scene.six.style.name(),
            scene.six.color,
            on_off(effects.styles_enabled()),
            on_off(effects.colors_enabled()),
            speed.as_millis(),
        );

        self.draw_text_lines(keys, &scene.layout.info_left)?;
        self.draw_text_lines(&status, &scene.layout.info_right)
    }

    pub fn draw_terminal_too_small(
        &mut self,
        screen: &Rect,
        required_width: u16,
        required_height: u16,
    ) -> Result<()> {
        let message = format!(
            "Terminal too small: {}x{}. Required: {}x{}",
            screen.width, screen.height, required_width, required_height,
        );

        let width = (message.chars().count() as u16).min(screen.width);

        let rect = Rect::new(
            screen.left() + screen.width.saturating_sub(width) / 2,
            screen.top() + screen.height / 2,
            width,
            1,
        );

        self.begin_frame()?;
        self.clear_rect(screen)?;
        self.draw_text_lines(&message, &rect)?;
        self.end_frame()
    }

    pub fn draw_info(&mut self, scene: &Scene, effects: &Effects, speed: Duration) -> Result<()> {
        self.begin_frame()?;

        self.clear_rect(&scene.layout.info_left)?;
        self.clear_rect(&scene.layout.info_right)?;
        self.draw_info_contents(scene, effects, speed)?;

        self.end_frame()
    }

    pub fn clear_rect(&mut self, rect: &Rect) -> Result<()> {
        if rect.is_empty() {
            return Ok(());
        }

        let spaces = " ".repeat(rect.width as usize);
        for y in rect.top()..=rect.bottom() {
            queue!(self.out, MoveTo(rect.left(), y), Print(&spaces))?;
        }

        Ok(())
    }

    pub fn draw_placed_art(&mut self, placed: &PlacedArt) -> Result<()> {
        queue!(self.out, SetForegroundColor(placed.color))?;

        self.draw_art_in_rect(&placed.art, &placed.rect)?;

        queue!(self.out, ResetColor)?;

        Ok(())
    }

    pub fn redraw_damage(&mut self, scene: &Scene, damage: &Damage) -> Result<()> {
        if damage.is_empty() {
            return Ok(());
        }

        self.begin_frame()?;

        if let Some(rect) = damage.old_six {
            self.clear_rect(&rect)?;
            self.draw_placed_art(&scene.six)?;
        }

        if let Some(rect) = damage.old_seven {
            self.clear_rect(&rect)?;
            self.draw_placed_art(&scene.seven)?;
        }

        self.end_frame()
    }
}

fn on_off(value: bool) -> &'static str {
    if value { "on" } else { "off" }
}
