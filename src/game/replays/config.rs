use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Resource)]
#[serde(default)]
pub struct ReplayConfig {
    pub record: bool, // Whether to enable replay recording or not
                      // TODO: Set filename here, with some pattern matching
}
