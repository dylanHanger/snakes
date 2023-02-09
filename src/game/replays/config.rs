use chrono::Utc;
use regex::{Captures, Regex};
use std::path::PathBuf;

use bevy::prelude::{Res, Resource};
use serde::Deserialize;

use crate::game::RngSeed;

fn default_replay_format() -> String {
    "{seed}-{time:%Y-%m-%dT%H-%M-%S}".to_string()
}

#[derive(Debug, Default, Deserialize, Resource)]
#[serde(default)]
pub struct ReplayConfig {
    pub record: bool,  // Whether to enable replay recording or not
    pub path: PathBuf, // Where to save the replay file
    #[serde(default = "default_replay_format")]
    pub format: String, // The format to save the filename
}

impl ReplayConfig {
    pub fn get_filename(&self, seed: &Res<RngSeed>) -> PathBuf {
        let time = Utc::now();

        let pattern = Regex::new(r"\{(.+?)\}").unwrap();
        let result = pattern.replace_all(&self.format, |captures: &Captures| {
            let capture = captures.get(1).unwrap().as_str();
            let parts: Vec<&str> = capture.split(':').collect();
            let name = parts[0];
            let format = if parts.len() > 1 {
                Some(parts[1])
            } else {
                None
            };

            match name {
                "seed" => seed
                    .0
                    .to_string()
                    .replace(|c: char| !c.is_alphanumeric(), ""),
                "time" => {
                    if let Some(fmt) = format {
                        time.format(fmt).to_string()
                    } else {
                        time.to_rfc3339()
                    }
                }
                _ => format!("{{{name}}}"),
            }
        });

        let mut filename = PathBuf::new();
        filename.set_file_name(result.to_string());
        filename.set_extension("rpl");

        filename
    }

    pub fn get_full_path(&self, seed: &Res<RngSeed>) -> PathBuf {
        self.path.join(self.get_filename(seed))
    }
}
