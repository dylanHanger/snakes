use std::{fs::File, io::BufReader, path::PathBuf};

use bevy::utils::HashMap;
use serde::{Deserialize, Deserializer};

use crate::{
    death::config::DeathConfig,
    food::config::FoodConfig,
    grid::prelude::GameGrid,
    players::config::{PlayerConfig, PlayerType},
    turns::config::TurnConfig,
};

pub fn read_config_from_file(path: &PathBuf) -> GameConfig {
    match File::open(path) {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            serde_yaml::from_reader(buf_reader).expect("The config file was malformed")
        }
        Err(e) => panic!("Could not read config file: {}", e),
    }
}

#[derive(Debug)]
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
