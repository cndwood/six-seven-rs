use std::time::Duration;

use crate::{
    Result,
    input::{Action, Input, InputEvent},
    rendering::{Damage, Effects, InvertedVerticalAnimation, Layout, Rect, Renderer, Terminal},
    scene::{Number, Scene},
    timer::Timer,
};

pub struct App {
    terminal: Terminal,
    renderer: Renderer,
    input: Input,
    scene: Scene,
    animation: InvertedVerticalAnimation,
    animation_timer: Timer,
    effects: Effects,
    show_info: bool,
    screen: Rect,
    terminal_too_small: bool,
}

const MIN_INTERVAL: Duration = Duration::from_millis(25);
const MAX_INTERVAL: Duration = Duration::from_millis(500);
const SPEED_STEP: Duration = Duration::from_millis(25);
const MIN_TERMINAL_WIDTH: u16 = 40;
const MIN_TERMINAL_HEIGHT: u16 = 16;

impl App {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new();
        let screen = terminal.screen_rect()?;
        let terminal_too_small =
            screen.width < MIN_TERMINAL_WIDTH || screen.height < MIN_TERMINAL_HEIGHT;
        let layout = Layout::new(&screen, false);

        Ok(Self {
            terminal,
            renderer: Renderer::new(),
            input: Input::new(),
            scene: Scene::new(layout),
            animation: InvertedVerticalAnimation::new(6),
            animation_timer: Timer::new(Duration::from_millis(75)),
            effects: Effects::new(),
            show_info: false,
            screen,
            terminal_too_small,
        })
    }

    fn apply_screen(&mut self, screen: Rect) -> Result<()> {
        self.screen = screen;
        self.terminal_too_small = !self.screen_fits(&self.screen);

        if self.terminal_too_small {
            return self.renderer.draw_terminal_too_small(
                &self.screen,
                MIN_TERMINAL_WIDTH,
                self.required_height(),
            );
        }

        let layout = Layout::new(&self.screen, self.show_info);

        self.scene.set_layout(layout);
        self.animation.reset();

        self.renderer.redraw_screen(
            &self.screen,
            &self.scene,
            &self.effects,
            self.animation_timer.interval(),
            self.show_info,
        )
    }

    fn handle_event(&mut self, event: InputEvent) -> Result<bool> {
        match event {
            InputEvent::Action(action) => self.handle_action(action),

            InputEvent::Resize { width, height } => {
                self.apply_screen(Rect::new(0, 0, width, height))?;

                Ok(false)
            }
        }
    }

    fn required_height(&self) -> u16 {
        MIN_TERMINAL_HEIGHT + if self.show_info { 8 } else { 0 }
    }

    fn screen_fits(&self, screen: &Rect) -> bool {
        screen.width >= MIN_TERMINAL_WIDTH && screen.height >= self.required_height()
    }

    fn add_speed(&mut self) {
        let interval = self
            .animation_timer
            .interval()
            .saturating_sub(SPEED_STEP)
            .max(MIN_INTERVAL);

        self.animation_timer.set_interval(interval);
    }

    fn reduce_speed(&mut self) {
        let interval = self
            .animation_timer
            .interval()
            .saturating_add(SPEED_STEP)
            .min(MAX_INTERVAL);

        self.animation_timer.set_interval(interval);
    }

    fn next_timeout(&self) -> Duration {
        if self.terminal_too_small {
            return Duration::from_millis(100);
        }

        let animation = self.animation_timer.remaining();

        self.effects
            .remaining()
            .map_or(animation, |effect| animation.min(effect))
    }

    pub fn run(mut self) -> Result<()> {
        self.terminal.enter()?;
        self.apply_screen(self.screen)?;

        loop {
            if let Some(event) = self.input.read_event(self.next_timeout())?
                && self.handle_event(event)?
            {
                return Ok(());
            }

            if self.terminal_too_small {
                continue;
            }

            self.update_animation()?;
            self.update_effects()?;
        }
    }

    fn update_animation(&mut self) -> Result<()> {
        if !self.animation_timer.tick() {
            return Ok(());
        }

        let Some(damage) = self.animation.update(&mut self.scene) else {
            return Ok(());
        };

        self.renderer.redraw_damage(&self.scene, &damage)
    }

    fn toggle_info(&mut self) -> Result<()> {
        self.show_info = !self.show_info;
        self.apply_screen(self.screen)
    }

    fn redraw_info(&mut self) -> Result<()> {
        if !self.show_info {
            return Ok(());
        }

        self.renderer
            .draw_info(&self.scene, &self.effects, self.animation_timer.interval())
    }

    fn handle_action(&mut self, action: Action) -> Result<bool> {
        if self.terminal_too_small && !matches!(action, Action::Quit | Action::ToggleInfo) {
            return Ok(false);
        }

        match action {
            Action::Quit => Ok(true),
            Action::PowSix => {
                let damage = self.scene.start_pow(Number::Six);
                self.effects.pow_six();
                self.renderer.redraw_damage(&self.scene, &damage)?;
                Ok(false)
            }
            Action::PowSeven => {
                let damage = self.scene.start_pow(Number::Seven);
                self.effects.pow_seven();
                self.renderer.redraw_damage(&self.scene, &damage)?;
                Ok(false)
            }

            Action::ToggleColors => {
                self.effects.toggle_colors();
                self.redraw_info()?;
                Ok(false)
            }

            Action::ToggleStyles => {
                self.effects.toggle_styles();
                self.redraw_info()?;
                Ok(false)
            }

            Action::AddSpeed => {
                self.add_speed();
                self.redraw_info()?;
                Ok(false)
            }

            Action::ReduceSpeed => {
                self.reduce_speed();
                self.redraw_info()?;
                Ok(false)
            }

            Action::ToggleInfo => {
                self.toggle_info()?;
                Ok(false)
            }
        }
    }

    fn update_effects(&mut self) -> Result<()> {
        let mut damage = Damage::default();

        let colors_changed = self.effects.colors_tick();
        let styles_changed = self.effects.styles_tick();

        if colors_changed {
            damage.merge(self.scene.cycle_colors());
        }

        if styles_changed {
            damage.merge(self.scene.cycle_styles());
        }

        if self.effects.six_pow_finished() {
            damage.merge(self.scene.end_pow(Number::Six));
        }

        if self.effects.seven_pow_finished() {
            damage.merge(self.scene.end_pow(Number::Seven));
        }

        self.renderer.redraw_damage(&self.scene, &damage)?;

        if colors_changed || styles_changed {
            self.redraw_info()?;
        }

        Ok(())
    }
}
