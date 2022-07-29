use bevy::prelude::*;

use crate::game::{
    death::prelude::DeathEvent,
    food::prelude::Food,
    grid::prelude::{GameGrid, GridPosition},
    players::prelude::PlayerId,
    snakes::prelude::Snake,
};

use super::data::Collidable;

pub fn collision_system(
    snakes: Query<(Entity, &GridPosition), With<Snake>>,
    collidables: Query<
        (Entity, &GridPosition, Option<&PlayerId>),
        (With<Collidable>, Without<Food>),
    >,
    grid: Res<GameGrid>,
    mut deaths: EventWriter<DeathEvent>,
) {
    for (e1, position1) in snakes.iter() {
        if !grid.contains_position(position1) {
            // Collided with bounds of arena
            // Kill this snake
            deaths.send(DeathEvent {
                target: e1,
                culprit: None,
            });
        }

        for (e2, position2, player) in collidables.iter() {
            if e1 == e2 {
                continue;
            }

            if position1 == position2 {
                // A collision has occured
                if let Some(&player) = player {
                    // It was with another snake
                    // Kill this snake
                    deaths.send(DeathEvent {
                        target: e1,
                        culprit: Some(player),
                    })
                } else {
                    // Collided with something else
                    deaths.send(DeathEvent {
                        target: e1,
                        culprit: None,
                    });
                }
            }
        }
    }
}
