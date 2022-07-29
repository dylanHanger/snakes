use bevy::prelude::Color;
use serde::Deserialize;

use crate::game::input::prelude::{BuiltinAi, KeyboardInput};

use super::prelude::Score;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PlayerType {
    Custom {
        executable: String,
        args: Vec<String>,
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
}
