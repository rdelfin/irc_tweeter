use config::{Config, ConfigError, File};
use std::env;

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
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"))?;

        let env = env::var("MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        s.merge(File::with_name(&format!("config/local")).required(false))?;

        s.try_into()
    }
}
