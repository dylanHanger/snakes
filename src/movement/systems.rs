use super::data::{Direction, *};
use crate::{grid::prelude::*, Actor};

use bevy::{
    input::Input,
    prelude::{Commands, Entity, KeyCode, Query, Res, With, Without},
};
use rand::Rng;

pub fn random_moves_system(
    mut commands: Commands,
    q: Query<Entity, (With<Actor>, With<RandomMoves>, Without<MoveIntent>)>,
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

pub fn keyboard_moves_system(
    mut commands: Commands,
    q: Query<(Entity, &KeyboardMoves), With<Actor>>,
    input: Res<Input<KeyCode>>,
) {
    for (e, controls) in q.iter() {
        let direction = if input.just_pressed(controls.north) {
            Some(Direction::North)
        } else if input.just_pressed(controls.east) {
            Some(Direction::East)
        } else if input.just_pressed(controls.south) {
            Some(Direction::South)
        } else if input.just_pressed(controls.west) {
            Some(Direction::West)
        } else {
            None
        };

        if let Some(dir) = direction {
            commands.entity(e).insert(MoveIntent::from(dir));
        }
    }
}

pub fn movement_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut GridPosition, &MoveIntent)>,
) {
    for (e, mut pos, intent) in q.iter_mut() {
        let dx = intent.delta_x();
        let dy = intent.delta_y();

        pos.x += dx;
        pos.y += dy;

        commands.entity(e).remove::<MoveIntent>();
    }
}
