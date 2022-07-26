use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct TurnConfig {
    pub turn_time: u64,
    pub wait_for_all: bool,
}
impl Default for TurnConfig {
    fn default() -> Self {
        Self {
            turn_time: 100,
            wait_for_all: false,
        }
    }
}
