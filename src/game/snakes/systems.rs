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
        // Ensure the tail grows with the snakes movement
        let mut segment = commands.spawn_empty();
        let mut bundle = SegmentBundle::new(*position);
        if let Some(player) = player {
            let details = players.get(player).expect("The player should exist");
            segment.insert(*player);
            bundle.sprite.color = details.color * 0.6;
        }
        segment.insert(bundle);

        snake.body.insert(0, segment.id());

        while snake.body.len() >= snake.length as usize {
            if let Some(tail) = snake.body.pop() {
                commands.entity(tail).despawn()
            }
        }

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
