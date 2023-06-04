use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, Resource)]
#[serde(default)]
pub struct TurnConfig {
    #[serde(rename = "timeout")]
    pub turn_time: u32,
    #[serde(rename = "wait")]
    pub wait_for_all: bool,
    #[serde(rename = "turns")]
    pub max_turns: u32,
    #[serde(rename = "start_paused")]
    pub start_paused: bool,
}
impl Default for TurnConfig {
    fn default() -> Self {
        Self {
            turn_time: 100,
            wait_for_all: false,
            max_turns: 1500,
            start_paused: false,
        }
    }
}
