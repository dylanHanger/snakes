use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource)]
#[serde(default)]
pub struct FoodConfig {
    #[serde(rename = "lifetime")]
    pub initial_lifetime: u32, // The number of turns the food will last for
    #[serde(rename = "value")]
    pub initial_value: i32, // Amount of growth gained for eating fresh food
}
impl Default for FoodConfig {
    fn default() -> Self {
        Self {
            initial_value: 5,
            initial_lifetime: 50,
        }
    }
}
