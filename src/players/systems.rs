use bevy::{
    prelude::{Query, Res},
    sprite::Sprite,
};

use crate::snakes::prelude::Snake;

use super::data::{Player, PlayerColors};

pub fn color_players(
    mut players: Query<(&Player, &mut Sprite, Option<&Snake>)>,
    colors: Res<PlayerColors>,
) {
    for (player, mut sprite, head) in players.iter_mut() {
        if let Some(mut color) = colors.get(player) {
            if head.is_none() {
                color *= 0.6;
            }
            sprite.color = color;
        }
    }
}
