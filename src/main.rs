#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod config;
mod ircdb;
mod tweeter;

use crate::config::Settings;
use ircdb::IrcDb;
use log4rs;
use std::time::Duration;
use tokio::time::delay_for;
use tweeter::Tweeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new()?;
    log4rs::init_file("config/log4rs.yml", Default::default())?;
    let tweeter = Tweeter::new(&settings);
    let db = IrcDb::new(&settings.db.file)?;
    let wait_time = Duration::from_secs(settings.interval_min * 60);

    info!("Config finished...");

    loop {
        let quote = db.get_random(1, 270)?;
        let message = format!("#{}:\n{}", quote.id, quote.quote);
        info!(
            "Tweeting quote #{}: {}",
            quote.id,
            quote.quote.replace("\n", "\\n")
        );
        tweeter.send_tweet(message).await?;
        info!("Sent tweet");

        info!(
            "Waiting for {}m {}s before sending next tweet",
            wait_time.as_secs() / 60,
            wait_time.as_secs() % 60
        );
        delay_for(wait_time).await;
    }
}
