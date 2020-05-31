use config::{self, Config, File};
use std::env;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("error loading the config")]
    ConfigError(#[from] config::ConfigError),
    #[error("invalid config path")]
    InvalidConfigPath,
}

#[derive(Debug, Deserialize)]
pub struct Twitter {
    pub api_key: String,
    pub api_secret: String,
    pub access_token: String,
    pub access_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub twitter: Twitter,
    pub db: Db,
    pub interval_min: u64,
    pub write: bool,
}

impl Settings {
    pub fn new(config_folder: &Path) -> Result<Self, SettingsError> {
        let env = env::var("MODE").unwrap_or("development".into());

        let mut s = Config::new();
        s.merge(File::with_name(&path_join_to_string(
            config_folder,
            "default",
        )?))?;
        s.merge(File::with_name(&path_join_to_string(config_folder, &env)?).required(false))?;
        s.merge(File::with_name(&path_join_to_string(config_folder, "local")?).required(false))?;

        Ok(s.try_into()?)
    }
}

fn path_join_to_string(path: &Path, joined: &str) -> Result<String, SettingsError> {
    path.join(joined)
        .as_path()
        .to_str()
        .map(|s| String::from(s))
        .ok_or(SettingsError::InvalidConfigPath)
}
