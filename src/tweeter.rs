use crate::config::Settings;
use egg_mode::error::Result;
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token};

pub struct Tweeter {
    token: Token,
    write: bool,
}

impl Tweeter {
    pub fn new(settings: &Settings) -> Tweeter {
        let key = KeyPair::new(
            settings.twitter.api_key.clone(),
            settings.twitter.api_secret.clone(),
        );
        let access = KeyPair::new(
            settings.twitter.access_token.clone(),
            settings.twitter.access_secret.clone(),
        );
        Tweeter {
            token: Token::Access {
                consumer: key,
                access: access,
            },
            write: settings.write,
        }
    }

    pub async fn send_tweet(&self, content: String) -> Result<()> {
        if !self.write {
            info!("TWEETING IN NO WRITE MODE. Skipping tweet.");
            info!("Would have tweeted: {}", content.replace("\n", "\\n"));
        } else {
            DraftTweet::new(content).send(&self.token).await?;
        }
        Ok(())
    }
}
