use std::time::{Duration, Instant};

pub struct Timer {
    interval: Duration,
    next_tick: Instant,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            next_tick: Instant::now() + interval,
        }
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
        self.next_tick = Instant::now() + interval;
    }

    pub fn remaining(&self) -> Duration {
        self.next_tick.saturating_duration_since(Instant::now())
    }

    pub fn tick(&mut self) -> bool {
        let now = Instant::now();

        if now < self.next_tick {
            return false;
        }

        while self.next_tick <= now {
            self.next_tick += self.interval;
        }

        true
    }
}

pub struct ToggleTimer {
    interval: Duration,
    next_tick: Instant,
    enabled: bool,
}

impl ToggleTimer {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            next_tick: Instant::now() + interval,
            enabled: false,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        self.next_tick = Instant::now() + self.interval;
    }

    pub fn remaining(&self) -> Option<Duration> {
        self.enabled
            .then(|| self.next_tick.saturating_duration_since(Instant::now()))
    }

    pub fn tick(&mut self) -> bool {
        let now = Instant::now();

        if !self.enabled || now < self.next_tick {
            return false;
        }

        while self.next_tick <= now {
            self.next_tick += self.interval;
        }

        true
    }
}

pub struct OneShotTimer {
    deadline: Option<Instant>,
}

impl OneShotTimer {
    pub fn new() -> Self {
        Self { deadline: None }
    }

    pub fn start(&mut self, duration: Duration) {
        self.deadline = Some(Instant::now() + duration);
    }

    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }

    pub fn finished(&mut self) -> bool {
        let Some(deadline) = self.deadline else {
            return false;
        };

        if Instant::now() < deadline {
            return false;
        }

        self.deadline = None;
        true
    }
}
