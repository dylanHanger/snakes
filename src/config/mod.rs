mod cli;

use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{self, BufReader},
    path::PathBuf,
};

use bevy::{
    prelude::{default, Color},
    utils::HashMap,
};
use serde::{Deserialize, Deserializer};

use crate::game::{
    death::config::DeathConfig,
    food::config::FoodConfig,
    grid::prelude::GameGrid,
    players::{
        config::{PlayerDetails, PlayerType},
        prelude::{PlayerId, Players},
    },
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

    pub seed: Option<u64>,

    pub turns: TurnConfig,
    pub death: DeathConfig,
    pub food: FoodConfig,
    pub players: Players,
}
impl<'de> Deserialize<'de> for GameConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct PlayerWrapper {
            #[serde(flatten)]
            pub player_type: PlayerType,
            // TODO: Colors
        }

        #[derive(Debug, Default, Deserialize)]
        #[serde(default)]
        struct Mapping {
            #[serde(flatten)]
            grid: GameGrid,

            seed: Option<String>,

            #[serde(flatten)]
            turns: TurnConfig,

            #[serde(flatten)]
            death: DeathConfig,

            food: FoodConfig,

            players: Vec<HashMap<String, PlayerWrapper>>,
        }

        let Mapping {
            grid,
            seed,
            turns,
            death,
            food,
            players,
        } = Mapping::deserialize(deserializer)?;

        let mut h = DefaultHasher::new();
        let seed = seed.map(|seed| {
            seed.hash(&mut h);
            h.finish()
        });

        let default_colors = vec![
            Color::rgb(0.8, 0.3, 0.3),
            Color::rgb(0.3, 0.8, 0.3),
            Color::rgb(0.3, 0.3, 0.8),
            Color::rgb(0.8, 0.8, 0.3),
            Color::rgb(0.3, 0.8, 0.8),
            Color::rgb(0.8, 0.3, 0.8),
        ];

        Ok(Self {
            grid,
            seed,
            death,
            turns,
            food,

            players: Players(
                players
                    .iter()
                    .enumerate()
                    .map(|(id, m)| {
                        let (name, player_wrapper) = m.iter().next().unwrap();
                        (
                            PlayerId { id: id as u32 },
                            PlayerDetails {
                                name: name.to_string(),
                                player_type: player_wrapper.player_type.clone(),
                                color: default_colors[id % default_colors.len()],
                                score: default(),
                            },
                        )
                    })
                    .collect(),
            ),
        })
    }
}
