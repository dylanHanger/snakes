use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, With, Without};
use bevy_turborand::RngComponent;

use crate::game::{
    collisions::prelude::Collidable,
    grid::prelude::{GameGrid, GridPosition},
    players::prelude::{PlayerId, Players},
    snakes::prelude::{Snake, SnakeBundle},
};

use super::{config::DeathConfig, data::*};

pub fn death_system(
    mut commands: Commands,
    mut snakes: Query<&mut Snake>,
    mut deaths: EventReader<DeathEvent>,
    player_ids: Query<&PlayerId>,
    mut players: ResMut<Players>,
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
            if let Ok(&player) = player_ids.get(target) {
                if let Some(details) = players.get_mut(&player) {
                    details.is_dead = true;
                    details.score.deaths += 1;
                }

                if let Some(other_player) = culprit {
                    if player != other_player {
                        if let Some(details) = players.get_mut(&other_player) {
                            details.score.kills += 1;
                        }
                    } else if let Some(details) = players.get_mut(&player) {
                        details.score.kills -= 1;
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
    mut respawns: Query<(Entity, &mut RngComponent, Option<&PlayerId>), With<Respawning>>,
    occupied: Query<&GridPosition, With<Collidable>>,
    grid: Res<GameGrid>,
    mut players: ResMut<Players>,
) {
    for (e, mut rng, player) in respawns.iter_mut() {
        let rng = rng.get_mut();
        let mut entity = commands.entity(e);
        let position = grid.get_unoccupied_position(&occupied, rng);

        let mut bundle = SnakeBundle::new(position);
        if let Some(player) = player {
            let mut details = players.get_mut(player).expect("The player should exist");
            bundle.sprite.color = details.color;
            entity.insert(*player);

            details.is_dead = false;
        }
        entity.remove::<Respawning>().insert_bundle(bundle);
    }
}
