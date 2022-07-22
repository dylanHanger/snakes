use super::data::{Direction, *};
use crate::grid::prelude::*;

use bevy::prelude::{Commands, Entity, Query, With, Without};
use rand::Rng;

pub fn random_moves_system(
    mut commands: Commands,
    q: Query<Entity, (With<RandomMoves>, Without<MoveIntent>)>,
) {
    let mut rng = rand::thread_rng();

    for e in q.iter() {
        let random_move = match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        };

        commands.entity(e).insert(MoveIntent::from(random_move));
    }
}

pub fn movement_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut GridPosition, &MoveIntent)>,
) {
    for (e, mut pos, intent) in q.iter_mut() {
        pos.x += intent.x;
        pos.y += intent.y;

        commands.entity(e).remove::<MoveIntent>();
    }
}
