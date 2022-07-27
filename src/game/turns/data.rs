use std::time::Duration;

use bevy::{core::Timer, prelude::StageLabel};

use super::config::TurnConfig;

#[derive(StageLabel, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TurnStage {
    PreTurn,
    Request,
    PostRequest,
    Simulate,
    PostSimulate,
}

pub struct Turn {
    pub ready: bool,
    pub requested: bool,

    pub timer: Timer,
    pub wait_for_all: bool,

    pub current: u64,
    pub max: u64,
}
impl Turn {
    pub fn new(duration: Duration, wait_for_all: bool, max_turns: u64) -> Self {
        Self {
            ready: false,
            requested: false,
            timer: Timer::new(duration, false),
            wait_for_all,

            current: 0,
            max: max_turns,
        }
    }
    pub fn reset(&mut self) {
        self.ready = false;
        self.requested = false;
        self.timer.reset();
    }
}
impl From<TurnConfig> for Turn {
    fn from(config: TurnConfig) -> Self {
        Self::new(
            Duration::from_millis(config.turn_time),
            config.wait_for_all,
            config.max_turns,
        )
    }
}
