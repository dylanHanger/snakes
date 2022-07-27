use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct TurnConfig {
    #[serde(rename = "timeout")]
    pub turn_time: u64,
    #[serde(rename = "wait")]
    pub wait_for_all: bool,
    #[serde(rename = "turns")]
    pub max_turns: u64,
}
impl Default for TurnConfig {
    fn default() -> Self {
        Self {
            turn_time: 100,
            wait_for_all: false,
            max_turns: 1500,
        }
    }
}
