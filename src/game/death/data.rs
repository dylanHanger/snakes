use bevy::prelude::{Component, Entity};

use crate::game::players::prelude::PlayerId;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<PlayerId>,
}

#[derive(Component)]
pub struct Respawning;

#[derive(Component)]
pub struct Dead {
    pub time: u32,
}
