use bevy::prelude::{Query, Res, ResMut, Time, With};

use crate::game::{movement::prelude::MoveIntent, Actor};

use super::data::*;

pub fn turns_finished(turn: Res<Turn>) -> bool {
    turn.current >= turn.max
}
pub fn turn_ready(turn: Res<Turn>) -> bool {
    turn.ready
}
pub fn turn_requested(turn: Res<Turn>) -> bool {
    turn.requested
}
pub fn end_turn_system(mut turn: ResMut<Turn>) {
    turn.reset();
    turn.current += 1;
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
