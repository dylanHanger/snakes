use serde::Deserialize;

use crate::input::prelude::{BuiltinAi, KeyboardInput};

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

#[derive(Debug, Deserialize)]
pub struct PlayerConfig {
    pub name: String,
    pub player_type: PlayerType,
}
impl PlayerConfig {
    pub fn new(name: String, snake_type: PlayerType) -> Self {
        Self {
            name,
            player_type: snake_type,
        }
    }
}