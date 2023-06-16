use chrono::Utc;
use regex::{Captures, Regex};
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::path::PathBuf;

use bevy::prelude::{Deref, Res, Resource};

use crate::game::RngSeed;

#[derive(Debug, Default, Resource, Deref)]
pub struct OptionalReplayConfig(pub Option<ReplayConfig>);

impl<'de> Deserialize<'de> for OptionalReplayConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Bool(true) => Ok(Self(Some(ReplayConfig::default()))),
            Value::Bool(false) => Ok(Self(None)),
            Value::Mapping(map) => {
                let replay_config = ReplayConfig::deserialize(Value::Mapping(map));

                if let Ok(replay_config) = replay_config {
                    Ok(Self(Some(replay_config)))
                } else {
                    Err(serde::de::Error::custom(
                        "Expected a boolean or a mapping for replay config",
                    ))
                }
            }
            _ => Err(serde::de::Error::custom(
                "Expected a boolean or a mapping for replay config",
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReplayConfig {
    #[serde(default = "ReplayConfig::default_path")]
    pub path: PathBuf, // Where to save the replay file
    #[serde(default = "ReplayConfig::default_format")]
    pub format: String, // The format to save the filename
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            path: Self::default_path(),
            format: ReplayConfig::default_format(),
        }
    }
}

impl ReplayConfig {
    fn default_format() -> String {
        "{seed}-{time:%Y-%m-%dT%H-%M-%S}".into()
    }

    fn default_path() -> PathBuf {
        "replays".into()
    }

    pub fn get_filename(&self, seed: &Res<RngSeed>) -> PathBuf {
        let time = Utc::now();

        let pattern = Regex::new(r"\{(.+?)\}").unwrap();
        let format = self.format.as_ref();
        let result = pattern.replace_all(format, |captures: &Captures| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_replay_deserialization_true() {
        let config_str = r#"
            true
        "#;

        let option: OptionalReplayConfig = serde_yaml::from_str(config_str).unwrap();
        assert!(option.is_some());

        let replay_config = option.0.unwrap();
        assert_eq!(replay_config.path, ReplayConfig::default_path());
        assert_eq!(replay_config.format, ReplayConfig::default_format());
    }

    #[test]
    fn test_replay_deserialization_false() {
        let config_str = r#"
            false
        "#;

        let option: OptionalReplayConfig = serde_yaml::from_str(config_str).unwrap();
        assert!(option.is_none(), "Expected None, got {:?}", option);
    }

    #[test]
    fn test_replay_deserialization_with_path() {
        let config_str = r#"
              path: /path/to/replays
              format: my-replay-%Y-%m-%d
        "#;

        let option: OptionalReplayConfig = serde_yaml::from_str(config_str).unwrap();
        assert!(option.is_some());

        let replay_config = option.0.unwrap();
        assert_eq!(replay_config.path, PathBuf::from("/path/to/replays"));
        assert_eq!(replay_config.format, "my-replay-%Y-%m-%d");
    }

    #[test]
    fn test_replay_deserialization_with_path_and_default_format() {
        let config_str = r#"
              path: /path/to/replays
        "#;

        let option: OptionalReplayConfig = serde_yaml::from_str(config_str).unwrap();
        assert!(option.is_some());

        let replay_config = option.0.unwrap();
        assert_eq!(replay_config.path, PathBuf::from("/path/to/replays"));
        assert_eq!(replay_config.format, ReplayConfig::default_format());
    }

    #[test]
    fn test_replay_deserialization_with_format_only() {
        let config_str = r#"
              format: custom-format-%H-%M-%S
        "#;

        let option: OptionalReplayConfig = serde_yaml::from_str(config_str).unwrap();
        assert!(option.is_some());

        let replay_config = option.0.unwrap();
        assert_eq!(replay_config.path, ReplayConfig::default_path());
        assert_eq!(replay_config.format, "custom-format-%H-%M-%S");
    }

    #[test]
    fn test_replay_deserialization_invalid_value() {
        let config_str = r#"
            invalid
        "#;

        let result: Result<OptionalReplayConfig, _> = serde_yaml::from_str(config_str);
        assert!(result.is_err());
    }
}
