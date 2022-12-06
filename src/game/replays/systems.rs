use bevy::prelude::{Commands, Query, Res, ResMut, With};
use chrono::Utc;
use std::fs::File;
use std::io::Write;

use crate::game::food::prelude::Food;
use crate::game::grid::prelude::{GameGrid, GridPosition};
use crate::game::players::prelude::{PlayerId, Players};
use crate::game::snakes::prelude::{Snake, SnakeSegment};
use crate::game::RngSeed;

use super::data::ReplayWriter;

pub fn create_replay(
    mut commands: Commands,
    seed: Res<RngSeed>,
    grid: Res<GameGrid>,
    players: Res<Players>,
) -> Result<(), std::io::Error> {
    // Create a new replay file
    let now = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H-%M-%S").to_string();
    let filename = format!("replays/{}.rpl", timestamp);
    std::fs::create_dir_all("replays/")?;
    let file = File::create(filename)?;

    let mut replay = ReplayWriter::new(file);

    // Write the timestamp to the file
    writeln!(replay, "{}", timestamp)?;
    // Write the game seed to the file
    writeln!(replay, "{}", seed.0)?;
    // Write the game size to the file
    writeln!(replay, "{} {}", grid.width, grid.height)?;
    // For each player, write their ID and name
    for (player, details) in players.iter() {
        writeln!(replay, "{} {}", player.id, details.name)?;
    }

    commands.insert_resource(replay);
    Ok(())
}

pub fn record_replay(
    mut replay: ResMut<ReplayWriter>,
    snakes: Query<(&GridPosition, &Snake, Option<&PlayerId>)>,
    segments: Query<&GridPosition, With<SnakeSegment>>,
    food: Query<(&GridPosition, &Food)>,
) -> Result<(), std::io::Error> {
    // TODO: Optimise this, current format is a lot of wasted space
    let mut sorted_snakes = snakes
        .iter()
        .collect::<Vec<(&GridPosition, &Snake, Option<&PlayerId>)>>();
    sorted_snakes.sort_by_key(|(_, _, p)| {
        if let Some(player) = p {
            player.id
        } else {
            u32::MAX
        }
    });

    // Save food
    for (position, food) in food.iter() {
        write!(replay, "{:.1} {} {} ", food.value, position.x, position.y)?;
    }
    writeln!(replay)?;

    // Save snakes
    for (position, snake, player) in &sorted_snakes {
        if let Some(player) = player {
            write!(replay, "{} ", player.id)?;
        } else {
            write!(replay, "-1 ")?;
        }
        write!(replay, "{} {}", position.x, position.y)?;
        for &body_part in &snake.body {
            if let Ok(position) = segments.get(body_part) {
                write!(replay, " {} {}", position.x, position.y)?;
            }
        }
        write!(replay, ",")?;
    }
    writeln!(replay)?;

    Ok(())
}
