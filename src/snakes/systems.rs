use bevy::prelude::{Commands, Entity, Query};

use crate::{grid::prelude::GridPosition, movement::prelude::MoveIntent};

use super::data::{Player, SegmentBundle, Snake};

pub fn slither_system(mut commands: Commands, mut q: Query<(&mut Snake, &Player, &GridPosition)>) {
    for (mut snake, player, position) in q.iter_mut() {
        // Ensure the tail grows with the snakes movement
        let segment = commands
            .spawn_bundle(SegmentBundle::new(*player, *position))
            .id();
        snake.body.insert(0, segment);

        while snake.body.len() >= snake.length {
            if let Some(tail) = snake.body.pop() {
                commands.entity(tail).despawn()
            }
        }
    }
}

pub fn limit_snake_moves(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Snake, Option<&mut MoveIntent>)>,
) {
    for (e, mut snake, intent) in q.iter_mut() {
        if let Some(mut intent) = intent {
            let can_move = snake.body.is_empty() || intent.direction != snake.direction.opposite();
            if !can_move {
                intent.direction = snake.direction;
            }
            snake.direction = intent.direction;
        } else {
            let intent = MoveIntent::from(snake.direction);
            commands.entity(e).insert(intent);
        }
    }
}
