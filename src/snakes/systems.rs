use bevy::{
    prelude::{default, Color, Commands, Entity, Query},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    grid::prelude::{GridPosition, GridScale},
    movement::prelude::{Direction, MoveIntent},
    Collidable,
};

use super::data::{PlayerId, Snake, SnakeSegment};

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
            .insert(SnakeSegment {
                player: PlayerId(0),
                direction: Direction::North,
            })
            .insert(Collidable)
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
    mut q: Query<(Entity, &Snake, &mut SnakeSegment, Option<&mut MoveIntent>)>,
) {
    for (e, head, mut segment, intent) in q.iter_mut() {
        if let Some(mut intent) = intent {
            let can_move = head.body.is_empty() || intent.direction != segment.direction.opposite();
            if !can_move {
                intent.direction = segment.direction;
            }
            segment.direction = intent.direction;
        } else {
            let intent = MoveIntent::from(segment.direction);
            commands.entity(e).insert(intent);
        }
    }
}
