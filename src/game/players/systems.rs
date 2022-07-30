use bevy::prelude::{Commands, Query, Res, ResMut};
use bevy_turborand::{GlobalRng, RngComponent};

use crate::game::{
    death::prelude::Respawning,
    input::prelude::{CustomAi, RandomAi},
    snakes::prelude::Snake,
};

use super::{config::PlayerType, data::PlayerId, prelude::Players};

pub fn scoreboard_system(snakes: Query<(&Snake, &PlayerId)>, mut players: ResMut<Players>) {
    for (snake, player) in snakes.iter() {
        let details = players.get_mut(player).unwrap();
        details.score.current_length = 1 + snake.body.len();
        details.score.max_length =
            usize::max(details.score.max_length, details.score.current_length);
    }
}

pub fn setup_players(
    mut commands: Commands,
    players: Res<Players>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let mut players = players.iter().collect::<Vec<_>>();
    players.sort_by_key(|(&id, _)| *id);

    for (&id, details) in players.iter() {
        let e = commands
            .spawn()
            .insert(RngComponent::from_global(&mut global_rng))
            .insert(id)
            .insert(Respawning)
            .id();

        match &details.player_type {
            PlayerType::Custom { executable, args } => commands
                .entity(e)
                .insert(CustomAi::new(executable.to_string(), args.to_vec())),
            PlayerType::Builtin { difficulty } => commands.entity(e).insert(*difficulty),
            PlayerType::Keyboard { keys } => commands.entity(e).insert(*keys),
            PlayerType::Random => commands.entity(e).insert(RandomAi),
        };
    }
}
