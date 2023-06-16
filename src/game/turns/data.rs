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

    pub timer: Option<Timer>,
    pub end_early: bool,

    pub current: u32,
    pub max: u32,
}
impl Turn {
    pub fn new(duration: Option<Duration>, end_early: bool, max_turns: u32) -> Self {
        Self {
            ready: false,
            requested: false,
            timer: duration.map(|duration| Timer::new(duration, bevy::time::TimerMode::Once)),
            end_early,

            current: 0,
            max: max_turns,
        }
    }
    pub fn reset(&mut self) {
        self.ready = false;
        self.requested = false;
        if let Some(timer) = &mut self.timer {
            timer.reset();
        }
    }
}
impl From<TurnConfig> for Turn {
    fn from(config: TurnConfig) -> Self {
        Self::new(
            config.turn_time.map(Duration::from_millis),
            config.end_early,
            config.max_turns,
        )
    }
}
