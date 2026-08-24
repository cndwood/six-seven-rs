use std::time::Duration;

use crate::timer::{OneShotTimer, ToggleTimer};

pub struct Effects {
    pub colors: ToggleTimer,
    pub styles: ToggleTimer,
    six_pow: OneShotTimer,
    seven_pow: OneShotTimer,
    pow_duration: Duration,
}

impl Effects {
    pub fn new() -> Self {
        Self {
            colors: ToggleTimer::new(Duration::from_millis(250)),
            styles: ToggleTimer::new(Duration::from_millis(700)),
            six_pow: OneShotTimer::new(),
            seven_pow: OneShotTimer::new(),
            pow_duration: Duration::from_millis(120),
        }
    }

    pub fn pow_six(&mut self) {
        self.six_pow.start(self.pow_duration);
    }

    pub fn pow_seven(&mut self) {
        self.seven_pow.start(self.pow_duration);
    }

    pub fn colors_tick(&mut self) -> bool {
        self.colors.tick()
    }

    pub fn styles_tick(&mut self) -> bool {
        self.styles.tick()
    }

    pub fn six_pow_finished(&mut self) -> bool {
        self.six_pow.finished()
    }

    pub fn seven_pow_finished(&mut self) -> bool {
        self.seven_pow.finished()
    }

    pub fn remaining(&self) -> Option<Duration> {
        [
            self.colors.remaining(),
            self.styles.remaining(),
            self.six_pow.remaining(),
            self.seven_pow.remaining(),
        ]
        .into_iter()
        .flatten()
        .min()
    }

    pub fn colors_enabled(&self) -> bool {
        self.colors.enabled()
    }

    pub fn styles_enabled(&self) -> bool {
        self.styles.enabled()
    }

    pub fn toggle_colors(&mut self) {
        self.colors.toggle();
    }

    pub fn toggle_styles(&mut self) {
        self.styles.toggle();
    }
}
