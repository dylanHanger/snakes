use std::time::Duration;

use bevy::core::Timer;

pub struct Turn {
    pub ready: bool,

    pub timer: Timer,
    pub wait_for_all: bool,
}
impl Turn {
    pub fn new(duration: Duration, wait_for_all: bool) -> Self {
        Self {
            ready: false,
            timer: Timer::new(duration, false),
            wait_for_all,
        }
    }
    pub fn reset(&mut self) {
        self.ready = false;
        self.timer.reset();
    }
}
