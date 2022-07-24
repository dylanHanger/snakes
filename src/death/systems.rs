use bevy::prelude::{Commands, Entity, EventReader, Query, Res};

use crate::{
    grid::prelude::{GameGrid, GridPosition},
    snakes::prelude::{Player, Snake, SnakeBundle},
};

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
            commands
                .entity(target)
                .remove_bundle::<SnakeBundle>()
                .insert(Respawning { time: 5 });
        }
    }
}

pub fn spawn_snakes_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Respawning, &Player)>,
    occupied: Query<&GridPosition>,
    grid: Res<GameGrid>,
) {
    for (e, mut respawn, player) in q.iter_mut() {
        if respawn.time > 0 {
            // Countdown
            respawn.time -= 1;
        } else {
            // Respawn
            let position = grid.get_unoccupied_position(&occupied);
            commands
                .entity(e)
                .insert_bundle(SnakeBundle::new(position))
                .insert(*player)
                .remove::<Respawning>();
        }
    }
}
