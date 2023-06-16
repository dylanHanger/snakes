use bevy::prelude::{NextState, Query, Res, ResMut, Time, With};

use crate::game::{movement::prelude::MoveIntent, Actor, GameState};

use super::data::*;

pub fn pause_after_step(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Paused)
}

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
    if let Some(timer) = &mut turn.timer {
        timer.tick(time.delta());
    }
}
pub fn turn_ready_system(mut turn: ResMut<Turn>, q: Query<Option<&MoveIntent>, With<Actor>>) {
    // let timer_finished = turn.timer.finished();
    // let all_ready = q.iter().all(|intent| intent.is_some());
    // let wait = turn.timer.is_none()
    // let is_waiting = wait && !all_ready;
    //
    //
    // turn.ready = timer_finished && !is_waiting && !game_over;
    let game_over = turn.current >= turn.max;

    let all_ready = q.iter().all(|intent| intent.is_some());
    let turn_over = if let Some(timer) = &turn.timer {
        timer.finished() || (turn.end_early && all_ready)
    } else {
        all_ready
    };

    turn.ready = turn_over && !game_over
}
