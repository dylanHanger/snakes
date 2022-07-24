use bevy::prelude::Entity;

use crate::snakes::prelude::Player;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<Player>,
}
