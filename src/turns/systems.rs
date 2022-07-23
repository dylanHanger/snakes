use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{Query, Res, ResMut, Time, With},
};

use crate::{movement::prelude::MoveIntent, Actor};

use super::data::*;

pub fn turn_ready(turn: Res<Turn>) -> ShouldRun {
    if turn.ready {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
pub fn reset_turn_system(mut turn: ResMut<Turn>) {
    turn.reset();
}
pub fn turn_timer_system(mut turn: ResMut<Turn>, time: Res<Time>) {
    turn.timer.tick(time.delta());
}
pub fn turn_ready_system(mut turn: ResMut<Turn>, q: Query<Option<&MoveIntent>, With<Actor>>) {
    if turn.timer.finished() {
        turn.ready = true;
    }
    if turn.wait_for_all {
        for intent in q.iter() {
            if intent.is_none() {
                turn.ready = false;
                break;
            }
        }
    }
}
