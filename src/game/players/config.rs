use bevy::prelude::Color;
use serde::Deserialize;

use crate::game::input::prelude::{BuiltinAi, KeyboardInput};

use super::prelude::Score;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PlayerType {
    Custom {
        executable: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        silent: bool,
    },
    Builtin {
        difficulty: BuiltinAi,
    },
    Keyboard {
        keys: KeyboardInput,
    },
    Random,
}

#[derive(Debug)]
pub struct PlayerDetails {
    pub name: String,
    pub color: Color,
    pub score: Score,
    pub player_type: PlayerType,
    pub is_dead: bool,
}
