use bevy::prelude::{Commands, Entity, Query, Res, Without};

use crate::game::{
    grid::prelude::GridPosition,
    movement::prelude::MoveIntent,
    players::prelude::{PlayerId, Players},
};

use super::data::{SegmentBundle, Snake};

pub fn slither_system(
    mut commands: Commands,
    mut q: Query<(&mut Snake, &GridPosition, &MoveIntent, Option<&PlayerId>)>,
    players: Res<Players>,
) {
    for (mut snake, position, intent, player) in q.iter_mut() {
        // Spawn a new tail segment
        let mut segment = commands.spawn_empty();
        let mut bundle = SegmentBundle::new(*position);
        if let Some(player) = player {
            if let Some(details) = players.get(player) {
                segment.insert(*player);
                bundle.sprite.color = details.color;
            }
        }
        segment.insert(bundle);

        // Add the new tail segment to the snake
        snake.body.insert(0, segment.id());

        // Remove the last segment if the snake is too long
        while snake.body.len() >= snake.length as usize {
            if let Some(tail) = snake.body.pop() {
                commands.entity(tail).despawn()
            }
        }

        // Set the new direction of the snake
        snake.direction = intent.direction;
    }
}

pub fn limit_snake_moves(mut q: Query<(&Snake, &mut MoveIntent)>) {
    for (snake, mut intent) in q.iter_mut() {
        if !snake.can_move(intent.direction) {
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
