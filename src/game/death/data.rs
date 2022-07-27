use bevy::prelude::{Component, Entity};

use crate::game::players::prelude::Player;

pub struct DeathEvent {
    pub target: Entity,
    pub culprit: Option<Player>,
}

#[derive(Component)]
pub struct Respawning {
    pub time: u32,
}
impl Respawning {
    pub fn now() -> Self {
        Self { time: 0 }
    }
}
