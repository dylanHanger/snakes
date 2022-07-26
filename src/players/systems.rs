use bevy::{
    prelude::{Query, Res, ResMut},
    sprite::Sprite,
};

use crate::snakes::prelude::Snake;

use super::data::{Player, PlayerColors, Scoreboard};

pub fn color_players(
    mut players: Query<(&Player, &mut Sprite, Option<&Snake>)>,
    colors: Res<PlayerColors>,
) {
    for (player, mut sprite, head) in players.iter_mut() {
        if let Some(mut color) = colors.get(player).cloned() {
            if head.is_none() {
                color *= 0.6;
            }
            sprite.color = color;
        }
    }
}

pub fn scoreboard_system(players: Query<(&Snake, &Player)>, mut scoreboard: ResMut<Scoreboard>) {
    for (snake, player) in players.iter() {
        let score = scoreboard.get_mut(player).unwrap();
        score.current_length = 1 + snake.body.len();
        score.max_length = usize::max(score.max_length, score.current_length);
    }
}

pub fn display_scoreboard(scoreboard: Res<Scoreboard>) {
    println!("{:?}", scoreboard)
}
