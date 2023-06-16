use bevy::prelude::Resource;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;

#[derive(Debug, Deserialize, Copy, Clone, Resource)]
#[serde(default)]
pub struct TurnConfig {
    #[serde(
        rename = "timeout",
        deserialize_with = "TurnConfig::deserialize_timeout"
    )]
    pub turn_time: Option<u64>,
    #[serde(rename = "quick")]
    pub end_early: bool,
    #[serde(rename = "turns")]
    pub max_turns: u32,
    #[serde(rename = "start_paused")]
    pub start_paused: bool,
}
impl Default for TurnConfig {
    fn default() -> Self {
        Self {
            turn_time: Some(100),
            end_early: false,
            max_turns: 1500,
            start_paused: false,
        }
    }
}

impl TurnConfig {
    fn deserialize_timeout<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Deserialize::deserialize(deserializer)?;
        if let Value::Bool(false) = value {
            Ok(None)
        } else {
            let time = value.as_u64();
            match time {
                Some(t) => Ok(Some(t)),
                None => Err(serde::de::Error::custom(
                    "Expected a positive integer or false for timeout",
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_timeout_with_value() {
        let yaml = r#"
            timeout: 100
        "#;
        let config: TurnConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.turn_time, Some(100));
    }

    #[test]
    fn test_deserialize_timeout_with_false() {
        let yaml = r#"
            timeout: false
        "#;
        let config: TurnConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.turn_time, None);
    }

    #[test]
    fn test_deserialize_timeout_with_true() {
        let yaml = r#"
            timeout: true
        "#;
        let result: Result<TurnConfig, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_timeout_with_invalid_value() {
        let yaml = r#"
            timeout: 23.1
        "#;
        let result: Result<TurnConfig, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }
}
