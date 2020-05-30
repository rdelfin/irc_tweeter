use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
struct Twitter {
    api_key: String,
    api_secret: String,
    access_token: String,
    access_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    twitter: Twitter,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"))?;

        let env = env::var("MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false));
        s.merge(File::with_name(&format!("config/local")).required(false));

        s.try_into()
    }
}
