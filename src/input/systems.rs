use bevy::prelude::{Commands, Entity, Input, KeyCode, Query, Res, ResMut, With, Without};
use rand::Rng;

use crate::{
    movement::prelude::{Direction, MoveIntent},
    turns::prelude::Turn,
    Actor,
};

use super::data::{ExternalMoves, KeyboardMoves, RandomMoves};

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

pub fn external_moves_system(
    mut commands: Commands,
    agents: Query<(Entity, &ExternalMoves), With<Actor>>,
) {
    for (e, agent) in agents.iter() {
        if let Some(answer) = agent.recv() {
            commands.entity(e).insert(MoveIntent::from(answer));
        }
    }
}

pub fn external_update_system(mut turn: ResMut<Turn>, agents: Query<&ExternalMoves, With<Actor>>) {
    turn.requested = true;
    for agent in agents.iter() {
        agent.send("\n".to_string());
    }
}
