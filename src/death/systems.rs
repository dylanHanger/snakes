use bevy::prelude::{Commands, EventReader, Query};

use crate::snakes::prelude::Snake;

use super::data::*;

pub fn death_system(
    mut commands: Commands,
    mut snakes: Query<&mut Snake>,
    mut deaths: EventReader<DeathEvent>,
) {
    for &DeathEvent { target, culprit } in deaths.iter() {
        if let Ok(mut snake) = snakes.get_mut(target) {
            while let Some(tail) = snake.body.pop() {
                commands.entity(tail).despawn();
            }
            commands.entity(target).despawn();
        }
    }
}
