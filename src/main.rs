#[macro_use]
extern crate serde_derive;

mod config;
mod ircdb;
mod tweeter;

use crate::config::Settings;
use ircdb::IrcDb;
use std::time::Duration;
use tokio::time::delay_for;
use tweeter::Tweeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new()?;
    let tweeter = Tweeter::new(&settings);
    let db = IrcDb::new(&settings.db.file)?;
    let wait_time = Duration::from_secs(settings.interval_min * 60);

    loop {
        let quote = db.get_random(1, 270)?;
        let message = format!("#{}:\n{}", quote.id, quote.quote);
        tweeter.send_tweet(message).await?;

        delay_for(wait_time).await;
    }
}
