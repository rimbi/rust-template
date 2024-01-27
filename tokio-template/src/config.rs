use std::path::Path;

use anyhow::Context;
use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub trace_level: String,
}

impl Config {
    /// Parses a TOML config file and returns the config.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the configuration file.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Config>`. On success, returns the parsed `Config`. On failure, returns an error.
    pub async fn from_file(config_path: &str) -> anyhow::Result<Self> {
        let path = Path::new(config_path);
        let mut file = File::open(path)
            .await
            .context("Failed to open config file")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .context("Failed to read config file")?;
        toml::from_str(&contents).context("Failed to parse config file")
    }
}
