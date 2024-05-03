use serde::Deserialize;
use std::{error::Error, fs};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub amqp_address: String,
    pub queue_name: String,
}

impl Settings {
    pub fn from_file(path: &str) -> Result<Settings, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&contents)?;
        Ok(settings)
    }
}
