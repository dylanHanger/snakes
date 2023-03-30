use std::time::Duration;

use bevy::{
    prelude::{Resource, SystemSet},
    time::Timer,
};

use super::config::TurnConfig;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[system_set(base)]
pub enum TurnSet {
    PreTurn,
    PreTurnFlush,
    Request,
    RequestFlush,
    PostRequest,
    PostRequestFlush,
    Simulate,
    SimulateFlush,
    PostSimulate,
    PostSimulateFlush,
}

#[derive(Resource)]
pub struct Turn {
    pub ready: bool,
    pub requested: bool,

    pub timer: Timer,
    pub wait_for_all: bool,

    pub current: u32,
    pub max: u32,
}
impl Turn {
    pub fn new(duration: Duration, wait_for_all: bool, max_turns: u32) -> Self {
        Self {
            ready: false,
            requested: false,
            timer: Timer::new(duration, bevy::time::TimerMode::Once),
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
            Duration::from_millis(config.turn_time.into()),
            config.wait_for_all,
            config.max_turns,
        )
    }
}
