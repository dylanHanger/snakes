use bevy::prelude::{Component, Entity};

use crate::snakes::prelude::Player;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<Player>,
}

#[derive(Component)]
pub struct Respawning {
    pub time: u32,
}
