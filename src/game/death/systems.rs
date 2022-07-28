use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, With, Without};
use bevy_turborand::RngComponent;

use crate::game::{
    grid::prelude::{GameGrid, GridPosition},
    players::prelude::{Player, Scoreboard},
    snakes::prelude::{Snake, SnakeBundle},
};

use super::{config::DeathConfig, data::*};

pub fn death_system(
    mut commands: Commands,
    mut snakes: Query<&mut Snake>,
    mut deaths: EventReader<DeathEvent>,
    players: Query<&Player>,
    mut scoreboard: ResMut<Scoreboard>,
    death_config: Res<DeathConfig>,
) {
    for &DeathEvent { target, culprit } in deaths.iter() {
        if let Ok(mut snake) = snakes.get_mut(target) {
            while let Some(tail) = snake.body.pop() {
                commands.entity(tail).despawn();
            }
            commands
                .entity(target)
                .remove_bundle::<SnakeBundle>()
                .insert(Dead {
                    time: death_config.respawn_time,
                });

            // Update scores
            if let Ok(&player) = players.get(target) {
                if let Some(score) = scoreboard.get_mut(&player) {
                    score.deaths += 1;
                }

                if let Some(other_player) = culprit {
                    if player != other_player {
                        if let Some(score) = scoreboard.get_mut(&other_player) {
                            score.kills += 1;
                        }
                    } else if let Some(score) = scoreboard.get_mut(&player) {
                        score.kills -= 1;
                    }
                }
            }
        }
    }
}

pub fn death_timer_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Dead), Without<Respawning>>,
) {
    for (e, mut respawn) in q.iter_mut() {
        if respawn.time > 0 {
            // Countdown
            respawn.time -= 1;
        } else {
            // Add respawn component
            let mut e = commands.entity(e);
            e.remove::<Dead>().insert(Respawning);
        }
    }
}

pub fn respawn_system(
    mut commands: Commands,
    mut respawns: Query<(Entity, &mut RngComponent, Option<&Player>), With<Respawning>>,
    occupied: Query<&GridPosition>,
    grid: Res<GameGrid>,
) {
    for (e, mut rng, player) in respawns.iter_mut() {
        let rng = rng.get_mut();
        let mut entity = commands.entity(e);
        let position = grid.get_unoccupied_position(&occupied, rng);

        entity
            .remove::<Respawning>()
            .insert_bundle(SnakeBundle::new(position));

        if let Some(player) = player {
            entity.insert(*player);
        }
    }
}
