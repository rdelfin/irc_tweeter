#[macro_use]
extern crate serde_derive;

mod config;
mod ircdb;
mod tweeter;

use crate::config::Settings;
use anyhow::Result;
use ircdb::IrcDb;
use smol::{self, Timer};
use std::time::Duration;
use tweeter::Tweeter;

fn main() -> Result<()> {
    let settings = Settings::new()?;
    let tweeter = Tweeter::new(&settings);
    let db = IrcDb::new(&settings.db.file)?;

    smol::run(async { main_loop(&settings, &tweeter, &db).await })?;
    Ok(())
}

async fn main_loop(settings: &Settings, tweeter: &Tweeter, db: &IrcDb) -> Result<()> {
    loop {
        tweeter.send_tweet(db.get_random(1)?.quote.clone()).await?;
        Timer::after(Duration::from_secs(settings.interval_min * 60)).await;
    }
}
