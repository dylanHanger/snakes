use bevy::prelude::{Commands, Entity, Query, Without};

use crate::{grid::prelude::GridPosition, movement::prelude::MoveIntent};

use super::data::{Player, SegmentBundle, Snake};

pub fn slither_system(
    mut commands: Commands,
    mut q: Query<(&mut Snake, &Player, &GridPosition, &MoveIntent)>,
) {
    for (mut snake, player, position, intent) in q.iter_mut() {
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

        snake.direction = intent.direction;
    }
}

pub fn limit_snake_moves(mut q: Query<(&Snake, &mut MoveIntent)>) {
    for (snake, mut intent) in q.iter_mut() {
        let can_move = snake.body.is_empty() || intent.direction != snake.direction.opposite();
        if !can_move {
            intent.direction = snake.direction;
        }
    }
}

pub fn default_snake_moves(
    mut commands: Commands,
    q: Query<(Entity, &Snake), Without<MoveIntent>>,
) {
    for (e, snake) in q.iter() {
        commands.entity(e).insert(MoveIntent::from(snake.direction));
    }
}