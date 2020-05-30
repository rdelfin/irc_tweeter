use crate::config::Settings;
use egg_mode::error::Result;
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token};

pub struct Tweeter {
    token: Token,
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
        }
    }

    pub async fn send_tweet(&self, content: String) -> Result<()> {
        DraftTweet::new(content).send(&self.token).await?;
        Ok(())
    }
}
