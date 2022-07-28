use bevy::prelude::{Commands, Query, Res, ResMut};

use crate::game::{
    death::prelude::Respawning,
    input::prelude::{CustomAi, RandomAi},
    snakes::prelude::Snake,
};

use super::{
    config::{PlayerConfig, PlayerType},
    data::{Player, Scoreboard},
};

pub fn scoreboard_system(players: Query<(&Snake, &Player)>, mut scoreboard: ResMut<Scoreboard>) {
    for (snake, player) in players.iter() {
        let score = scoreboard.get_mut(player).unwrap();
        score.current_length = 1 + snake.body.len();
        score.max_length = usize::max(score.max_length, score.current_length);
    }
}

pub fn setup_scoreboard(mut scoreboard: ResMut<Scoreboard>, players: Res<Vec<PlayerConfig>>) {
    for (id, _config) in players.iter().enumerate() {
        let player = Player { id: id as u32 };
        scoreboard.add_player(player);
    }
}

pub fn setup_players(mut commands: Commands, players: Res<Vec<PlayerConfig>>) {
    for (id, config) in players.iter().enumerate() {
        let player = Player { id: id as u32 };
        let e = commands.spawn().insert(player).insert(Respawning).id();

        match &config.player_type {
            PlayerType::Custom { executable, args } => commands
                .entity(e)
                .insert(CustomAi::new(executable.to_string(), args.to_vec())),
            PlayerType::Builtin { difficulty } => commands.entity(e).insert(*difficulty),
            PlayerType::Keyboard { keys } => commands.entity(e).insert(*keys),
            PlayerType::Random => commands.entity(e).insert(RandomAi),
        };
    }
}
