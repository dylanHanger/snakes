mod cli;

use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use bevy::utils::HashMap;
use serde::{Deserialize, Deserializer};

use crate::game::{
    death::config::DeathConfig,
    food::config::FoodConfig,
    grid::prelude::GameGrid,
    players::config::{PlayerConfig, PlayerType},
    turns::config::TurnConfig,
};

pub use cli::*;

#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    ParseError(serde_yaml::Error),
}
impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::IOError(error)
    }
}
impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::ParseError(error)
    }
}

pub fn read_config_from_file(path: &PathBuf) -> Result<GameConfig, ConfigError> {
    // Open the config file and read it using serde_yaml
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: GameConfig = serde_yaml::from_reader(reader)?;
    Ok(config)
}

#[derive(Debug, Default)]
pub struct GameConfig {
    pub grid: GameGrid,

    pub turns: TurnConfig,
    pub death: DeathConfig,
    pub food: FoodConfig,
    pub players: Vec<PlayerConfig>,
}
impl<'de> Deserialize<'de> for GameConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Default, Deserialize)]
        #[serde(default)]
        struct Mapping {
            #[serde(flatten)]
            grid: GameGrid,

            #[serde(flatten)]
            turns: TurnConfig,

            #[serde(flatten)]
            death: DeathConfig,

            food: FoodConfig,

            players: Vec<HashMap<String, PlayerType>>,
        }

        let Mapping {
            grid,
            turns,
            death,
            food,
            players,
        } = Mapping::deserialize(deserializer)?;

        Ok(Self {
            grid,
            death,
            turns,
            food,

            players: players
                .iter()
                .map(|m| {
                    let (name, player_type) = m.iter().next().unwrap();
                    PlayerConfig::new(name.to_string(), player_type.clone())
                })
                .collect(),
        })
    }
}
