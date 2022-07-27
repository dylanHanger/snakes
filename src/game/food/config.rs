use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct FoodConfig {
    #[serde(rename = "lifetime")]
    pub last_for_turns: u32, // The number of turns the food will last for
    #[serde(rename = "value")]
    pub growth_amount: i32, // Amount of growth gained for eating fresh food
}
impl Default for FoodConfig {
    fn default() -> Self {
        Self {
            growth_amount: 5,
            last_for_turns: 50,
        }
    }
}
