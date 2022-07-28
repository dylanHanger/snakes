use bevy::prelude::{Component, Entity};

use crate::game::players::prelude::Player;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<Player>,
}

#[derive(Component)]
pub struct Respawning;

#[derive(Component)]
pub struct Dead {
    pub time: u32,
}
