use bevy::prelude::{Commands, Query, Res, ResMut, With};
use std::fs::File;
use std::io::Write;

use crate::game::food::config::FoodConfig;
use crate::game::food::prelude::Food;
use crate::game::grid::prelude::{GameGrid, GridPosition};
use crate::game::players::prelude::{PlayerId, Players};
use crate::game::snakes::prelude::{Snake, SnakeSegment};
use crate::game::RngSeed;

use super::config::ReplayConfig;
use super::data::ReplayWriter;

pub fn create_replay(
    mut commands: Commands,
    config: Res<ReplayConfig>,
    seed: Res<RngSeed>,
    grid: Res<GameGrid>,
    players: Res<Players>,
    food: Res<FoodConfig>,
) -> Result<(), std::io::Error> {
    if !config.record {
        return Ok(());
    }

    let full_path = config.get_full_path(&seed);
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = File::create(full_path)?;

    let mut replay = ReplayWriter::new(file);

    // Write the game seed to the file
    writeln!(replay, "{}", seed.0)?;
    // Write the game size to the file
    writeln!(replay, "{} {}", grid.width, grid.height)?;
    // Write the food config to the file
    writeln!(replay, "{} {}", food.initial_lifetime, food.initial_value)?;
    // For each player, write their ID and name
    for (player, details) in players.iter() {
        writeln!(replay, "{} {}", player.id, details.name)?;
    }

    commands.insert_resource(replay);
    Ok(())
}

pub fn record_replay(
    mut replay: ResMut<ReplayWriter>,
    config: Res<ReplayConfig>,
    snakes: Query<(&GridPosition, &Snake, Option<&PlayerId>)>,
    segments: Query<&GridPosition, With<SnakeSegment>>,
    food: Query<(&GridPosition, &Food)>,
) -> Result<(), std::io::Error> {
    if !config.record {
        return Ok(());
    }

    // Save food
    for (position, food) in food.iter() {
        write!(replay, "{} {} {} ", food.lifetime, position.x, position.y)?;
    }
    writeln!(replay)?;

    // Save snakes
    for (position, snake, player) in &snakes {
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
