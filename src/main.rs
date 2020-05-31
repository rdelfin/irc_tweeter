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
use std::path::Path;
use std::time::Duration;
use structopt::StructOpt;
use tokio::time::delay_for;
use tweeter::Tweeter;

#[derive(StructOpt, Debug)]
#[structopt(name = "irc_tweeter")]
struct Opt {
    #[structopt(short = "c", long)]
    config_dir: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let config_dir = opt.config_dir.unwrap_or(String::from("config"));

    let settings = Settings::new(&Path::new(&config_dir))?;
    log4rs::init_file(
        &Path::new(&config_dir).join("log4rs.yml").as_path(),
        Default::default(),
    )?;
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
