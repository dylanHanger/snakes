use bevy::{
    prelude::{default, Color, Commands, Entity, Query},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    grid::prelude::{GridPosition, GridScale},
    movement::prelude::MoveIntent,
};

use super::data::{Snake, SnakeSegment};

pub fn slither_system(mut commands: Commands, mut q: Query<(&mut Snake, &GridPosition)>) {
    for (mut snake, position) in q.iter_mut() {
        // Ensure the tail grows with the snakes movement
        let segment = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.8) * 0.6,
                    ..default()
                },
                ..default()
            })
            .insert(*position)
            .insert(GridScale::square(0.6))
            .insert(SnakeSegment)
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
            if !snake.can_move(intent.direction) {
                intent.direction = snake.direction;
            }
            snake.direction = intent.direction;
        } else {
            let intent = MoveIntent::from(snake.direction);
            commands.entity(e).insert(intent);
        }
    }
}
