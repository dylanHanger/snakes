use bevy::prelude::Entity;

use crate::snakes::prelude::PlayerId;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<PlayerId>,
}
