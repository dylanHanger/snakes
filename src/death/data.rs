use bevy::prelude::{Component, Entity};

use crate::players::prelude::Player;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<Player>,
}

#[derive(Component)]
pub struct Respawning {
    pub time: u32,
}

pub struct DeathConfig {
    pub respawn_time: u32,
}