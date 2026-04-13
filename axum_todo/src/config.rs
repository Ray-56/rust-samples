//! Configuration file

use serde::Deserialize;

/// Web configuration
#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub addr: String,
}

/// Application configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub web: WebConfig,
}

impl Config {
    /// Initialize configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
