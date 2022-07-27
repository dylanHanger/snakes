use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DeathConfig {
    #[serde(rename = "respawn")]
    pub respawn_time: u32,
}
impl Default for DeathConfig {
    fn default() -> Self {
        Self { respawn_time: 10 }
    }
}
