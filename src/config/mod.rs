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

// TODO: I can definitely improve this
//          - Only the snakes need the custom logic, I can #derive for the rest
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
        #[derive(Debug, Deserialize)]
        struct SnakeWrapper {
            #[serde(flatten)]
            snake_type: PlayerType,
        }

        #[derive(Debug, Deserialize)]
        #[serde(default)]
        struct Mapping {
            width: u32,
            height: u32,

            timeout: u64,
            wait: bool,

            respawn: u32,

            food: FoodConfig,

            snakes: Vec<HashMap<String, SnakeWrapper>>,
        }
        impl Default for Mapping {
            fn default() -> Self {
                Self {
                    width: 32,
                    height: 32,
                    timeout: 100,
                    wait: false,
                    respawn: 10,
                    food: Default::default(),
                    snakes: vec![],
                }
            }
        }

        let Mapping {
            width,
            height,
            timeout,
            wait,
            respawn,
            food,
            snakes,
        } = Mapping::deserialize(deserializer)?;

        Ok(Self {
            grid: GameGrid::new(width, height),
            death: DeathConfig {
                respawn_time: respawn,
            },
            turns: TurnConfig {
                turn_time: timeout,
                wait_for_all: wait,
            },

            food,

            players: snakes
                .iter()
                .map(|m| {
                    let name = m.keys().next().unwrap_or(&"Snake".to_string()).clone();
                    let wrapper = m.values().next().unwrap_or(&SnakeWrapper {
                        snake_type: PlayerType::Random,
                    });

                    PlayerConfig::new(name, wrapper.snake_type.clone())
                })
                .collect(),
        })
    }
}
